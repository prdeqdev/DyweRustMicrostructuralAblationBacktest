//! dywe-ablation-sim v0.3.1 — SCENARIO-BASED CAUSAL MICROSTRUCTURE BACKTEST.
//!
//! New in v0.3.1:
//!   * `--pending-policy <hold|drop>` (default: `hold`): controls whether taker orders arriving
//!     during an MM cancel-replace gap wait in `pending` until the fresh quote lands (`hold`) or
//!     immediately expire like real IOC orders (`drop`).
//!   * `--mid-mode <bidask|trade>` (default: `bidask`): when `bookTicker` is absent, reconstructs
//!     L1 mid either from separate last aggressive buy (`ask`) and last aggressive sell (`bid`)
//!     executions (`(bid + ask) / 2`), or from single-trade `price +/- half_tick` (`trade`).

#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

// ============================================================================
// Constants & Fixed-Point Scaling
// ============================================================================

const PRICE_SCALE: i64 = 100_000_000; // 1e8
const QTY_SCALE: i64 = 100_000_000;   // 1e8
/// 1 "sub" unit = 1e-6 relative price; 100 sub = 1.00 bps.
const SUB: i128 = 1_000_000;
/// Extra precision multiplier in Phase 2: value_q = sub_bps * Q.
const Q: i128 = 1_000;
const HORIZONS_MS: [i64; 3] = [0, 100, 1000];
const NS_PER_MS: i64 = 1_000_000;
/// Upper feasibility cap for equilibrium half-spread: 10.00 bps = 1000 sub.
const S_CAP_Q: i128 = 1_000 * Q;

const CAPTURES_PPM: [i128; 6] = [0, 100_000, 250_000, 500_000, 750_000, 1_000_000];
const IMPACTS_SUB: [i128; 5] = [0, 50, 100, 200, 300];
const SHARES_PPM: [i128; 4] = [0, 250_000, 500_000, 1_000_000];
const LEAKS_PPM: [i128; 4] = [0, 100_000, 250_000, 500_000];

macro_rules! emit {
    ($rep:expr, $($arg:tt)*) => {{
        let s = format!($($arg)*);
        println!("{}", s);
        $rep.push_str(&s);
        $rep.push('\n');
    }};
}

// ============================================================================
// Deterministic PRNG (SplitMix64)
// ============================================================================

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    #[inline(always)]
    fn range_i64(&mut self, min: i64, max: i64) -> i64 {
        if min >= max {
            return min;
        }
        let span = (max - min + 1) as u64;
        min + (self.next_u64() % span) as i64
    }
}

// ============================================================================
// Core Data Structures
// ============================================================================

#[derive(Clone, Copy, Debug)]
struct Mid {
    ts_ms: i64,
    mid: i64, // 1e8
}

#[derive(Clone, Copy, Debug)]
struct Taker {
    ts_ms: i64,
    dir: i8, // +1 taker buys (lifts Ask), -1 taker sells (hits Bid)
    notional_usd: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Regime {
    Fifo,
    CancelPrio,
    UniformFba,
    IbteCancelPrio,
}

impl Regime {
    fn is_batch(self) -> bool {
        !matches!(self, Regime::Fifo)
    }
    fn code(self) -> &'static str {
        match self {
            Regime::Fifo => "A",
            Regime::CancelPrio => "B",
            Regime::UniformFba => "C",
            Regime::IbteCancelPrio => "D",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingPolicy {
    Hold,
    Drop,
}

impl PendingPolicy {
    fn as_str(self) -> &'static str {
        match self {
            PendingPolicy::Hold => "hold",
            PendingPolicy::Drop => "drop",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MidMode {
    BidAskProxy,
    TradeHalfTick,
}

impl MidMode {
    fn as_str(self) -> &'static str {
        match self {
            MidMode::BidAskProxy => "bidask",
            MidMode::TradeHalfTick => "trade",
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Cfg {
    feed_ms: i64,
    sniper_feed_ms: i64,
    mm_react_ms: i64,
    sniper_react_ms: i64,
    net_ms: i64,
    window_ms: i64,
    decrypt_ms: i64,
    requote_thr_sub: i64,
    sniper_thr_sub: i64,
    sniper_clip_usd: i64,
    large_usd: i64,
    pending_policy: PendingPolicy,
    seed: u64,
    log: bool,
}

fn default_cfg() -> Cfg {
    Cfg {
        feed_ms: 5,
        sniper_feed_ms: 5,
        mm_react_ms: 10,
        sniper_react_ms: 2,
        net_ms: 30,
        window_ms: 50,
        decrypt_ms: 0,
        requote_thr_sub: 150,
        sniper_thr_sub: 200,
        sniper_clip_usd: 25_000,
        large_usd: 50_000,
        pending_policy: PendingPolicy::Hold,
        seed: 0xD15E_0001,
        log: false,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Cancel {
        target: u64,
    },
    New {
        id: u64,
        reff: i64,
    },
    Taker {
        dir: i8,
        notional: i64,
        send_ms: i64,
    },
    Sniper {
        dir: i8,
        notional: i64,
        send_ms: i64,
        target_id: u64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Ev {
    arr_ns: i64,
    decision_ms: i64,
    kind: Kind,
}

#[derive(Clone, Copy, Debug)]
struct Quote {
    live: bool,
    id: u64,
    reff: i64,
}

#[derive(Clone, Copy, Debug)]
struct TakerOrd {
    dir: i8,
    notional: i64,
    send_ms: i64,
}

#[derive(Clone, Debug, PartialEq)]
struct FillLog {
    clear_ms: i64,
    kind: u8,
    ref_mid: i64,
    sign: i8,
    notional: i64,
}

// ============================================================================
// Additive Accumulator
// ============================================================================

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
struct Agg {
    mm_n: i128,
    mm_w: [i128; 3],
    mm_taker_n: i128,
    mm_taker_w1s: i128,
    mm_sniper_stale_n: i128,
    mm_sniper_stale_w1s: i128,
    mm_sniper_late_n: i128,
    mm_sniper_late_w1s: i128,

    mm_large_n: i128,
    mm_large_exp_n: i128,
    mm_fills: i64,
    sniper_hit: i64,
    sniper_late_fresh: i64,
    sniper_missed: i64,

    tk_n: i128,
    tk_scoef_w: i128,
    tk_stale_w: i128,
    tk_wait_all_w: i128,
    tk_large_n: i128,
    tk_large_exp_n: i128,
    tk_inf_n: i128,
    tk_inf_wait_w: i128,
    tk_fills: i64,
    tk_dropped: i64,
    tk_dropped_usd: i128,
    batches: i64,
    pending_dropped: i64,
}

impl Agg {
    fn merge(&mut self, o: &Agg) {
        self.mm_n += o.mm_n;
        for i in 0..3 {
            self.mm_w[i] += o.mm_w[i];
        }
        self.mm_taker_n += o.mm_taker_n;
        self.mm_taker_w1s += o.mm_taker_w1s;
        self.mm_sniper_stale_n += o.mm_sniper_stale_n;
        self.mm_sniper_stale_w1s += o.mm_sniper_stale_w1s;
        self.mm_sniper_late_n += o.mm_sniper_late_n;
        self.mm_sniper_late_w1s += o.mm_sniper_late_w1s;

        self.mm_large_n += o.mm_large_n;
        self.mm_large_exp_n += o.mm_large_exp_n;
        self.mm_fills += o.mm_fills;
        self.sniper_hit += o.sniper_hit;
        self.sniper_late_fresh += o.sniper_late_fresh;
        self.sniper_missed += o.sniper_missed;
        self.tk_n += o.tk_n;
        self.tk_scoef_w += o.tk_scoef_w;
        self.tk_stale_w += o.tk_stale_w;
        self.tk_wait_all_w += o.tk_wait_all_w;
        self.tk_large_n += o.tk_large_n;
        self.tk_large_exp_n += o.tk_large_exp_n;
        self.tk_inf_n += o.tk_inf_n;
        self.tk_inf_wait_w += o.tk_inf_wait_w;
        self.tk_fills += o.tk_fills;
        self.tk_dropped += o.tk_dropped;
        self.tk_dropped_usd += o.tk_dropped_usd;
        self.batches += o.batches;
        self.pending_dropped += o.pending_dropped;
    }
}

fn total(blocks: &[Agg]) -> Agg {
    let mut t = Agg::default();
    for b in blocks {
        t.merge(b);
    }
    t
}

// ============================================================================
// Price Lookup Helpers
// ============================================================================

#[inline(always)]
fn mid_at(mids: &[Mid], t_ms: i64) -> i64 {
    let idx = mids.partition_point(|m| m.ts_ms <= t_ms);
    if idx == 0 {
        mids[0].mid
    } else {
        mids[idx - 1].mid
    }
}

#[inline(always)]
fn rel_sub(a: i64, base: i64) -> i64 {
    (((a - base) as i128) * SUB / (base.max(1) as i128)) as i64
}

fn shift_takers(takers: &[Taker], t0: i64, t1: i64, shift_ms: i64) -> Vec<Taker> {
    let span = (t1 - t0).max(1);
    let mut v: Vec<Taker> = takers
        .iter()
        .map(|t| {
            let rel = (t.ts_ms - t0).rem_euclid(span);
            let ts = t0 + (rel + shift_ms).rem_euclid(span);
            Taker { ts_ms: ts, ..*t }
        })
        .collect();
    v.sort_by_key(|t| t.ts_ms);
    v
}

// ============================================================================
// Phase 1: Causal Event Generation
// ============================================================================

fn gen_mm_events(mids: &[Mid], cfg: &Cfg) -> Vec<Ev> {
    let mut out: Vec<Ev> = Vec::new();
    if mids.is_empty() {
        return out;
    }
    let mut rng = SplitMix64::new(cfg.seed ^ 0x4D4D);
    let mut intended_ref = mids[0].mid;
    let mut quote_id: u64 = 0;
    let mut busy_until_ms: i64 = i64::MIN;
    for m in mids {
        let seen_ms = m.ts_ms + cfg.feed_ms;
        if seen_ms < busy_until_ms {
            continue;
        }
        if rel_sub(m.mid, intended_ref).abs() >= cfg.requote_thr_sub {
            let decision_ms = seen_ms + cfg.mm_react_ms;
            let new_ref = mid_at(mids, decision_ms - cfg.feed_ms);
            let jitter = rng.range_i64(0, 500_000);
            let arr = (decision_ms + cfg.net_ms) * NS_PER_MS + jitter;
            out.push(Ev {
                arr_ns: arr,
                decision_ms,
                kind: Kind::Cancel { target: quote_id },
            });
            quote_id += 1;
            out.push(Ev {
                arr_ns: arr + 50_000,
                decision_ms,
                kind: Kind::New {
                    id: quote_id,
                    reff: new_ref,
                },
            });
            intended_ref = new_ref;
            busy_until_ms = decision_ms;
        }
    }
    out.sort_by_key(|e| e.arr_ns);
    out
}

fn gen_taker_events(takers: &[Taker], cfg: &Cfg) -> Vec<Ev> {
    let mut rng = SplitMix64::new(cfg.seed ^ 0x544B);
    let mut v: Vec<Ev> = Vec::with_capacity(takers.len());
    for t in takers {
        let jitter = rng.range_i64(0, 900_000);
        v.push(Ev {
            arr_ns: (t.ts_ms + cfg.net_ms) * NS_PER_MS + jitter,
            decision_ms: t.ts_ms,
            kind: Kind::Taker {
                dir: t.dir,
                notional: t.notional_usd.max(1),
                send_ms: t.ts_ms,
            },
        });
    }
    v.sort_by_key(|e| e.arr_ns);
    v
}

// ============================================================================
// Phase 1: Matching & Execution Engine
// ============================================================================

struct Sim<'a> {
    mids: &'a [Mid],
    regime: Regime,
    cfg: Cfg,
    week_start_ms: i64,
    block_ms: i64,
    agg: Vec<Agg>,
    proc: Quote,
    visible: Quote,
    pending: Vec<TakerOrd>,
    tomb: HashSet<u64>,
    sniper_inflight: bool,
    last_sniped_quote_id: Option<u64>,
    log: Vec<FillLog>,
}

struct SimOut {
    agg: Vec<Agg>,
    log: Vec<FillLog>,
}

impl<'a> Sim<'a> {
    fn block_of(&self, ms: i64) -> usize {
        let n = self.agg.len() as i64;
        let b = ((ms - self.week_start_ms).max(0)) / self.block_ms;
        b.min(n - 1) as usize
    }

    fn bump_missed(&mut self, ms: i64) {
        let b = self.block_of(ms);
        self.agg[b].sniper_missed += 1;
    }

    fn bump_hit(&mut self, ms: i64) {
        let b = self.block_of(ms);
        self.agg[b].sniper_hit += 1;
    }

    fn bump_batch(&mut self, ms: i64) {
        let b = self.block_of(ms);
        self.agg[b].batches += 1;
    }

    fn bump_dropped_taker(&mut self, ms: i64, notional: i64) {
        let b = self.block_of(ms);
        self.agg[b].tk_dropped += 1;
        self.agg[b].tk_dropped_usd += notional.max(1) as i128;
    }

    fn rec_mm(
        &mut self,
        notional: i64,
        mm_sign: i8,
        reff: i64,
        clear_ms: i64,
        sniper_mode: u8,
        large_n: i128,
        exposed: bool,
    ) {
        let n = notional.max(1) as i128;
        let mut d = [0i64; 3];
        for (i, h) in HORIZONS_MS.iter().enumerate() {
            let mf = mid_at(self.mids, clear_ms + *h);
            d[i] = (mm_sign as i64) * rel_sub(mf, reff);
        }
        let w1s = n * (d[2] as i128);
        let b = self.block_of(clear_ms);
        {
            let a = &mut self.agg[b];
            a.mm_n += n;
            for i in 0..3 {
                a.mm_w[i] += n * (d[i] as i128);
            }
            a.mm_large_n += large_n;
            if exposed {
                a.mm_large_exp_n += large_n;
            }
            a.mm_fills += 1;

            match sniper_mode {
                1 => {
                    a.mm_sniper_stale_n += n;
                    a.mm_sniper_stale_w1s += w1s;
                    a.sniper_hit += 1;
                }
                2 => {
                    a.mm_sniper_late_n += n;
                    a.mm_sniper_late_w1s += w1s;
                    a.sniper_late_fresh += 1;
                }
                _ => {
                    a.mm_taker_n += n;
                    a.mm_taker_w1s += w1s;
                }
            }
        }
        if self.cfg.log {
            self.log.push(FillLog {
                clear_ms,
                kind: if sniper_mode == 1 {
                    1
                } else if sniper_mode == 2 {
                    3
                } else {
                    0
                },
                ref_mid: reff,
                sign: mm_sign,
                notional,
            });
        }
    }

    fn rec_taker(&mut self, t: &TakerOrd, reff: i64, clear_ms: i64, s_coeff: i32, exposed: bool) {
        let n = t.notional.max(1) as i128;
        let mid_clear = mid_at(self.mids, clear_ms);
        let mid_send = mid_at(self.mids, t.send_ms);
        let stale = (t.dir as i64) * rel_sub(reff, mid_clear);
        let wait = (t.dir as i64) * rel_sub(mid_clear, mid_send);
        let large = t.notional >= self.cfg.large_usd;
        let b = self.block_of(clear_ms);
        let a = &mut self.agg[b];
        a.tk_n += n;
        a.tk_scoef_w += n * (s_coeff as i128);
        a.tk_stale_w += n * (stale as i128);
        a.tk_wait_all_w += n * (wait as i128);
        a.tk_fills += 1;
        if large {
            a.tk_large_n += n;
            if exposed {
                a.tk_large_exp_n += n;
            }
            a.tk_inf_n += n;
            a.tk_inf_wait_w += n * (wait as i128);
        }
    }

    fn exec_taker(&mut self, t: &TakerOrd, clear_ms: i64, exposed: bool) {
        let reff = self.proc.reff;
        let large_n = if t.notional >= self.cfg.large_usd {
            t.notional.max(1) as i128
        } else {
            0
        };
        self.rec_mm(t.notional, -t.dir, reff, clear_ms, 0, large_n, exposed);
        self.rec_taker(t, reff, clear_ms, 1, exposed);
    }

    fn exec_sniper(&mut self, dir: i8, notional: i64, target_id: u64, clear_ms: i64, exposed: bool) {
        let reff = self.proc.reff;
        let is_true_stale_hit = self.proc.id == target_id;
        let mode = if is_true_stale_hit { 1 } else { 2 };
        self.rec_mm(notional, -dir, reff, clear_ms, mode, 0, exposed);
    }

    fn flush_pending(&mut self, clear_ms: i64, exposed: bool) {
        if self.pending.is_empty() {
            return;
        }
        let p = std::mem::take(&mut self.pending);
        for t in p.iter() {
            self.exec_taker(t, clear_ms, exposed);
        }
    }

    fn apply_cancel(&mut self, target: u64) {
        if self.proc.id == target {
            self.proc.live = false;
        } else {
            self.tomb.insert(target);
        }
    }

    fn apply_new(&mut self, id: u64, reff: i64) -> bool {
        if self.tomb.remove(&id) {
            return false;
        }
        self.proc = Quote {
            live: true,
            id,
            reff,
        };
        true
    }

    fn handle_event(&mut self, ev: &Ev, clear_ms: i64, exposed: bool) {
        match ev.kind {
            Kind::Cancel { target } => self.apply_cancel(target),
            Kind::New { id, reff } => {
                if self.apply_new(id, reff) {
                    self.flush_pending(clear_ms, exposed);
                }
            }
            Kind::Taker {
                dir,
                notional,
                send_ms,
            } => {
                let t = TakerOrd {
                    dir,
                    notional,
                    send_ms,
                };
                if self.proc.live {
                    self.exec_taker(&t, clear_ms, exposed);
                } else {
                    match self.cfg.pending_policy {
                        PendingPolicy::Hold => self.pending.push(t),
                        PendingPolicy::Drop => self.bump_dropped_taker(clear_ms, notional),
                    }
                }
            }
            Kind::Sniper {
                dir,
                notional,
                target_id,
                ..
            } => {
                self.sniper_inflight = false;
                if self.proc.live {
                    self.exec_sniper(dir, notional, target_id, clear_ms, exposed);
                } else {
                    self.bump_missed(clear_ms);
                }
            }
        }
    }

    fn process_batch(&mut self, buf: &mut Vec<Ev>, clear_ns: i64) {
        let clear_ms = clear_ns / NS_PER_MS;
        let exposed = buf.iter().any(|e| matches!(e.kind, Kind::Cancel { .. }));
        buf.sort_by_key(|e| {
            (
                if matches!(e.kind, Kind::Cancel { .. }) {
                    0u8
                } else {
                    1u8
                },
                e.arr_ns,
            )
        });
        for ev in buf.iter() {
            self.handle_event(ev, clear_ms, exposed);
        }
        self.bump_batch(clear_ms);
    }

    fn process_fba(&mut self, buf: &mut Vec<Ev>, clear_ns: i64) {
        let clear_ms = clear_ns / NS_PER_MS;
        for ev in buf.iter() {
            match ev.kind {
                Kind::Cancel { target } => self.apply_cancel(target),
                Kind::New { id, reff } => {
                    self.apply_new(id, reff);
                }
                _ => {}
            }
        }
        let mut orders: Vec<TakerOrd> = std::mem::take(&mut self.pending);
        let mut snipers: Vec<(i8, i64)> = Vec::new();
        for ev in buf.iter() {
            match ev.kind {
                Kind::Taker {
                    dir,
                    notional,
                    send_ms,
                } => orders.push(TakerOrd {
                    dir,
                    notional,
                    send_ms,
                }),
                Kind::Sniper { dir, notional, .. } => snipers.push((dir, notional)),
                _ => {}
            }
        }
        if !snipers.is_empty() {
            self.sniper_inflight = false;
        }
        self.bump_batch(clear_ms);
        if !self.proc.live {
            match self.cfg.pending_policy {
                PendingPolicy::Hold => {
                    self.pending = orders;
                }
                PendingPolicy::Drop => {
                    for o in orders {
                        self.bump_dropped_taker(clear_ms, o.notional);
                    }
                }
            }
            for _ in 0..snipers.len() {
                self.bump_missed(clear_ms);
            }
            return;
        }
        let large_usd = self.cfg.large_usd;
        let mut buys: i128 = 0;
        let mut sells: i128 = 0;
        let mut tk_total: i128 = 0;
        let mut tk_large: i128 = 0;
        for o in orders.iter() {
            let n = o.notional.max(1) as i128;
            tk_total += n;
            if o.notional >= large_usd {
                tk_large += n;
            }
            if o.dir > 0 {
                buys += n;
            } else {
                sells += n;
            }
        }
        for &(d, n) in snipers.iter() {
            let n = n.max(1) as i128;
            if d > 0 {
                buys += n;
            } else {
                sells += n;
            }
        }
        let net = buys - sells;
        let netsign: i32 = if net > 0 {
            1
        } else if net < 0 {
            -1
        } else {
            0
        };
        let reff = self.proc.reff;
        for o in orders.iter() {
            let sc = (o.dir as i32) * netsign;
            self.rec_taker(o, reff, clear_ms, sc, false);
        }
        if net != 0 {
            let mm_n = net.abs();
            let mm_large = if tk_total > 0 {
                mm_n * tk_large / tk_total
            } else {
                0
            };
            let mm_sign: i8 = if net > 0 { -1 } else { 1 };
            self.rec_mm(mm_n as i64, mm_sign, reff, clear_ms, 0, mm_large, false);
            if self.cfg.log {
                if let Some(l) = self.log.last_mut() {
                    l.kind = 2;
                }
            }
        }
        for _ in 0..snipers.len() {
            self.bump_hit(clear_ms);
        }
    }
}

fn simulate(
    mids: &[Mid],
    takers: &[Taker],
    regime: Regime,
    cfg: &Cfg,
    week_start_ms: i64,
    block_ms: i64,
    nblocks: usize,
) -> SimOut {
    let nblocks = nblocks.max(1);
    if mids.is_empty() {
        return SimOut {
            agg: vec![Agg::default(); nblocks],
            log: Vec::new(),
        };
    }
    let mut sim = Sim {
        mids,
        regime,
        cfg: *cfg,
        week_start_ms,
        block_ms: block_ms.max(1),
        agg: vec![Agg::default(); nblocks],
        proc: Quote {
            live: true,
            id: 0,
            reff: mids[0].mid,
        },
        visible: Quote {
            live: true,
            id: 0,
            reff: mids[0].mid,
        },
        pending: Vec::new(),
        tomb: HashSet::new(),
        sniper_inflight: false,
        last_sniped_quote_id: None,
        log: Vec::new(),
    };

    let mm_ev = gen_mm_events(mids, cfg);
    let tk_ev = gen_taker_events(takers, cfg);
    let mut arrivals: Vec<Ev> = Vec::with_capacity(mm_ev.len() + tk_ev.len());
    arrivals.extend(mm_ev);
    arrivals.extend(tk_ev);
    arrivals.sort_by_key(|e| e.arr_ns);

    const INF: i64 = i64::MAX;
    let batch = regime.is_batch();
    let window_ns = cfg.window_ms.max(1) * NS_PER_MS;
    let grid0 = mids[0].ts_ms * NS_PER_MS;
    let mut next_close = grid0 + window_ns;
    let mut ai = 0usize;
    let mut si = 0usize;
    let mut sq: VecDeque<Ev> = VecDeque::new();
    let mut buf: Vec<Ev> = Vec::new();
    let mut commits: VecDeque<(i64, Quote)> = VecDeque::new();
    let mut srng = SplitMix64::new(cfg.seed ^ 0x534E);

    loop {
        let t_arr_main = if ai < arrivals.len() {
            arrivals[ai].arr_ns
        } else {
            INF
        };
        let t_arr_sq = sq.front().map_or(INF, |e| e.arr_ns);
        let t_arr = t_arr_main.min(t_arr_sq);
        let t_commit = commits.front().map_or(INF, |c| c.0);
        let t_snipe = if si < mids.len() && cfg.sniper_clip_usd > 0 {
            (mids[si].ts_ms + cfg.sniper_feed_ms + cfg.sniper_react_ms) * NS_PER_MS
        } else {
            INF
        };
        if t_arr == INF && t_snipe == INF && buf.is_empty() {
            break;
        }
        if batch && buf.is_empty() {
            let t_other = t_arr.min(t_snipe).min(t_commit);
            if t_other != INF && t_other > next_close {
                let k = (t_other - grid0 + window_ns - 1) / window_ns;
                next_close = grid0 + k * window_ns;
            }
        }
        let t_close = if batch { next_close } else { INF };
        let t_min = t_commit.min(t_arr).min(t_close).min(t_snipe);
        if t_min == INF {
            break;
        }

        if t_commit == t_min {
            if let Some((_, q)) = commits.pop_front() {
                sim.visible = q;
            }
            continue;
        }
        if t_arr == t_min {
            let ev = if t_arr_main <= t_arr_sq {
                let e = arrivals[ai];
                ai += 1;
                e
            } else {
                match sq.pop_front() {
                    Some(e) => e,
                    None => continue,
                }
            };
            if batch {
                buf.push(ev);
            } else {
                sim.handle_event(&ev, ev.arr_ns / NS_PER_MS, false);
                sim.visible = sim.proc;
            }
            continue;
        }
        if t_close == t_min {
            if !buf.is_empty() {
                let clear_ns = t_close + cfg.decrypt_ms * NS_PER_MS;
                if regime == Regime::UniformFba {
                    sim.process_fba(&mut buf, clear_ns);
                } else {
                    sim.process_batch(&mut buf, clear_ns);
                }
                buf.clear();
                if cfg.decrypt_ms == 0 {
                    sim.visible = sim.proc;
                } else {
                    commits.push_back((clear_ns, sim.proc));
                }
            }
            next_close += window_ns;
            continue;
        }
        let m = mids[si];
        si += 1;
        if !sim.sniper_inflight
            && sim.visible.live
            && sim.last_sniped_quote_id != Some(sim.visible.id)
            && rel_sub(m.mid, sim.visible.reff).abs() >= cfg.sniper_thr_sub
        {
            let dir: i8 = if m.mid > sim.visible.reff { 1 } else { -1 };
            let decide_ms = t_snipe / NS_PER_MS;
            let arr = (decide_ms + cfg.net_ms) * NS_PER_MS + srng.range_i64(0, 200_000);
            sq.push_back(Ev {
                arr_ns: arr,
                decision_ms: decide_ms,
                kind: Kind::Sniper {
                    dir,
                    notional: cfg.sniper_clip_usd,
                    send_ms: decide_ms,
                    target_id: sim.visible.id,
                },
            });
            sim.sniper_inflight = true;
            sim.last_sniped_quote_id = Some(sim.visible.id);
        }
    }

    if !sim.pending.is_empty() {
        let dropped = sim.pending.len() as i64;
        if let Some(last) = sim.agg.last_mut() {
            last.pending_dropped += dropped;
        }
    }
    SimOut {
        agg: sim.agg,
        log: sim.log,
    }
}

// ============================================================================
// Phase 2: Deterministic i128 Convolution, Endogenous s*, All-In Taker Cost
// ============================================================================

#[derive(Clone, Copy, Debug)]
struct Econ {
    capture_ppm: i128,
    impact_sub: i128,
    mm_share_ppm: i128,
    leak_ppm: i128,
    rebate_sub: i128,
    opex_sub: i128,
    horizon: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SStar {
    Val(i128),
    Floor,
    Cap,
}

#[derive(Clone, Copy, Debug)]
struct Outcome {
    drift_q: i128,
    mev_mm_q: i128,
    mev_tk_q: i128,
    s_gross: SStar,
    s_net: SStar,
    spread_paid_q: Option<i128>,
    stale_q: i128,
    wait_q: i128,
    all_in_gross: Option<i128>,
    all_in_net: Option<i128>,
}

fn classify(v: i128) -> SStar {
    if v < 0 {
        SStar::Floor
    } else if v > S_CAP_Q {
        SStar::Cap
    } else {
        SStar::Val(v)
    }
}

fn evaluate(a: &Agg, regime: Regime, e: &Econ) -> Option<Outcome> {
    if a.mm_n <= 0 || a.tk_n <= 0 {
        return None;
    }
    let (p, large_mm, large_tk) = if regime == Regime::IbteCancelPrio {
        (e.leak_ppm, a.mm_large_exp_n, a.tk_large_exp_n)
    } else {
        (e.capture_ppm, a.mm_large_n, a.tk_large_n)
    };
    let drift_q = a.mm_w[e.horizon] * Q / a.mm_n;
    let den: i128 = 1_000_000_000_000;
    let mev_mm_q = p * e.impact_sub * e.mm_share_ppm * Q * large_mm / (den * a.mm_n);
    let mev_tk_q = p * e.impact_sub * (1_000_000 - e.mm_share_ppm) * Q * large_tk / (den * a.tk_n);
    let gross_v = -drift_q + mev_mm_q;
    let net_v = gross_v - e.rebate_sub * Q + e.opex_sub * Q;
    let s_gross = classify(gross_v);
    let s_net = classify(net_v);
    let stale_q = a.tk_stale_w * Q / a.tk_n;
    let wait_q = if a.tk_inf_n > 0 {
        a.tk_inf_wait_w * Q / a.tk_inf_n
    } else {
        0
    };
    let spread_paid = |sv: SStar| -> Option<i128> {
        let s = match sv {
            SStar::Val(v) => v,
            SStar::Floor => 0,
            SStar::Cap => return None,
        };
        Some(a.tk_scoef_w * s / a.tk_n)
    };
    let sp_gross = spread_paid(s_gross);
    let sp_net = spread_paid(s_net);
    Some(Outcome {
        drift_q,
        mev_mm_q,
        mev_tk_q,
        s_gross,
        s_net,
        spread_paid_q: sp_net,
        stale_q,
        wait_q,
        all_in_gross: sp_gross.map(|sp| sp + stale_q + mev_tk_q + wait_q),
        all_in_net: sp_net.map(|sp| sp + stale_q + mev_tk_q + wait_q),
    })
}

fn delta_all_in(b: &Agg, d: &Agg, e: &Econ) -> Option<i128> {
    let ob = evaluate(b, Regime::CancelPrio, e)?;
    let od = evaluate(d, Regime::IbteCancelPrio, e)?;
    Some(od.all_in_net? - ob.all_in_net?)
}

fn ci95(mut v: Vec<i128>) -> Option<(i128, i128)> {
    if v.len() < 20 {
        return None;
    }
    v.sort();
    let n = v.len();
    Some((v[n * 25 / 1000], v[(n * 975 / 1000).min(n - 1)]))
}

fn verdict(ci: Option<(i128, i128)>) -> &'static str {
    match ci {
        None => "n/a",
        Some((lo, hi)) => {
            if hi < 0 {
                "D<B"
            } else if lo > 0 {
                "B<D"
            } else {
                "~"
            }
        }
    }
}

fn bootstrap_sums(combined: &[Vec<Agg>], reps: usize, seed: u64) -> Vec<Vec<Agg>> {
    let nspec = combined.len();
    let mut out: Vec<Vec<Agg>> = vec![Vec::new(); nspec];
    if nspec == 0 || reps == 0 {
        return out;
    }
    let nb = combined[0].len();
    if nb == 0 {
        return out;
    }
    let mut rng = SplitMix64::new(seed);
    for _ in 0..reps {
        let idx: Vec<usize> = (0..nb)
            .map(|_| (rng.next_u64() % (nb as u64)) as usize)
            .collect();
        for s in 0..nspec {
            let mut acc = Agg::default();
            for &i in &idx {
                acc.merge(&combined[s][i]);
            }
            out[s].push(acc);
        }
    }
    out
}

// ============================================================================
// Integer Formatting Helpers
// ============================================================================

fn fmt_bps(v_q: i128) -> String {
    let milli = v_q / 100;
    let sign = if milli < 0 { "-" } else { "" };
    let a = milli.abs();
    format!("{}{}.{:03}", sign, a / 1000, a % 1000)
}

fn fmt_bps_signed(v_q: i128) -> String {
    let s = fmt_bps(v_q);
    if v_q >= 0 {
        format!("+{}", s)
    } else {
        s
    }
}

fn fmt_permille(v: i128) -> String {
    let sign = if v < 0 { "-" } else { "" };
    let a = v.abs();
    format!("{}{}.{}%", sign, a / 10, a % 10)
}

fn fmt_s(s: SStar) -> String {
    match s {
        SStar::Val(v) => fmt_bps(v),
        SStar::Floor => "<0".to_string(),
        SStar::Cap => ">10 (infeasible)".to_string(),
    }
}

fn fmt_opt(v: Option<i128>) -> String {
    match v {
        Some(x) => fmt_bps(x),
        None => "infeasible".to_string(),
    }
}

fn fmt_usd(val: i128) -> String {
    let s = val.abs().to_string();
    let len = s.len();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    if val < 0 {
        format!("-${}", out)
    } else {
        format!("${}", out)
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

// ============================================================================
// Market Data Parsers
// ============================================================================

#[inline(always)]
fn parse_fixed_1e8(s: &str) -> Option<i64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let bytes = s.as_bytes();
    let first = bytes[0];
    if !first.is_ascii_digit() && first != b'-' && first != b'+' {
        return None;
    }
    let mut neg = false;
    let mut idx = 0;
    if first == b'-' {
        neg = true;
        idx += 1;
    } else if first == b'+' {
        idx += 1;
    }
    let mut int_part: i64 = 0;
    let mut frac_part: i64 = 0;
    let mut frac_scale: i64 = PRICE_SCALE;
    let mut in_frac = false;
    while idx < bytes.len() {
        let b = bytes[idx];
        if b == b'.' {
            in_frac = true;
        } else if b.is_ascii_digit() {
            let d = (b - b'0') as i64;
            if !in_frac {
                int_part = int_part.wrapping_mul(10).wrapping_add(d);
            } else if frac_scale > 1 {
                frac_scale /= 10;
                frac_part = frac_part.wrapping_add(d * frac_scale);
            }
        } else {
            return None;
        }
        idx += 1;
    }
    let combined = int_part.checked_mul(PRICE_SCALE)?.checked_add(frac_part)?;
    Some(if neg { -combined } else { combined })
}

#[inline(always)]
fn parse_i64_fast(s: &str) -> Option<i64> {
    s.trim().parse::<i64>().ok()
}

#[inline(always)]
fn parse_bool_fast(s: &str) -> bool {
    let t = s.trim();
    t.eq_ignore_ascii_case("true") || t == "1" || t.eq_ignore_ascii_case("t")
}

fn civil_to_days(y: i64, m: i64, d: i64) -> i64 {
    let y = y - if m <= 2 { 1 } else { 0 };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn date_str(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn parse_timestamp_to_ms(s: &str) -> Option<i64> {
    let t = s.trim();
    if let Ok(raw) = t.parse::<i64>() {
        return Some(if raw > 100_000_000_000_000 {
            raw / 1_000
        } else {
            raw
        });
    }
    if t.len() >= 19 && t.as_bytes()[4] == b'-' && t.as_bytes()[7] == b'-' {
        let year: i64 = t[0..4].parse().ok()?;
        let month: i64 = t[5..7].parse().ok()?;
        let day: i64 = t[8..10].parse().ok()?;
        let hour: i64 = t[11..13].parse().ok()?;
        let min: i64 = t[14..16].parse().ok()?;
        let sec: i64 = t[17..19].parse().ok()?;
        let days = civil_to_days(year, month, day);
        return Some((days * 86_400 + hour * 3_600 + min * 60 + sec) * 1_000);
    }
    None
}

fn half_tick_1e8(sym: &str) -> i64 {
    match sym {
        "BTCUSDT" => 5_000_000,
        "ETHUSDT" => 500_000,
        _ => 0,
    }
}

fn push_mid(v: &mut Vec<Mid>, ts: i64, mid: i64) {
    if let Some(l) = v.last_mut() {
        if l.ts_ms == ts {
            l.mid = mid;
            return;
        }
        if l.mid == mid {
            return;
        }
    }
    v.push(Mid { ts_ms: ts, mid });
}

fn notional_usd(price: i64, qty: i64) -> i64 {
    ((price as i128 * qty as i128) / (PRICE_SCALE as i128 * QTY_SCALE as i128)) as i64
}

fn load_book_ticker(path: &Path) -> Result<Vec<Mid>, String> {
    let file = File::open(path).map_err(|e| format!("open {}: {}", path.display(), e))?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut out: Vec<Mid> = Vec::with_capacity(4_000_000);
    let (mut c_bid, mut c_ask, mut c_ts) = (1usize, 3usize, 5usize);
    let mut first = true;
    for line_res in reader.lines() {
        let line = line_res.map_err(|e| e.to_string())?;
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if first {
            first = false;
            if t.as_bytes()[0].is_ascii_alphabetic() {
                for (i, name) in t.split(',').enumerate() {
                    match name.trim() {
                        "best_bid_price" => c_bid = i,
                        "best_ask_price" => c_ask = i,
                        "transaction_time" => c_ts = i,
                        _ => {}
                    }
                }
                continue;
            }
        }
        let mut bid: Option<i64> = None;
        let mut ask: Option<i64> = None;
        let mut ts: Option<i64> = None;
        let max_c = c_bid.max(c_ask).max(c_ts);
        for (i, f) in t.split(',').enumerate() {
            if i == c_bid {
                bid = parse_fixed_1e8(f);
            } else if i == c_ask {
                ask = parse_fixed_1e8(f);
            } else if i == c_ts {
                ts = parse_timestamp_to_ms(f);
            }
            if i >= max_c {
                break;
            }
        }
        if let (Some(b), Some(a), Some(ts)) = (bid, ask, ts) {
            if b > 0 && a >= b {
                push_mid(&mut out, ts, (b + a) / 2);
            }
        }
    }
    out.sort_by_key(|m| m.ts_ms);
    Ok(out)
}

fn merge_taker(takers: &mut Vec<Taker>, ts: i64, dir: i8, notional: i64) {
    if let Some(l) = takers.last_mut() {
        if l.ts_ms == ts && l.dir == dir {
            l.notional_usd += notional;
            return;
        }
    }
    takers.push(Taker {
        ts_ms: ts,
        dir,
        notional_usd: notional,
    });
}

fn load_agg_trades_csv(path: &Path) -> Result<Vec<Taker>, String> {
    let file = File::open(path).map_err(|e| format!("open {}: {}", path.display(), e))?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut out: Vec<Taker> = Vec::with_capacity(1_200_000);
    for line_res in reader.lines() {
        let line = line_res.map_err(|e| e.to_string())?;
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let mut p = t.split(',');
        if parse_i64_fast(p.next().unwrap_or("")).is_none() {
            continue;
        }
        let price = match parse_fixed_1e8(p.next().unwrap_or("")) {
            Some(x) if x > 0 => x,
            _ => continue,
        };
        let qty = match parse_fixed_1e8(p.next().unwrap_or("")) {
            Some(x) if x > 0 => x,
            _ => continue,
        };
        let _first = p.next();
        let _last = p.next();
        let ts = match parse_timestamp_to_ms(p.next().unwrap_or("")) {
            Some(x) => x,
            None => continue,
        };
        let ibm = parse_bool_fast(p.next().unwrap_or(""));
        let dir: i8 = if ibm { -1 } else { 1 };
        merge_taker(&mut out, ts, dir, notional_usd(price, qty));
    }
    out.sort_by_key(|t| t.ts_ms);
    Ok(out)
}

/// Reconstructs takers and L1 mid-price from raw `trades` CSV:
///   * `MidMode::BidAskProxy`: tracks `last_ask` (buyer aggressor, `ibm == false`) and `last_bid`
///     (seller aggressor, `ibm == true`), setting `mid = (last_bid + last_ask) / 2` once both exist.
///   * `MidMode::TradeHalfTick`: sets `mid = price +/- half_tick` on every execution.
fn load_trades_csv(
    path: &Path,
    half_tick: i64,
    mode: MidMode,
) -> Result<(Vec<Taker>, Vec<Mid>), String> {
    let file = File::open(path).map_err(|e| format!("open {}: {}", path.display(), e))?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut takers: Vec<Taker> = Vec::with_capacity(2_000_000);
    let mut mids: Vec<Mid> = Vec::with_capacity(2_000_000);
    let mut last_bid: Option<i64> = None;
    let mut last_ask: Option<i64> = None;

    for line_res in reader.lines() {
        let line = line_res.map_err(|e| e.to_string())?;
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let mut p = t.split(',');
        if parse_i64_fast(p.next().unwrap_or("")).is_none() {
            continue;
        }
        let price = match parse_fixed_1e8(p.next().unwrap_or("")) {
            Some(x) if x > 0 => x,
            _ => continue,
        };
        let qty = parse_fixed_1e8(p.next().unwrap_or("")).unwrap_or(0);
        let _quote = p.next();
        let ts = match parse_timestamp_to_ms(p.next().unwrap_or("")) {
            Some(x) => x,
            None => continue,
        };
        let ibm = parse_bool_fast(p.next().unwrap_or(""));
        if qty <= 0 {
            continue;
        }
        let dir: i8 = if ibm { -1 } else { 1 };
        match mode {
            MidMode::TradeHalfTick => {
                let mid = if ibm {
                    price + half_tick
                } else {
                    price - half_tick
                };
                push_mid(&mut mids, ts, mid);
            }
            MidMode::BidAskProxy => {
                if ibm {
                    last_bid = Some(price);
                } else {
                    last_ask = Some(price);
                }
                if let (Some(b), Some(a)) = (last_bid, last_ask) {
                    let mid = if a >= b { (b + a) / 2 } else { price };
                    push_mid(&mut mids, ts, mid);
                }
            }
        }
        merge_taker(&mut takers, ts, dir, notional_usd(price, qty));
    }
    takers.sort_by_key(|t| t.ts_ms);
    mids.sort_by_key(|m| m.ts_ms);
    Ok((takers, mids))
}

// ============================================================================
// Run Specifications, CLI Arguments & Daily Parallel Worker
// ============================================================================

struct Spec {
    regime: Regime,
    react: i64,
    decrypt: i64,
    shift: i64,
    cfg: Cfg,
    label: String,
}

struct Args {
    data_dir: String,
    out_dir: String,
    symbols: Vec<String>,
    start: String,
    days: usize,
    window_ms: i64,
    b_window_ms: i64,
    fba_window_ms: i64,
    mm_react: Vec<i64>,
    decrypt: Vec<i64>,
    placebo_shifts: Vec<i64>,
    pending_policy: PendingPolicy,
    mid_mode: MidMode,
    block_hours: i64,
    allow_reconstructed: bool,
    bootstrap: usize,
    threads: usize,
    skip_preflight: bool,
    preflight_only: bool,
}

impl Args {
    fn b_window(&self) -> i64 {
        if self.b_window_ms > 0 {
            self.b_window_ms
        } else {
            self.window_ms
        }
    }
}

fn parse_list_i64(s: &str) -> Vec<i64> {
    s.split(',').filter_map(|x| x.trim().parse().ok()).collect()
}

fn parse_args() -> Args {
    let mut a = Args {
        data_dir: "./data".to_string(),
        out_dir: ".".to_string(),
        symbols: vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()],
        start: "2026-11-10".to_string(),
        days: 7,
        window_ms: 50,
        b_window_ms: 0,
        fba_window_ms: 100,
        mm_react: vec![10],
        decrypt: vec![0, 35, 70],
        placebo_shifts: vec![600_000, 3_600_000],
        pending_policy: PendingPolicy::Hold,
        mid_mode: MidMode::BidAskProxy,
        block_hours: 3,
        allow_reconstructed: false,
        bootstrap: 1000,
        threads: 4,
        skip_preflight: false,
        preflight_only: false,
    };
    let v: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < v.len() {
        let k = v[i].clone();
        match k.as_str() {
            "--allow-reconstructed" => a.allow_reconstructed = true,
            "--skip-preflight" => a.skip_preflight = true,
            "--preflight-only" => a.preflight_only = true,
            _ => {
                let val = v.get(i + 1).cloned().unwrap_or_default();
                i += 1;
                match k.as_str() {
                    "--data-dir" => a.data_dir = val,
                    "--out-dir" => a.out_dir = val,
                    "--symbols" => {
                        a.symbols = val.split(',').map(|s| s.trim().to_string()).collect()
                    }
                    "--start" => a.start = val,
                    "--days" => a.days = val.parse().unwrap_or(7),
                    "--window-ms" => a.window_ms = val.parse().unwrap_or(50),
                    "--b-window-ms" => a.b_window_ms = val.parse().unwrap_or(0),
                    "--fba-window-ms" => a.fba_window_ms = val.parse().unwrap_or(100),
                    "--mm-react" => a.mm_react = parse_list_i64(&val),
                    "--decrypt" => a.decrypt = parse_list_i64(&val),
                    "--placebo-shifts" => a.placebo_shifts = parse_list_i64(&val),
                    "--pending-policy" => match val.trim().to_ascii_lowercase().as_str() {
                        "hold" => a.pending_policy = PendingPolicy::Hold,
                        "drop" => a.pending_policy = PendingPolicy::Drop,
                        _ => {
                            eprintln!("--pending-policy must be 'hold' or 'drop'");
                            std::process::exit(2);
                        }
                    },
                    "--mid-mode" => match val.trim().to_ascii_lowercase().as_str() {
                        "bidask" => a.mid_mode = MidMode::BidAskProxy,
                        "trade" => a.mid_mode = MidMode::TradeHalfTick,
                        _ => {
                            eprintln!("--mid-mode must be 'bidask' or 'trade'");
                            std::process::exit(2);
                        }
                    },
                    "--block-hours" => a.block_hours = val.parse().unwrap_or(3),
                    "--bootstrap" => a.bootstrap = val.parse().unwrap_or(1000),
                    "--threads" => a.threads = val.parse().unwrap_or(4),
                    other => {
                        eprintln!("Unknown CLI argument: {}", other);
                        std::process::exit(2);
                    }
                }
            }
        }
        i += 1;
    }
    a.block_hours = a.block_hours.max(1);
    a.threads = a.threads.max(1);
    if a.mm_react.is_empty() {
        a.mm_react = vec![10];
    }
    if a.decrypt.is_empty() {
        a.decrypt = vec![0, 35, 70];
    }
    a.placebo_shifts.retain(|s| *s != 0);
    a
}

fn push_core_specs(v: &mut Vec<Spec>, args: &Args, r: i64, shift: i64) {
    let suffix = if shift == 0 {
        String::new()
    } else {
        format!(" | PLACEBO shift {}s", shift / 1000)
    };
    let mut base = default_cfg();
    base.mm_react_ms = r;
    base.pending_policy = args.pending_policy;
    let bw = args.b_window();

    let mut a = base;
    a.window_ms = args.window_ms;
    v.push(Spec {
        regime: Regime::Fifo,
        react: r,
        decrypt: 0,
        shift,
        cfg: a,
        label: format!("A: Continuous FIFO | react {}ms{}", r, suffix),
    });
    let mut b = base;
    b.window_ms = bw;
    v.push(Spec {
        regime: Regime::CancelPrio,
        react: r,
        decrypt: 0,
        shift,
        cfg: b,
        label: format!("B: Cancel-Priority {}ms | react {}ms{}", bw, r, suffix),
    });
    let mut c = base;
    c.window_ms = args.fba_window_ms;
    v.push(Spec {
        regime: Regime::UniformFba,
        react: r,
        decrypt: 0,
        shift,
        cfg: c,
        label: format!(
            "C: Uniform-Price FBA {}ms | react {}ms{}",
            args.fba_window_ms, r, suffix
        ),
    });
    for &d in &args.decrypt {
        let mut dd = base;
        dd.window_ms = args.window_ms;
        dd.decrypt_ms = d;
        v.push(Spec {
            regime: Regime::IbteCancelPrio,
            react: r,
            decrypt: d,
            shift,
            cfg: dd,
            label: format!(
                "D: Dywe IBTE + Cancel-Prio {}ms (dec {}ms) | react {}ms{}",
                args.window_ms, d, r, suffix
            ),
        });
    }
}

fn build_specs(args: &Args) -> Vec<Spec> {
    let mut v: Vec<Spec> = Vec::new();
    for &r in &args.mm_react {
        push_core_specs(&mut v, args, r, 0);
    }
    for &s in &args.placebo_shifts {
        push_core_specs(&mut v, args, args.mm_react[0], s);
    }
    v
}

fn find_spec(specs: &[Spec], regime: Regime, react: i64, decrypt: i64, shift: i64) -> Option<usize> {
    specs.iter().position(|s| {
        s.regime == regime && s.react == react && s.decrypt == decrypt && s.shift == shift
    })
}

struct DayResult {
    aggs: Vec<Vec<Agg>>,
    skipped: Option<String>,
    line: String,
    reconstructed: bool,
    mid_ticks: usize,
    taker_orders: usize,
}

fn run_day(
    sym_idx: usize,
    day_idx: usize,
    args: &Args,
    specs: &[Spec],
    start_days: i64,
    nblocks: usize,
) -> DayResult {
    let t0 = Instant::now();
    let sym = args.symbols[sym_idx].clone();
    let date = date_str(start_days + day_idx as i64);
    let skip = |why: String| DayResult {
        aggs: Vec::new(),
        skipped: Some(why),
        line: String::new(),
        reconstructed: false,
        mid_ticks: 0,
        taker_orders: 0,
    };
    let bt_s = format!("{}/{}-bookTicker-{}.csv", args.data_dir, sym, date);
    let tr_s = format!("{}/{}-trades-{}.csv", args.data_dir, sym, date);
    let ag_s = format!("{}/{}-aggTrades-{}.csv", args.data_dir, sym, date);
    let bt = Path::new(&bt_s);
    let tr = Path::new(&tr_s);
    let ag = Path::new(&ag_s);

    let mut trade_takers: Option<Vec<Taker>> = None;
    let mut reconstructed = false;
    let mids: Vec<Mid> = if bt.exists() {
        match load_book_ticker(bt) {
            Ok(m) => m,
            Err(e) => return skip(format!("{} {}: {}", sym, date, e)),
        }
    } else if args.allow_reconstructed && tr.exists() {
        match load_trades_csv(tr, half_tick_1e8(&sym), args.mid_mode) {
            Ok((t, m)) => {
                trade_takers = Some(t);
                reconstructed = true;
                m
            }
            Err(e) => return skip(format!("{} {}: {}", sym, date, e)),
        }
    } else {
        return skip(format!(
            "{} {}: missing bookTicker (and --allow-reconstructed not set)",
            sym, date
        ));
    };
    let takers: Vec<Taker> = if ag.exists() {
        match load_agg_trades_csv(ag) {
            Ok(t) => t,
            Err(e) => return skip(format!("{} {}: {}", sym, date, e)),
        }
    } else if let Some(t) = trade_takers {
        t
    } else if tr.exists() {
        match load_trades_csv(tr, 0, args.mid_mode) {
            Ok((t, _)) => t,
            Err(e) => return skip(format!("{} {}: {}", sym, date, e)),
        }
    } else {
        return skip(format!("{} {}: missing aggTrades/trades", sym, date));
    };
    if mids.is_empty() || takers.is_empty() {
        return skip(format!("{} {}: empty dataset", sym, date));
    }

    let t_first = mids[0].ts_ms;
    let t_last = mids[mids.len() - 1].ts_ms;
    let mut shifted: Vec<(i64, Vec<Taker>)> = Vec::new();
    for sp in specs {
        if sp.shift != 0 && !shifted.iter().any(|(s, _)| *s == sp.shift) {
            shifted.push((sp.shift, shift_takers(&takers, t_first, t_last, sp.shift)));
        }
    }

    let week_start_ms = start_days * 86_400_000;
    let block_ms = args.block_hours * 3_600_000;
    let day_seed = 0xD15E_0000u64 ^ ((sym_idx as u64) << 32) ^ (day_idx as u64);
    let mut aggs: Vec<Vec<Agg>> = Vec::with_capacity(specs.len());
    for sp in specs {
        let mut cfg = sp.cfg;
        cfg.seed = day_seed;
        let flow: &Vec<Taker> = if sp.shift == 0 {
            &takers
        } else {
            shifted
                .iter()
                .find(|(s, _)| *s == sp.shift)
                .map(|(_, v)| v)
                .unwrap_or(&takers)
        };
        let out = simulate(
            &mids,
            flow,
            sp.regime,
            &cfg,
            week_start_ms,
            block_ms,
            nblocks,
        );
        aggs.push(out.agg);
    }
    let src_tag = if reconstructed {
        format!("RECONSTRUCTED({})", args.mid_mode.as_str())
    } else {
        "bookTicker".to_string()
    };
    let line = format!(
        "[OK] {:<8} {} | mid ticks {:>9} | takers {:>8} | source: {} | {:.1?}",
        sym,
        date,
        mids.len(),
        takers.len(),
        src_tag,
        t0.elapsed()
    );
    DayResult {
        aggs,
        skipped: None,
        line,
        reconstructed,
        mid_ticks: mids.len(),
        taker_orders: takers.len(),
    }
}

// ============================================================================
// Calibration context (static notes from public documentation)
// ============================================================================

const CALIBRATION_NOTES: [(&str, &str, &str); 6] = [
    (
        "Hyperliquid: end-to-end latency",
        "The docs define end-to-end latency as the time from sending a request to receiving the committed response. A third-party mirror that cites the official docs reports a median of about 0.2 s and a 99th percentile of about 0.9 s for a co-located client (an older analyst note quotes a 0.3 s median). The numeric values were not visible in the official page excerpt retrieved here.",
        "https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview ; https://www.dwellir.com/docs/hyperliquid/architecture",
    ),
    (
        "Hyperliquid: block interval",
        "Not stated in the official page retrieved. A third-party node-docs site says blocks come about every 70 ms with finality in about two rounds (around 140 ms); an analyst post describes block commits in the range of 200-300 ms and an observed lag of about 700 ms in fills versus Binance (maker update in one block, taker fill in the next). Treat the block interval as roughly 70-300 ms and UNVERIFIED.",
        "https://hypernode-docs.polynode.dev/research/consensus ; https://arrakis.finance/blog/crypto-price-discovery",
    ),
    (
        "dYdX Chain (v4): block time",
        "The official onboarding FAQ states an average block time of about 1 second, and that short-term order cancels are best effort until the block is final. Some third-party pages claim 0.5 s or even 100 ms blocks; they conflict with the official FAQ and are not used here.",
        "https://docs.dydx.xyz/concepts/onboarding-faqs",
    ),
    (
        "Not documented (stay assumptions)",
        "No venue documentation was found for market-maker reaction time, market-data feed latency, client-to-sequencer network latency or latency-arbitrageur reaction time. The model defaults (feed 5 ms, MM reaction 10 ms, sniper reaction 2 ms, network 30 ms) are assumptions. Sweep them (--mm-react) instead of trusting a single value.",
        "n/a",
    ),
    (
        "Dywe window",
        "The 50 ms micro-batch and the 35 ms decryption RTT are Dywe design/benchmark parameters, not venue documentation. They are not validated by any source above.",
        "n/a",
    ),
    (
        "Implication for Regime B",
        "If B is meant to represent Hyperliquid, a 50 ms window is shorter than every documented latency figure above. Use --b-window-ms 70 and --b-window-ms 200 (separate runs) to read the cancel-priority gap at those scales, and --b-window-ms 1000 for a dYdX-like cadence.",
        "n/a",
    ),
];

// ============================================================================
// Synthetic Invariant Pre-Flight & Unit Tests
// ============================================================================

mod selfcheck {
    use super::*;

    pub fn synth(seed: u64, n_ticks: usize) -> (Vec<Mid>, Vec<Taker>) {
        let mut rng = SplitMix64::new(seed);
        let mut mids: Vec<Mid> = Vec::with_capacity(n_ticks);
        let mut ts: i64 = 1_700_000_000_000;
        let mut p: i64 = 9_000_000_000_000;
        for _ in 0..n_ticks {
            ts += 1 + rng.range_i64(0, 3);
            let step = (1 + rng.range_i64(0, 9)) * 10_000_000;
            p += if rng.next_u64() & 1 == 0 { step } else { -step };
            if rng.range_i64(0, 799) == 0 {
                let j = ((p as i128) * ((300 + rng.range_i64(0, 200)) as i128) / SUB) as i64;
                p += if rng.next_u64() & 1 == 0 { j } else { -j };
            }
            mids.push(Mid { ts_ms: ts, mid: p });
        }
        let end = ts;
        let mut takers: Vec<Taker> = Vec::new();
        let mut t = mids[0].ts_ms;
        while t < end {
            t += 5 + rng.range_i64(0, 70);
            let notional = 1_000 + rng.range_i64(0, 120_000);
            let dir: i8 = if rng.next_u64() & 1 == 0 { 1 } else { -1 };
            takers.push(Taker {
                ts_ms: t,
                dir,
                notional_usd: notional,
            });
        }
        (mids, takers)
    }

    fn run(mids: &[Mid], takers: &[Taker], regime: Regime, cfg: &Cfg) -> SimOut {
        simulate(mids, takers, regime, cfg, mids[0].ts_ms, 60_000, 64)
    }

    fn cfg_for(regime: Regime) -> Cfg {
        let mut c = default_cfg();
        c.log = true;
        match regime {
            Regime::UniformFba => c.window_ms = 100,
            Regime::IbteCancelPrio => c.decrypt_ms = 35,
            _ => {}
        }
        c
    }

    pub fn causality() -> Result<String, String> {
        let (mids, takers) = synth(7, 200_000);
        let t_cut = mids[mids.len() / 2].ts_ms;
        let mut mids2 = mids.clone();
        let mut takers2 = takers.clone();
        for m in mids2.iter_mut() {
            if m.ts_ms > t_cut {
                m.mid += if m.ts_ms & 1 == 0 {
                    50_000_000_000
                } else {
                    -30_000_000_000
                };
            }
        }
        for t in takers2.iter_mut() {
            if t.ts_ms > t_cut {
                t.dir = -t.dir;
                t.notional_usd += 77_000;
            }
        }
        let regimes = [
            Regime::Fifo,
            Regime::CancelPrio,
            Regime::UniformFba,
            Regime::IbteCancelPrio,
        ];
        let mut checked = 0usize;
        for &r in regimes.iter() {
            let cfg = cfg_for(r);
            let o1 = run(&mids, &takers, r, &cfg);
            let o2 = run(&mids2, &takers2, r, &cfg);
            let f1: Vec<FillLog> = o1
                .log
                .iter()
                .filter(|x| x.clear_ms <= t_cut)
                .cloned()
                .collect();
            let f2: Vec<FillLog> = o2
                .log
                .iter()
                .filter(|x| x.clear_ms <= t_cut)
                .cloned()
                .collect();
            if f1.is_empty() {
                return Err(format!("causality: regime {} had zero fills prior to T", r.code()));
            }
            if f1 != f2 {
                return Err(format!(
                    "causality: regime {} fills prior to T changed after mutating future ticks",
                    r.code()
                ));
            }
            checked += f1.len();
        }
        Ok(format!(
            "{} fills prior to T matched bit-for-bit across all 4 regimes",
            checked
        ))
    }

    pub fn identity() -> Result<String, String> {
        let (mids, takers) = synth(11, 200_000);
        let mut cb = default_cfg();
        cb.log = false;
        let mut cd = cb;
        cd.decrypt_ms = 0;
        let ob = run(&mids, &takers, Regime::CancelPrio, &cb);
        let od = run(&mids, &takers, Regime::IbteCancelPrio, &cd);
        if ob.agg != od.agg {
            return Err("identity: D(decrypt=0) != B at Agg accumulator level".to_string());
        }
        let e = Econ {
            capture_ppm: 0,
            impact_sub: 200,
            mm_share_ppm: 500_000,
            leak_ppm: 0,
            rebate_sub: 50,
            opex_sub: 30,
            horizon: 2,
        };
        let tb = total(&ob.agg);
        let td = total(&od.agg);
        let eb = evaluate(&tb, Regime::CancelPrio, &e);
        let ed = evaluate(&td, Regime::IbteCancelPrio, &e);
        match (eb, ed) {
            (Some(x), Some(y)) => {
                if x.s_net != y.s_net || x.all_in_net != y.all_in_net {
                    return Err("identity: Outcome(D) != Outcome(B)".to_string());
                }
            }
            _ => return Err("identity: evaluate returned None".to_string()),
        }
        let mut cd35 = cb;
        cd35.decrypt_ms = 35;
        let od35 = run(&mids, &takers, Regime::IbteCancelPrio, &cd35);
        if od35.agg == ob.agg {
            return Ok(
                "D==B at decrypt=0; WARNING: decrypt=35 produced no state change".to_string(),
            );
        }
        Ok("D==B bit-for-bit at decrypt=0, mev=0, leak=0; decrypt=35 alters execution timing".to_string())
    }

    pub fn shuffled_zero_spread() -> Result<String, String> {
        let (mids, base) = synth(21, 150_000);
        let mut cfg = default_cfg();
        cfg.sniper_clip_usd = 0;
        let k: usize = 16;
        let mut means: Vec<f64> = Vec::new();
        for s in 0..k {
            let mut rng = SplitMix64::new(1000 + s as u64);
            let tk: Vec<Taker> = base
                .iter()
                .map(|t| Taker {
                    ts_ms: t.ts_ms,
                    dir: if rng.next_u64() & 1 == 0 { 1 } else { -1 },
                    notional_usd: t.notional_usd,
                })
                .collect();
            let o = run(&mids, &tk, Regime::Fifo, &cfg);
            let a = total(&o.agg);
            if a.mm_n == 0 {
                return Err("shuffled: zero fills recorded".to_string());
            }
            means.push((a.mm_w[2] * Q / a.mm_n) as f64);
        }
        let kf = k as f64;
        let mean = means.iter().sum::<f64>() / kf;
        let var = means.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / (kf - 1.0);
        let se = (var / kf).sqrt();
        if mean.abs() > 4.0 * se + 50.0 {
            return Err(format!(
                "shuffled: |mean|={:.1}q > 4*SE+50 (SE={:.1}q): hidden directional bias",
                mean.abs(),
                se
            ));
        }
        let tk_inf: Vec<Taker> = base
            .iter()
            .map(|t| {
                let a = mid_at(&mids, t.ts_ms);
                let b = mid_at(&mids, t.ts_ms + 1000);
                Taker {
                    ts_ms: t.ts_ms,
                    dir: if b > a { 1 } else { -1 },
                    notional_usd: t.notional_usd,
                }
            })
            .collect();
        let oi = run(&mids, &tk_inf, Regime::Fifo, &cfg);
        let ai = total(&oi.agg);
        let mi = ai.mm_w[2] * Q / ai.mm_n.max(1);
        if mi >= -5_000 {
            return Err(format!(
                "shuffled: informed flow drift={}q, expected strongly negative",
                mi
            ));
        }
        Ok(format!(
            "shuffled mean={:.1}q (SE={:.1}q) ~ 0; informed flow drift={}q < 0",
            mean, se, mi
        ))
    }

    pub fn mev_share_sum() -> Result<String, String> {
        let n: i128 = 1_000_000_000;
        let large: i128 = 200_000_000;
        let a = Agg {
            mm_n: n,
            tk_n: n,
            mm_large_n: large,
            tk_large_n: large,
            ..Agg::default()
        };
        for &m in [0i128, 250_000, 500_000, 1_000_000].iter() {
            let e = Econ {
                capture_ppm: 500_000,
                impact_sub: 200,
                mm_share_ppm: m,
                leak_ppm: 0,
                rebate_sub: 0,
                opex_sub: 0,
                horizon: 2,
            };
            let o = evaluate(&a, Regime::CancelPrio, &e).ok_or("evaluate returned None")?;
            let got = o.mev_mm_q * n + o.mev_tk_q * n;
            let want = e.capture_ppm * e.impact_sub * large * Q / 1_000_000;
            if (got - want).abs() > 2 * n {
                return Err(format!("mev_share_sum: m={} got={} want={}", m, got, want));
            }
        }
        Ok("MM and taker MEV shares sum identically to total extraction".to_string())
    }

    pub fn sniper_protection() -> Result<String, String> {
        let (mids, takers) = synth(31, 300_000);
        let ca = default_cfg();
        let oa = run(&mids, &takers, Regime::Fifo, &ca);
        let ob = run(&mids, &takers, Regime::CancelPrio, &ca);
        let ha = total(&oa.agg).sniper_hit;
        let hb = total(&ob.agg).sniper_hit;
        if ha == 0 {
            return Err("sniper_protection: zero stale sniper hits in Regime A".to_string());
        }
        if hb >= ha {
            return Err(format!(
                "sniper_protection: Regime B ({}) did not reduce stale hits vs Regime A ({})",
                hb, ha
            ));
        }
        Ok(format!("true stale sniper hits: Regime A={} > Regime B={}", ha, hb))
    }

    pub fn accounting_reconciliation() -> Result<String, String> {
        let (mids, takers) = synth(47, 150_000);
        let regimes = [
            Regime::Fifo,
            Regime::CancelPrio,
            Regime::UniformFba,
            Regime::IbteCancelPrio,
        ];
        let econ = Econ {
            capture_ppm: 250_000,
            impact_sub: 150,
            mm_share_ppm: 500_000,
            leak_ppm: 100_000,
            rebate_sub: 50,
            opex_sub: 30,
            horizon: 2,
        };
        for &r in regimes.iter() {
            let cfg = cfg_for(r);
            let out = run(&mids, &takers, r, &cfg);
            let a = total(&out.agg);
            let sum_vol = a.mm_taker_n + a.mm_sniper_stale_n + a.mm_sniper_late_n;
            if a.mm_n != sum_vol {
                return Err(format!(
                    "accounting: regime {} mm_n ({}) != cohort sum ({})",
                    r.code(),
                    a.mm_n,
                    sum_vol
                ));
            }
            let sum_w1s = a.mm_taker_w1s + a.mm_sniper_stale_w1s + a.mm_sniper_late_w1s;
            if a.mm_w[2] != sum_w1s {
                return Err(format!(
                    "accounting: regime {} mm_w[2] ({}) != cohort w1s sum ({})",
                    r.code(),
                    a.mm_w[2],
                    sum_w1s
                ));
            }
            let o = evaluate(&a, r, &econ).ok_or("evaluate returned None")?;
            if let (Some(sp), Some(all_in)) = (o.spread_paid_q, o.all_in_net) {
                let recomputed = sp + o.stale_q + o.wait_q + o.mev_tk_q;
                if all_in != recomputed {
                    return Err(format!(
                        "accounting: regime {} all_in_net ({}) != sum of components ({})",
                        r.code(),
                        all_in,
                        recomputed
                    ));
                }
            }
        }
        Ok("exact volume, cohort markout, and All-In Net component identity verified across all regimes".to_string())
    }

    pub fn placebo_breaks_link() -> Result<String, String> {
        let (mids, base) = synth(53, 150_000);
        let mut cfg = default_cfg();
        cfg.sniper_clip_usd = 0;
        let inf: Vec<Taker> = base
            .iter()
            .map(|t| {
                let a = mid_at(&mids, t.ts_ms);
                let b = mid_at(&mids, t.ts_ms + 1000);
                Taker {
                    ts_ms: t.ts_ms,
                    dir: if b > a { 1 } else { -1 },
                    notional_usd: t.notional_usd,
                }
            })
            .collect();
        let t0 = mids[0].ts_ms;
        let t1 = mids[mids.len() - 1].ts_ms;
        let shifted = shift_takers(&inf, t0, t1, (t1 - t0) / 3);
        if shifted.len() != inf.len() {
            return Err("placebo: shift changed the number of takers".to_string());
        }
        if !shifted.windows(2).all(|w| w[0].ts_ms <= w[1].ts_ms) {
            return Err("placebo: shifted flow is not time-sorted".to_string());
        }
        let notional_before: i64 = inf.iter().map(|t| t.notional_usd).sum();
        let notional_after: i64 = shifted.iter().map(|t| t.notional_usd).sum();
        if notional_before != notional_after {
            return Err("placebo: total notional changed".to_string());
        }
        let ri = total(&run(&mids, &inf, Regime::Fifo, &cfg).agg);
        let rs = total(&run(&mids, &shifted, Regime::Fifo, &cfg).agg);
        let mi = ri.mm_w[2] * Q / ri.mm_n.max(1);
        let ms = rs.mm_w[2] * Q / rs.mm_n.max(1);
        if mi >= -5_000 {
            return Err(format!("placebo: informed drift {}q is not strongly negative", mi));
        }
        if ms.abs() * 4 > mi.abs() {
            return Err(format!(
                "placebo: shifted drift {}q is not small versus informed {}q",
                ms, mi
            ));
        }
        Ok(format!(
            "informed drift {}q -> placebo drift {}q (counts and notional preserved)",
            mi, ms
        ))
    }

    /// Verifies that `--pending-policy drop` drops takers arriving during a cancel-replace gap
    /// instead of holding them until the next quote.
    pub fn pending_drop_policy() -> Result<String, String> {
        let (mids, takers) = synth(61, 150_000);
        let mut cfg_hold = default_cfg();
        cfg_hold.window_ms = 200;
        cfg_hold.pending_policy = PendingPolicy::Hold;

        let mut cfg_drop = cfg_hold;
        cfg_drop.pending_policy = PendingPolicy::Drop;

        let a_hold = total(&run(&mids, &takers, Regime::CancelPrio, &cfg_hold).agg);
        let a_drop = total(&run(&mids, &takers, Regime::CancelPrio, &cfg_drop).agg);

        if a_hold.tk_dropped != 0 {
            return Err("pending_drop: hold policy should have 0 dropped takers".to_string());
        }
        if a_drop.tk_dropped == 0 {
            return Err("pending_drop: drop policy did not drop any cancel-gap takers".to_string());
        }
        if a_drop.tk_fills >= a_hold.tk_fills {
            return Err("pending_drop: drop policy should have fewer fills than hold".to_string());
        }
        Ok(format!(
            "drop policy dropped {} gap takers (${} USD) instead of holding them",
            a_drop.tk_dropped, a_drop.tk_dropped_usd
        ))
    }

    pub fn run_all() -> bool {
        let checks: [(&str, fn() -> Result<String, String>); 8] = [
            ("causality (no look-ahead)", causality),
            ("identity D==B (decrypt=0, mev=0, leak=0)", identity),
            ("shuffled-side zero-spread + sign control", shuffled_zero_spread),
            ("MEV share conservation (no double-count)", mev_share_sum),
            ("sniper protection via cancel-priority", sniper_protection),
            ("accounting reconciliation (volumes & All-In)", accounting_reconciliation),
            ("placebo breaks the flow-price link", placebo_breaks_link),
            ("pending-policy drop vs hold semantics", pending_drop_policy),
        ];
        let mut ok = true;
        for (name, f) in checks.iter() {
            match f() {
                Ok(msg) => println!("[PREFLIGHT OK ] {} — {}", name, msg),
                Err(e) => {
                    println!("[PREFLIGHT FAIL] {} — {}", name, e);
                    ok = false;
                }
            }
        }
        ok
    }
}

#[cfg(test)]
mod tests {
    use super::selfcheck;

    #[test]
    fn t_causality() {
        selfcheck::causality().unwrap();
    }
    #[test]
    fn t_identity() {
        selfcheck::identity().unwrap();
    }
    #[test]
    fn t_shuffled_zero_spread() {
        selfcheck::shuffled_zero_spread().unwrap();
    }
    #[test]
    fn t_mev_share_sum() {
        selfcheck::mev_share_sum().unwrap();
    }
    #[test]
    fn t_sniper_protection() {
        selfcheck::sniper_protection().unwrap();
    }
    #[test]
    fn t_accounting_reconciliation() {
        selfcheck::accounting_reconciliation().unwrap();
    }
    #[test]
    fn t_placebo_breaks_link() {
        selfcheck::placebo_breaks_link().unwrap();
    }
    #[test]
    fn t_pending_drop_policy() {
        selfcheck::pending_drop_policy().unwrap();
    }
    #[test]
    fn t_html_escape() {
        assert_eq!(super::html_escape("D<B & B<D"), "D&lt;B &amp; B&lt;D");
    }
}

// ============================================================================
// Markdown & HTML Report Generation
// ============================================================================

fn base_econ() -> Econ {
    Econ {
        capture_ppm: 0,
        impact_sub: 0,
        mm_share_ppm: 500_000,
        leak_ppm: 0,
        rebate_sub: 50,
        opex_sub: 30,
        horizon: 2,
    }
}

fn markout1s(a: &Agg) -> Option<i128> {
    if a.mm_n > 0 {
        Some(a.mm_w[2] * Q / a.mm_n)
    } else {
        None
    }
}

fn table(rep: &mut String, title: &str, specs: &[Spec], totals: &[Agg], shift: i64) {
    let e0 = base_econ();
    emit!(rep, "\n### {}", title);
    emit!(
        rep,
        "| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |"
    );
    emit!(
        rep,
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
    );
    for (sp, a) in specs.iter().zip(totals.iter()) {
        if sp.shift != shift {
            continue;
        }
        match evaluate(a, sp.regime, &e0) {
            None => {
                emit!(rep, "| {} | n/a | | | | | | | | | | | | |", sp.label);
            }
            Some(o) => {
                let m0 = a.mm_w[0] * Q / a.mm_n;
                let m1 = a.mm_w[1] * Q / a.mm_n;
                let m2 = a.mm_w[2] * Q / a.mm_n;
                emit!(
                    rep,
                    "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}/{} | {} | {} |",
                    sp.label,
                    fmt_usd(a.mm_n),
                    fmt_bps(m0),
                    fmt_bps(m1),
                    fmt_bps(m2),
                    fmt_s(o.s_gross),
                    fmt_s(o.s_net),
                    fmt_opt(o.spread_paid_q),
                    fmt_bps(o.stale_q),
                    fmt_bps(o.wait_q),
                    fmt_opt(o.all_in_net),
                    a.sniper_hit,
                    a.sniper_missed,
                    a.tk_dropped,
                    a.batches
                );
            }
        }
    }
    emit!(
        rep,
        "\nUnits: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding)."
    );
}

fn cohort_table(rep: &mut String, title: &str, specs: &[Spec], totals: &[Agg]) {
    emit!(rep, "\n### {} — MM Flow Cohort Decomposition & Volume Reconciliation", title);
    emit!(
        rep,
        "| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout |"
    );
    emit!(rep, "|---|---:|---:|---:|---:|---:|---:|---:|---:|");
    for (sp, a) in specs.iter().zip(totals.iter()) {
        if sp.shift != 0 {
            continue;
        }
        let sum_v = a.mm_taker_n + a.mm_sniper_stale_n + a.mm_sniper_late_n;
        let diff_v = a.mm_n - sum_v;
        let m_tk = if a.mm_taker_n > 0 {
            fmt_bps(a.mm_taker_w1s * Q / a.mm_taker_n)
        } else {
            "n/a".to_string()
        };
        let m_st = if a.mm_sniper_stale_n > 0 {
            fmt_bps(a.mm_sniper_stale_w1s * Q / a.mm_sniper_stale_n)
        } else {
            "n/a".to_string()
        };
        let m_lt = if a.mm_sniper_late_n > 0 {
            fmt_bps(a.mm_sniper_late_w1s * Q / a.mm_sniper_late_n)
        } else {
            "n/a".to_string()
        };
        emit!(
            rep,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            sp.label,
            fmt_usd(a.mm_taker_n),
            fmt_usd(a.mm_sniper_stale_n),
            fmt_usd(a.mm_sniper_late_n),
            fmt_usd(a.mm_n),
            fmt_usd(diff_v),
            m_tk,
            m_st,
            m_lt
        );
    }
}

fn placebo_lines(specs: &[Spec], full: &[Agg], args: &Args) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let r0 = args.mm_react[0];
    let (ia, ib) = match (
        find_spec(specs, Regime::Fifo, r0, 0, 0),
        find_spec(specs, Regime::CancelPrio, r0, 0, 0),
    ) {
        (Some(a), Some(b)) => (a, b),
        _ => return out,
    };
    let (ma, mb) = match (markout1s(&full[ia]), markout1s(&full[ib])) {
        (Some(a), Some(b)) => (a, b),
        _ => return out,
    };
    let gap = mb - ma;
    out.push(format!(
        "Real flow (react {}ms): A {} bp, B {} bp, cancel-priority gap (B minus A) {} bp.",
        r0,
        fmt_bps(ma),
        fmt_bps(mb),
        fmt_bps(gap)
    ));
    for &s in &args.placebo_shifts {
        let (pa, pb) = match (
            find_spec(specs, Regime::Fifo, r0, 0, s),
            find_spec(specs, Regime::CancelPrio, r0, 0, s),
        ) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        let (mpa, mpb) = match (markout1s(&full[pa]), markout1s(&full[pb])) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        let gap_p = mpb - mpa;
        let share = if gap != 0 { gap_p * 1000 / gap } else { 0 };
        out.push(format!(
            "Placebo shift {} s: A {} bp, B {} bp, gap {} bp = {} of the real gap.",
            s / 1000,
            fmt_bps(mpa),
            fmt_bps(mpb),
            fmt_bps(gap_p),
            fmt_permille(share)
        ));
    }
    out
}

fn gap_vs_ratio_lines(specs: &[Spec], full: &[Agg], args: &Args) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for &r in &args.mm_react {
        let (ia, ib) = match (
            find_spec(specs, Regime::Fifo, r, 0, 0),
            find_spec(specs, Regime::CancelPrio, r, 0, 0),
        ) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        let (ma, mb) = match (markout1s(&full[ia]), markout1s(&full[ib])) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        let w = specs[ib].cfg.window_ms.max(1);
        let d = r - specs[ib].cfg.sniper_react_ms;
        let gap = mb - ma;
        let removed = if ma != 0 { gap * 1000 / ma.abs() } else { 0 };
        out.push(format!(
            "react {}ms: handicap (MM minus sniper reaction) {}ms, B window {}ms, handicap/window {}%, gap {} bp = {} of A's 1s markout magnitude.",
            r,
            d,
            w,
            d * 100 / w,
            fmt_bps(gap),
            fmt_permille(removed)
        ));
    }
    out
}

struct FrontierGrid {
    decrypt_ms: i64,
    react_ms: i64,
    cells: Vec<String>,
}

fn frontier(
    rep: &mut String,
    csv: &mut String,
    specs: &[Spec],
    full: &[Agg],
    boot: &[Vec<Agg>],
    args: &Args,
) -> Vec<FrontierGrid> {
    let mut html_grids = Vec::new();
    csv.push_str("react_ms,decrypt_ms,b_window_ms,d_window_ms,pending_policy,mid_mode,capture_ppm,impact_sub,mm_share_ppm,leak_ppm,allin_B_q,allin_D_q,delta_D_minus_B_q,ci_lo_q,ci_hi_q,verdict\n");
    for &react in &args.mm_react {
        let ib = match find_spec(specs, Regime::CancelPrio, react, 0, 0) {
            Some(i) => i,
            None => continue,
        };
        for &dec in &args.decrypt {
            let id = match find_spec(specs, Regime::IbteCancelPrio, react, dec, 0) {
                Some(i) => i,
                None => continue,
            };
            let bw = specs[ib].cfg.window_ms;
            let dw = specs[id].cfg.window_ms;
            let mut cells: Vec<String> = vec![String::new(); CAPTURES_PPM.len() * IMPACTS_SUB.len()];
            for &share in SHARES_PPM.iter() {
                for &leak in LEAKS_PPM.iter() {
                    for (ci_idx, &cap) in CAPTURES_PPM.iter().enumerate() {
                        for (ii_idx, &imp) in IMPACTS_SUB.iter().enumerate() {
                            let e = Econ {
                                capture_ppm: cap,
                                impact_sub: imp,
                                mm_share_ppm: share,
                                leak_ppm: leak,
                                rebate_sub: 50,
                                opex_sub: 30,
                                horizon: 2,
                            };
                            let ab = evaluate(&full[ib], Regime::CancelPrio, &e)
                                .and_then(|o| o.all_in_net);
                            let ad = evaluate(&full[id], Regime::IbteCancelPrio, &e)
                                .and_then(|o| o.all_in_net);
                            let d_full = delta_all_in(&full[ib], &full[id], &e);
                            let mut samples: Vec<i128> = Vec::new();
                            if !boot[ib].is_empty() && boot[ib].len() == boot[id].len() {
                                for r in 0..boot[ib].len() {
                                    if let Some(d) = delta_all_in(&boot[ib][r], &boot[id][r], &e) {
                                        samples.push(d);
                                    }
                                }
                            }
                            let ci = ci95(samples);
                            let vd = verdict(ci);
                            let (lo, hi) = match ci {
                                Some((l, h)) => (l.to_string(), h.to_string()),
                                None => (String::new(), String::new()),
                            };
                            csv.push_str(&format!(
                                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                                react,
                                dec,
                                bw,
                                dw,
                                args.pending_policy.as_str(),
                                args.mid_mode.as_str(),
                                cap,
                                imp,
                                share,
                                leak,
                                ab.map_or(String::new(), |x| x.to_string()),
                                ad.map_or(String::new(), |x| x.to_string()),
                                d_full.map_or(String::new(), |x| x.to_string()),
                                lo,
                                hi,
                                vd
                            ));
                            if share == 500_000 && leak == 100_000 && react == args.mm_react[0] {
                                cells[ci_idx * IMPACTS_SUB.len() + ii_idx] = match d_full {
                                    Some(x) => format!("{} {}", fmt_bps(x), vd),
                                    None => "infeasible".to_string(),
                                };
                            }
                        }
                    }
                }
            }
            if react == args.mm_react[0] {
                emit!(
                    rep,
                    "\n#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window {}ms, D window {}ms, decrypt {}ms | react {}ms | pending={} | mid={}",
                    bw,
                    dw,
                    dec,
                    react,
                    args.pending_policy.as_str(),
                    args.mid_mode.as_str()
                );
                let mut head = String::from("| Capture \\ Impact |");
                for &imp in IMPACTS_SUB.iter() {
                    head.push_str(&format!(" {} bp |", fmt_bps(imp * Q)));
                }
                emit!(rep, "{}", head);
                let mut sep = String::from("|---|");
                for _ in IMPACTS_SUB.iter() {
                    sep.push_str("---:|");
                }
                emit!(rep, "{}", sep);
                for (ci_idx, &cap) in CAPTURES_PPM.iter().enumerate() {
                    let mut row = format!("| {}% |", cap / 10_000);
                    for ii_idx in 0..IMPACTS_SUB.len() {
                        row.push_str(&format!(" {} |", cells[ci_idx * IMPACTS_SUB.len() + ii_idx]));
                    }
                    emit!(rep, "{}", row);
                }
                html_grids.push(FrontierGrid {
                    decrypt_ms: dec,
                    react_ms: react,
                    cells,
                });
            }
        }
    }
    html_grids
}

fn render_mech_rows_html(specs: &[Spec], totals: &[Agg], shift: i64) -> String {
    let e0 = base_econ();
    let mut rows = String::new();
    for (sp, a) in specs.iter().zip(totals.iter()) {
        if sp.shift != shift {
            continue;
        }
        let is_dywe = sp.regime == Regime::IbteCancelPrio && sp.decrypt == 35;
        let tr_cls = if is_dywe { " class=\"highlight-row\"" } else { "" };
        let label = html_escape(&sp.label);
        match evaluate(a, sp.regime, &e0) {
            None => {
                rows.push_str(&format!(
                    "<tr{}><td>{}</td><td colspan=\"13\">n/a</td></tr>\n",
                    tr_cls, label
                ));
            }
            Some(o) => {
                let m0 = a.mm_w[0] * Q / a.mm_n;
                let m1 = a.mm_w[1] * Q / a.mm_n;
                let m2 = a.mm_w[2] * Q / a.mm_n;
                rows.push_str(&format!(
                    "<tr{}><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><strong>{}</strong></td><td>{}/{}</td><td>{}</td><td>{}</td></tr>\n",
                    tr_cls,
                    label,
                    fmt_usd(a.mm_n),
                    fmt_bps(m0),
                    fmt_bps(m1),
                    fmt_bps(m2),
                    html_escape(&fmt_s(o.s_gross)),
                    html_escape(&fmt_s(o.s_net)),
                    html_escape(&fmt_opt(o.spread_paid_q)),
                    fmt_bps(o.stale_q),
                    fmt_bps(o.wait_q),
                    html_escape(&fmt_opt(o.all_in_net)),
                    a.sniper_hit,
                    a.sniper_missed,
                    a.tk_dropped,
                    a.batches
                ));
            }
        }
    }
    rows
}

fn render_cohort_rows_html(specs: &[Spec], totals: &[Agg]) -> String {
    let mut rows = String::new();
    for (sp, a) in specs.iter().zip(totals.iter()) {
        if sp.shift != 0 {
            continue;
        }
        let is_dywe = sp.regime == Regime::IbteCancelPrio && sp.decrypt == 35;
        let tr_cls = if is_dywe { " class=\"highlight-row\"" } else { "" };
        let sum_v = a.mm_taker_n + a.mm_sniper_stale_n + a.mm_sniper_late_n;
        let diff_v = a.mm_n - sum_v;
        let m_tk = if a.mm_taker_n > 0 {
            fmt_bps(a.mm_taker_w1s * Q / a.mm_taker_n)
        } else {
            "n/a".to_string()
        };
        let m_st = if a.mm_sniper_stale_n > 0 {
            fmt_bps(a.mm_sniper_stale_w1s * Q / a.mm_sniper_stale_n)
        } else {
            "n/a".to_string()
        };
        let m_lt = if a.mm_sniper_late_n > 0 {
            fmt_bps(a.mm_sniper_late_w1s * Q / a.mm_sniper_late_n)
        } else {
            "n/a".to_string()
        };
        rows.push_str(&format!(
            "<tr{}><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            tr_cls,
            html_escape(&sp.label),
            fmt_usd(a.mm_taker_n),
            fmt_usd(a.mm_sniper_stale_n),
            fmt_usd(a.mm_sniper_late_n),
            fmt_usd(a.mm_n),
            fmt_usd(diff_v),
            m_tk,
            m_st,
            m_lt
        ));
    }
    rows
}

fn mech_table_html(title: &str, rows_html: &str) -> String {
    format!(
        r#"<div class="table-card" style="margin-bottom: 22px;">
                <h2>{}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>Architecture Regime</th>
                            <th>MM Total Vol</th>
                            <th>Markout 0ms</th>
                            <th>Markout 100ms</th>
                            <th>Markout 1s</th>
                            <th>s* Gross</th>
                            <th>s* Net</th>
                            <th>[1] Spread Paid</th>
                            <th>[2] Taker Stale</th>
                            <th>[3] Informed Wait</th>
                            <th>[=] All-In Net</th>
                            <th>StaleSnipe Hit/Miss</th>
                            <th>Dropped Takers</th>
                            <th>Batches</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>"#,
        html_escape(title),
        rows_html
    )
}

fn render_placebo_html(specs: &[Spec], full: &[Agg], args: &Args) -> String {
    let mut out = String::new();
    let lines = placebo_lines(specs, full, args);
    if lines.is_empty() {
        return out;
    }
    out.push_str(
        "<div class=\"bounds-box\"><h3>Placebo test: circularly shifted taker flow</h3><ul>",
    );
    for l in &lines {
        out.push_str(&format!("<li>{}</li>", html_escape(l)));
    }
    out.push_str("</ul></div>");
    for &s in &args.placebo_shifts {
        let rows = render_mech_rows_html(specs, full, s);
        if !rows.is_empty() {
            out.push_str(&mech_table_html(
                &format!("Placebo: takers shifted {}s (consolidated, MEV = 0)", s / 1000),
                &rows,
            ));
        }
    }
    out
}

fn render_calibration_html() -> String {
    let mut out = String::from(
        "<div class=\"bounds-box\"><h3>Calibration context (public documentation, retrieved 2026-11-18; not simulation inputs)</h3><ul>",
    );
    for (topic, statement, source) in CALIBRATION_NOTES.iter() {
        out.push_str(&format!(
            "<li><strong>{}:</strong> {} <em>Source: {}</em></li>",
            html_escape(topic),
            html_escape(statement),
            html_escape(source)
        ));
    }
    out.push_str("</ul></div>");
    out
}

fn generate_html_report(
    output_path: &Path,
    args: &Args,
    days_ok: usize,
    total_tasks: usize,
    days_reconstructed: usize,
    total_mid_ticks: usize,
    total_taker_orders: usize,
    nblocks: usize,
    specs: &[Spec],
    per_sym: &[Vec<Vec<Agg>>],
    full: &[Agg],
    html_grids: &[FrontierGrid],
    sign_check_text: &str,
) -> Result<(), String> {
    let file = File::create(output_path)
        .map_err(|e| format!("Failed to create {}: {}", output_path.display(), e))?;
    let mut w = BufWriter::new(file);

    let e0 = base_econ();
    let react0 = args.mm_react[0];
    let ia = find_spec(specs, Regime::Fifo, react0, 0, 0).unwrap_or(0);
    let ib = find_spec(specs, Regime::CancelPrio, react0, 0, 0).unwrap_or(0);
    let ic = find_spec(specs, Regime::UniformFba, react0, 0, 0).unwrap_or(0);
    let id35 = find_spec(specs, Regime::IbteCancelPrio, react0, 35, 0)
        .or_else(|| find_spec(specs, Regime::IbteCancelPrio, react0, 0, 0))
        .unwrap_or(0);
    let d_dec = specs[id35].decrypt;
    let b_win = specs[ib].cfg.window_ms;
    let c_win = specs[ic].cfg.window_ms;
    let d_win = specs[id35].cfg.window_ms;

    let oa = evaluate(&full[ia], Regime::Fifo, &e0);
    let ob = evaluate(&full[ib], Regime::CancelPrio, &e0);
    let od35 = evaluate(&full[id35], Regime::IbteCancelPrio, &e0);

    let d35_snet = od35.map_or("n/a".to_string(), |o| fmt_s(o.s_net));
    let a_snet = oa.map_or("n/a".to_string(), |o| fmt_s(o.s_net));
    let b_snet = ob.map_or("n/a".to_string(), |o| fmt_s(o.s_net));

    let d35_m1s = if full[id35].mm_n > 0 {
        fmt_bps(full[id35].mm_w[2] * Q / full[id35].mm_n)
    } else {
        "n/a".to_string()
    };
    let a_m1s = if full[ia].mm_n > 0 {
        fmt_bps(full[ia].mm_w[2] * Q / full[ia].mm_n)
    } else {
        "n/a".to_string()
    };

    let zero_mev_cost = match (ob.and_then(|o| o.all_in_net), od35.and_then(|o| o.all_in_net)) {
        (Some(b_val), Some(d_val)) => format!("{} bps", fmt_bps_signed(d_val - b_val)),
        _ => "n/a".to_string(),
    };

    let e_mid = Econ {
        capture_ppm: 250_000,
        impact_sub: 100,
        mm_share_ppm: 500_000,
        leak_ppm: 100_000,
        rebate_sub: 50,
        opex_sub: 30,
        horizon: 2,
    };
    let mid_scenario_delta = delta_all_in(&full[ib], &full[id35], &e_mid)
        .map_or("n/a".to_string(), |d| format!("{} bps", fmt_bps_signed(d)));

    let extract_chart_series = |sym_idx: usize, spec_idx: usize| -> String {
        if sym_idx >= per_sym.len() {
            return "0, 0, 0".to_string();
        }
        let agg = total(&per_sym[sym_idx][spec_idx]);
        if agg.mm_n <= 0 {
            return "0, 0, 0".to_string();
        }
        format!(
            "{}, {}, {}",
            fmt_bps(agg.mm_w[0] * Q / agg.mm_n),
            fmt_bps(agg.mm_w[1] * Q / agg.mm_n),
            fmt_bps(agg.mm_w[2] * Q / agg.mm_n)
        )
    };

    let btc_a = extract_chart_series(0, ia);
    let btc_b = extract_chart_series(0, ib);
    let btc_c = extract_chart_series(0, ic);
    let btc_d = extract_chart_series(0, id35);

    let eth_idx = if per_sym.len() > 1 { 1 } else { 0 };
    let eth_a = extract_chart_series(eth_idx, ia);
    let eth_b = extract_chart_series(eth_idx, ib);
    let eth_c = extract_chart_series(eth_idx, ic);
    let eth_d = extract_chart_series(eth_idx, id35);

    let sym1_name = html_escape(&args.symbols.get(0).cloned().unwrap_or_else(|| "BTCUSDT".to_string()));
    let sym2_name = html_escape(&args.symbols.get(1).cloned().unwrap_or_else(|| "ETHUSDT".to_string()));

    let cons_rows_html = render_mech_rows_html(specs, full, 0);
    let cons_cohort_html = render_cohort_rows_html(specs, full);

    let mut per_sym_sections_html = String::new();
    for (s_idx, sym_name) in args.symbols.iter().enumerate() {
        let tot: Vec<Agg> = per_sym[s_idx].iter().map(|c| total(c)).collect();
        let rows_html = render_mech_rows_html(specs, &tot, 0);
        per_sym_sections_html.push_str(&mech_table_html(
            &format!("{} — Pure Microstructure Mechanics (MEV = 0)", sym_name),
            &rows_html,
        ));
    }

    let placebo_html = render_placebo_html(specs, full, args);
    let calibration_html = render_calibration_html();

    let ratio_lines = gap_vs_ratio_lines(specs, full, args);
    let mut ratio_html = String::new();
    if !ratio_lines.is_empty() {
        ratio_html.push_str("<div class=\"bounds-box\"><h3>Cancel-priority gap versus reaction handicap / window</h3><ul>");
        for l in &ratio_lines {
            ratio_html.push_str(&format!("<li>{}</li>", html_escape(l)));
        }
        ratio_html.push_str("</ul></div>");
    }

    let mut frontier_cards_html = String::new();
    for fg in html_grids {
        let mut tbody = String::new();
        for (ci_idx, &cap) in CAPTURES_PPM.iter().enumerate() {
            tbody.push_str(&format!("<tr><td><strong>{}%</strong></td>", cap / 10_000));
            for ii_idx in 0..IMPACTS_SUB.len() {
                let cell = &fg.cells[ci_idx * IMPACTS_SUB.len() + ii_idx];
                let cls = if cell.contains("D<B") {
                    "cell-win"
                } else if cell.contains("B<D") {
                    "cell-loss"
                } else {
                    "cell-neutral"
                };
                tbody.push_str(&format!("<td class=\"{}\">{}</td>", cls, html_escape(cell)));
            }
            tbody.push_str("</tr>\n");
        }
        frontier_cards_html.push_str(&format!(
            r#"<div class="table-card" style="margin-bottom: 20px;">
                <h2>Break-Even Frontier: All-In(D) &minus; All-In(B) (bps) &mdash; Decrypt {}ms | React {}ms</h2>
                <p style="font-size: 12px; color: var(--text-muted); margin-bottom: 12px;">
                    Assumptions: MM MEV Loss Share = 50%, Cancel-Header Leakage = 10%, Maker Rebate = +0.50 bps revenue, Hedge OpEx = 0.30 bps, Pending Policy = <code>{}</code>, Mid Mode = <code>{}</code>.
                </p>
                <table>
                    <thead>
                        <tr>
                            <th>Capture Rate \ Impact</th>
                            <th>0.000 bps</th>
                            <th>0.500 bps</th>
                            <th>1.000 bps</th>
                            <th>2.000 bps</th>
                            <th>3.000 bps</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>"#,
            fg.decrypt_ms,
            fg.react_ms,
            args.pending_policy.as_str(),
            args.mid_mode.as_str(),
            tbody
        ));
    }

    let data_source_badge = if days_reconstructed > 0 {
        format!("Reconstructed ({}, {}/{}d)", args.mid_mode.as_str(), days_reconstructed, days_ok)
    } else {
        "L1 BookTicker Mid".to_string()
    };

    let recon_banner = if days_reconstructed > 0 {
        format!(
            r#"<div class="warn-banner">
                <strong>Methodological Note:</strong> {} of {} days used reconstructed L1 mid-prices (mode: <code>{}</code>) with pending policy <code>{}</code>.
            </div>"#,
            days_reconstructed,
            days_ok,
            args.mid_mode.as_str(),
            args.pending_policy.as_str()
        )
    } else {
        String::new()
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dywe Protocol — Causal Microstructure &amp; Break-Even Ablation Report (v0.3.1)</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.1/dist/chart.umd.min.js"></script>
    <style>
        :root {{
            --bg-primary: #060913;
            --bg-card: #0d1323;
            --bg-card-alt: #131c31;
            --border-color: #1e293b;
            --border-highlight: rgba(194, 138, 255, 0.35);
            --text-main: #f8fafc;
            --text-muted: #94a3b8;
            --accent-purple: #c28aff;
            --accent-blue: #38bdf8;
            --accent-amber: #f59e0b;
            --accent-red: #f43f5e;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            background-color: var(--bg-primary);
            background-image: radial-gradient(circle at 50% 0%, rgba(194, 138, 255, 0.06) 0%, transparent 55%);
            color: var(--text-main);
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            line-height: 1.5;
            padding: 28px 40px 44px 40px;
        }}
        .container {{ max-width: 1520px; margin: 0 auto; }}
        .top-bar {{
            display: flex; justify-content: space-between; align-items: center;
            margin-bottom: 18px; font-family: 'JetBrains Mono', monospace;
            font-size: 11px; letter-spacing: 0.08em; text-transform: uppercase;
        }}
        .brand-pill {{
            display: inline-flex; align-items: center; gap: 8px;
            color: var(--accent-purple); background: rgba(194, 138, 255, 0.08);
            border: 1px solid var(--border-highlight); padding: 5px 12px;
            border-radius: 999px; font-weight: 600;
        }}
        .brand-dot {{
            width: 7px; height: 7px; border-radius: 50%;
            background: var(--accent-purple); box-shadow: 0 0 8px var(--accent-purple);
        }}
        .engine-tag {{ color: var(--text-muted); }}
        header {{
            display: flex; justify-content: space-between; align-items: flex-end;
            gap: 24px; border-bottom: 1px solid var(--border-color);
            padding-bottom: 22px; margin-bottom: 24px;
        }}
        .title-group h1 {{ font-size: 24px; font-weight: 700; letter-spacing: -0.02em; color: #ffffff; }}
        .title-group p {{ color: var(--text-muted); font-size: 13px; margin-top: 6px; }}
        .meta-badges {{ display: flex; gap: 8px; flex-wrap: wrap; justify-content: flex-end; }}
        .badge {{
            background: var(--bg-card-alt); border: 1px solid var(--border-color);
            padding: 5px 11px; border-radius: 6px; font-size: 11px;
            font-family: 'JetBrains Mono', monospace; color: #cbd5e1;
        }}
        .badge strong {{ color: var(--accent-purple); font-weight: 600; }}
        .warn-banner {{
            background: rgba(245, 158, 11, 0.08); border: 1px solid rgba(245, 158, 11, 0.35);
            color: #fde68a; padding: 12px 16px; border-radius: 8px; font-size: 12.5px;
            margin-bottom: 22px;
        }}
        .kpi-grid {{
            display: grid; grid-template-columns: repeat(4, 1fr);
            gap: 18px; margin-bottom: 24px;
        }}
        .kpi-card {{
            background: linear-gradient(180deg, rgba(19, 28, 49, 0.9) 0%, var(--bg-card) 100%);
            border: 1px solid var(--border-color); border-top: 1px solid rgba(194, 138, 255, 0.25);
            border-radius: 12px; padding: 18px 20px;
        }}
        .kpi-card .label {{
            font-size: 10.5px; font-weight: 600; text-transform: uppercase;
            letter-spacing: 0.06em; color: var(--text-muted);
        }}
        .kpi-card .value {{
            font-size: 25px; font-weight: 700; color: var(--accent-purple);
            margin: 8px 0; font-family: 'JetBrains Mono', monospace; letter-spacing: -0.02em;
        }}
        .kpi-pill {{
            display: inline-block; font-size: 11px; font-family: 'JetBrains Mono', monospace;
            color: #e2e8f0; background: rgba(56, 189, 248, 0.1);
            border: 1px solid rgba(56, 189, 248, 0.25); padding: 3px 8px; border-radius: 5px;
        }}
        .charts-row {{
            display: grid; grid-template-columns: repeat(2, 1fr);
            gap: 20px; margin-bottom: 24px;
        }}
        .chart-card, .table-card {{
            background: var(--bg-card); border: 1px solid var(--border-color);
            border-radius: 12px; padding: 22px 24px; overflow-x: auto;
        }}
        .chart-card h2, .table-card h2 {{ font-size: 15.5px; font-weight: 600; margin-bottom: 6px; color: #ffffff; }}
        .chart-card p {{ font-size: 12px; color: var(--text-muted); margin-bottom: 14px; }}
        .chart-wrapper {{ position: relative; height: 320px; width: 100%; }}
        table {{ width: 100%; border-collapse: collapse; font-size: 11.5px; }}
        th, td {{
            padding: 9px 9px; text-align: right; border-bottom: 1px solid var(--border-color);
            font-family: 'JetBrains Mono', monospace; white-space: nowrap;
        }}
        th:first-child, td:first-child {{ text-align: left; font-family: 'Inter', sans-serif; font-weight: 600; }}
        th {{
            color: var(--text-muted); font-size: 9.5px; text-transform: uppercase;
            letter-spacing: 0.05em; background: var(--bg-card-alt);
        }}
        tr.highlight-row {{ background: rgba(194, 138, 255, 0.08); }}
        tr.highlight-row td {{ color: var(--accent-purple); font-weight: 600; }}
        td.cell-win {{ color: #c28aff; background: rgba(194, 138, 255, 0.07); font-weight: 600; }}
        td.cell-loss {{ color: #fbbf24; background: rgba(245, 158, 11, 0.06); }}
        td.cell-neutral {{ color: #94a3b8; }}
        .bounds-box {{
            background: var(--bg-card); border: 1px solid var(--border-color);
            border-left: 3px solid var(--accent-purple); border-radius: 10px;
            padding: 18px 22px; margin: 22px 0; font-size: 12.5px; color: var(--text-muted);
        }}
        .bounds-box h3 {{ color: #ffffff; font-size: 14px; margin-bottom: 8px; }}
        .bounds-box ul {{ padding-left: 18px; display: flex; flex-direction: column; gap: 5px; }}
        code {{ color: var(--accent-purple); background: var(--bg-card-alt); padding: 2px 5px; border-radius: 4px; }}
    </style>
</head>
<body>
<div class="container">
    <div class="top-bar">
        <span class="brand-pill"><span class="brand-dot"></span>DYWE PROTOCOL // CAUSAL MICROSTRUCTURE RESEARCH (v0.3.1)</span>
        <span class="engine-tag">PENDING={pending_pol} &bull; MID={mid_mode} &bull; DETERMINISTIC i128</span>
    </div>

    <header>
        <div class="title-group">
            <h1>DYWE EXCHANGE &mdash; CAUSAL MICROSTRUCTURE &amp; BREAK-EVEN ABLATION STUDY</h1>
            <p>Scenario-Based Counterfactual Replay on Binance Futures flow ({syms}, {start} + {days}d | {nblocks} Joint {bh}h Bootstrap Blocks)</p>
        </div>
        <div class="meta-badges">
            <span class="badge">Pair-Days: <strong>{days_ok}/{total_tasks}</strong></span>
            <span class="badge">Mid Ticks: <strong>{total_mid_ticks}</strong></span>
            <span class="badge">Taker Flow: <strong>{total_taker_orders}</strong></span>
            <span class="badge">Pending: <strong>{pending_pol}</strong></span>
            <span class="badge">Mid Mode: <strong>{mid_mode}</strong></span>
            <span class="badge">Source: <strong>{data_source_badge}</strong></span>
        </div>
    </header>

    {recon_banner}

    <div class="kpi-grid">
        <div class="kpi-card">
            <div class="label">Net Half-Spread s* (MEV=0), Regime D ({d_dec}ms decrypt)</div>
            <div class="value">{d35_snet} bps</div>
            <span class="kpi-pill">vs {a_snet} bps (FIFO) &amp; {b_snet} bps (Cancel-Prio {b_win}ms)</span>
        </div>
        <div class="kpi-card">
            <div class="label">1s MM Markout (MEV=0), Regime D</div>
            <div class="value">{d35_m1s} bps</div>
            <span class="kpi-pill">vs {a_m1s} bps (FIFO); see placebo section</span>
        </div>
        <div class="kpi-card">
            <div class="label">{d_dec}ms Decrypt Cost vs Cancel-Prio (Zero-MEV)</div>
            <div class="value">{zero_mev_cost}</div>
            <span class="kpi-pill">All-In Taker Cost delta</span>
        </div>
        <div class="kpi-card">
            <div class="label">Scenario &Delta; All-In (25% capture, 1 bp, 10% leak)</div>
            <div class="value">{mid_scenario_delta}</div>
            <span class="kpi-pill">Scenario output &mdash; not a measurement</span>
        </div>
    </div>

    <div class="charts-row">
        <div class="chart-card">
            <h2>Chart 1A: {sym1_name} Signed MM Markout Trajectory (bps)</h2>
            <p>Volume-weighted post-trade MM markout across 0 ms &rarr; 1000 ms (MEV = 0 pure mechanics)</p>
            <div class="chart-wrapper"><canvas id="chartSym1"></canvas></div>
        </div>
        <div class="chart-card">
            <h2>Chart 1B: {sym2_name} Signed MM Markout Trajectory (bps)</h2>
            <p>Volume-weighted post-trade MM markout across 0 ms &rarr; 1000 ms (MEV = 0 pure mechanics)</p>
            <div class="chart-wrapper"><canvas id="chartSym2"></canvas></div>
        </div>
    </div>

    <div class="table-card" style="margin-bottom: 24px;">
        <h2>Consolidated Multi-Pair Benchmark &mdash; Pure Latency &amp; Sequencing Mechanics (MEV = 0)</h2>
        <table>
            <thead>
                <tr>
                    <th>Architecture Regime</th>
                    <th>MM Total Vol</th>
                    <th>Markout 0ms</th>
                    <th>Markout 100ms</th>
                    <th>Markout 1s</th>
                    <th>s* Gross</th>
                    <th>s* Net</th>
                    <th>[1] Spread Paid</th>
                    <th>[2] Taker Stale</th>
                    <th>[3] Informed Wait</th>
                    <th>[=] All-In Net</th>
                    <th>StaleSnipe Hit/Miss</th>
                    <th>Dropped Takers</th>
                    <th>Batches</th>
                </tr>
            </thead>
            <tbody>
                {cons_rows_html}
            </tbody>
        </table>
        <p style="margin-top: 10px; font-size: 11.5px; color: var(--text-muted);">
            <strong>Identity (MEV = 0):</strong> <code>[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net</code>.
            <strong>Adverse selection check (Regime A, 1s markout):</strong> {sign_check_text}
        </p>
    </div>

    {placebo_html}

    {ratio_html}

    <div class="table-card" style="margin-bottom: 24px;">
        <h2>Consolidated MM Flow Cohort Decomposition &amp; Volume Reconciliation</h2>
        <table>
            <thead>
                <tr>
                    <th>Architecture Regime</th>
                    <th>Taker Vol</th>
                    <th>StaleSnipe Vol</th>
                    <th>LateSnipe Vol</th>
                    <th>Total MM Vol</th>
                    <th>Vol Diff</th>
                    <th>Taker 1s Markout</th>
                    <th>StaleSnipe 1s Markout</th>
                    <th>LateSnipe 1s Markout</th>
                </tr>
            </thead>
            <tbody>
                {cons_cohort_html}
            </tbody>
        </table>
    </div>

    {per_sym_sections_html}

    <h2 style="font-size: 18px; margin: 26px 0 12px 0; color: #ffffff;">Phase 2: Break-Even Frontier Across MEV Extraction Scenarios</h2>
    {frontier_cards_html}

    {calibration_html}
</div>

<script>
    Chart.defaults.color = '#94a3b8';
    Chart.defaults.font.family = "'Inter', -apple-system, sans-serif";
    const horizons = ['0 ms', '100 ms', '1000 ms'];

    function makeMarkoutChart(canvasId, dD, dB, dC, dA) {{
        new Chart(document.getElementById(canvasId), {{
            type: 'line',
            data: {{
                labels: horizons,
                datasets: [
                    {{
                        label: 'Regime D: IBTE + Cancel-Prio ({d_win}ms window, {d_dec}ms decrypt)',
                        data: dD,
                        borderColor: '#c28aff',
                        backgroundColor: 'rgba(194, 138, 255, 0.12)',
                        borderWidth: 3,
                        pointRadius: 4.5,
                        tension: 0.2
                    }},
                    {{
                        label: 'Regime B: Cancel-Priority ({b_win}ms)',
                        data: dB,
                        borderColor: '#38bdf8',
                        borderWidth: 2,
                        borderDash: [5, 5],
                        pointRadius: 3.5,
                        tension: 0.2
                    }},
                    {{
                        label: 'Regime C: Uniform-Price FBA ({c_win}ms)',
                        data: dC,
                        borderColor: '#f59e0b',
                        borderWidth: 2,
                        pointRadius: 3.5,
                        tension: 0.2
                    }},
                    {{
                        label: 'Regime A: Continuous FIFO',
                        data: dA,
                        borderColor: '#f43f5e',
                        borderWidth: 2,
                        pointRadius: 3.5,
                        tension: 0.2
                    }}
                ]
            }},
            options: {{
                responsive: true,
                maintainAspectRatio: false,
                plugins: {{
                    legend: {{ position: 'bottom', labels: {{ boxWidth: 12, padding: 14, font: {{ size: 11.5 }} }} }}
                }},
                scales: {{
                    y: {{
                        grid: {{ color: '#1e293b' }},
                        title: {{ display: true, text: 'Signed MM Markout (bps)' }}
                    }},
                    x: {{ grid: {{ color: '#1e293b' }} }}
                }}
            }}
        }});
    }}

    makeMarkoutChart('chartSym1', [{btc_d}], [{btc_b}], [{btc_c}], [{btc_a}]);
    makeMarkoutChart('chartSym2', [{eth_d}], [{eth_b}], [{eth_c}], [{eth_a}]);
</script>
</body>
</html>
"#,
        syms = html_escape(&args.symbols.join(", ")),
        start = html_escape(&args.start),
        days = args.days,
        nblocks = nblocks,
        bh = args.block_hours,
        days_ok = days_ok,
        total_tasks = total_tasks,
        pending_pol = args.pending_policy.as_str(),
        mid_mode = args.mid_mode.as_str(),
        data_source_badge = data_source_badge,
        recon_banner = recon_banner,
        d35_snet = d35_snet,
        a_snet = a_snet,
        b_snet = b_snet,
        d35_m1s = d35_m1s,
        a_m1s = a_m1s,
        zero_mev_cost = zero_mev_cost,
        mid_scenario_delta = mid_scenario_delta,
        d_dec = d_dec,
        b_win = b_win,
        c_win = c_win,
        d_win = d_win,
        sym1_name = sym1_name,
        sym2_name = sym2_name,
        cons_rows_html = cons_rows_html,
        cons_cohort_html = cons_cohort_html,
        sign_check_text = html_escape(sign_check_text),
        placebo_html = placebo_html,
        ratio_html = ratio_html,
        per_sym_sections_html = per_sym_sections_html,
        frontier_cards_html = frontier_cards_html,
        calibration_html = calibration_html,
        btc_a = btc_a,
        btc_b = btc_b,
        btc_c = btc_c,
        btc_d = btc_d,
        eth_a = eth_a,
        eth_b = eth_b,
        eth_c = eth_c,
        eth_d = eth_d,
    );

    w.write_all(html.as_bytes()).map_err(|e| e.to_string())?;
    w.flush().map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================================
// Main Entrypoint
// ============================================================================

fn main() {
    let args = parse_args();

    if !args.skip_preflight {
        println!("=== PREFLIGHT (Synthetic Invariant Suite) ===");
        if !selfcheck::run_all() {
            eprintln!("Pre-flight invariant check failed — aborting market replay.");
            std::process::exit(2);
        }
    }
    if args.preflight_only {
        return;
    }

    let start_days = {
        let p: Vec<i64> = args
            .start
            .split('-')
            .filter_map(|x| x.parse().ok())
            .collect();
        if p.len() != 3 {
            eprintln!("--start must be formatted as YYYY-MM-DD");
            std::process::exit(2);
        }
        civil_to_days(p[0], p[1], p[2])
    };
    let nblocks = (((args.days as i64) * 24 + args.block_hours - 1) / args.block_hours) as usize;
    let specs = build_specs(&args);
    let mut rep = String::new();

    emit!(rep, "# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.1)");
    emit!(
        rep,
        "Window: {} + {}d | Pairs: {} | Bootstrap Block: {}h ({} blocks) | Runs per (Pair, Day): {} | B window {}ms, D window {}ms, FBA window {}ms | pending={} | mid={} | placebo shifts (s): {:?}",
        args.start,
        args.days,
        args.symbols.join(","),
        args.block_hours,
        nblocks,
        specs.len(),
        args.b_window(),
        args.window_ms,
        args.fba_window_ms,
        args.pending_policy.as_str(),
        args.mid_mode.as_str(),
        args.placebo_shifts.iter().map(|s| s / 1000).collect::<Vec<i64>>()
    );

    let mut tasks: Vec<(usize, usize)> = Vec::new();
    for s in 0..args.symbols.len() {
        for d in 0..args.days {
            tasks.push((s, d));
        }
    }
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<Option<DayResult>>> =
        Mutex::new((0..tasks.len()).map(|_| None).collect());
    let t_all = Instant::now();
    std::thread::scope(|sc| {
        for _ in 0..args.threads {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= tasks.len() {
                    break;
                }
                let (s, d) = tasks[i];
                let r = run_day(s, d, &args, &specs, start_days, nblocks);
                if !r.line.is_empty() {
                    println!("{}", r.line);
                }
                if let Ok(mut g) = results.lock() {
                    g[i] = Some(r);
                }
            });
        }
    });
    println!("Phase 1 completed in {:.1?}", t_all.elapsed());

    let nsym = args.symbols.len();
    let mut per_sym: Vec<Vec<Vec<Agg>>> =
        vec![vec![vec![Agg::default(); nblocks]; specs.len()]; nsym];
    let mut skipped: Vec<String> = Vec::new();
    let mut days_ok = 0usize;
    let mut days_reconstructed = 0usize;
    let mut total_mid_ticks = 0usize;
    let mut total_taker_orders = 0usize;

    let res_vec = results.into_inner().unwrap_or_default();
    for (i, slot) in res_vec.into_iter().enumerate() {
        let (s, _d) = tasks[i];
        match slot {
            None => skipped.push("internal worker error".to_string()),
            Some(r) => {
                if let Some(why) = r.skipped {
                    skipped.push(why);
                } else {
                    days_ok += 1;
                    total_mid_ticks += r.mid_ticks;
                    total_taker_orders += r.taker_orders;
                    if r.reconstructed {
                        days_reconstructed += 1;
                    }
                    for (si, blocks) in r.aggs.iter().enumerate() {
                        for (b, a) in blocks.iter().enumerate() {
                            per_sym[s][si][b].merge(a);
                        }
                    }
                }
            }
        }
    }
    if days_ok == 0 {
        eprintln!("No valid pair-days loaded. Skipped:");
        for s in &skipped {
            eprintln!("  {}", s);
        }
        std::process::exit(1);
    }
    emit!(rep, "\nPair-Days Processed: {} of {}.", days_ok, tasks.len());
    if !skipped.is_empty() {
        emit!(rep, "Skipped:");
        for s in &skipped {
            emit!(rep, "- {}", s);
        }
    }

    let mut combined: Vec<Vec<Agg>> = vec![vec![Agg::default(); nblocks]; specs.len()];
    for s in 0..nsym {
        for si in 0..specs.len() {
            for b in 0..nblocks {
                let a = per_sym[s][si][b];
                combined[si][b].merge(&a);
            }
        }
    }
    let full: Vec<Agg> = combined.iter().map(|c| total(c)).collect();

    for s in 0..nsym {
        let tot: Vec<Agg> = per_sym[s].iter().map(|c| total(c)).collect();
        table(&mut rep, &format!("{} (MEV = 0)", args.symbols[s]), &specs, &tot, 0);
    }
    table(&mut rep, "Consolidated Multi-Pair (MEV = 0)", &specs, &full, 0);
    cohort_table(&mut rep, "Consolidated Multi-Pair", &specs, &full);

    if !args.placebo_shifts.is_empty() {
        emit!(rep, "\n## Placebo test: circularly shifted taker flow");
        for &s in &args.placebo_shifts {
            table(
                &mut rep,
                &format!("Placebo: takers shifted {}s (consolidated, MEV = 0)", s / 1000),
                &specs,
                &full,
                s,
            );
        }
        emit!(rep, "\n### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)");
        for l in placebo_lines(&specs, &full, &args) {
            emit!(rep, "- {}", l);
        }
    }

    let ratio_lines = gap_vs_ratio_lines(&specs, &full, &args);
    if !ratio_lines.is_empty() {
        emit!(rep, "\n### Cancel-priority gap versus reaction handicap / window");
        for l in &ratio_lines {
            emit!(rep, "- {}", l);
        }
    }

    let boot = bootstrap_sums(&combined, args.bootstrap, 0xB007_5712);
    let mut sign_check_summary = String::from("Insufficient bootstrap samples.");
    if let Some(ia) = find_spec(&specs, Regime::Fifo, args.mm_react[0], 0, 0) {
        let mut samples: Vec<i128> = Vec::new();
        if !boot[ia].is_empty() {
            for r in boot[ia].iter() {
                if r.mm_n > 0 {
                    samples.push(r.mm_w[2] * Q / r.mm_n);
                }
            }
        }
        let point = if full[ia].mm_n > 0 {
            full[ia].mm_w[2] * Q / full[ia].mm_n
        } else {
            0
        };
        match ci95(samples) {
            Some((lo, hi)) => {
                let flag = if hi >= 0 {
                    " RED FLAG: 95% CI includes 0 or is positive — adverse selection not confirmed."
                } else {
                    " Expected negative sign confirmed."
                };
                sign_check_summary = format!(
                    "{} bps, 95% CI [{}, {}].{}",
                    fmt_bps(point),
                    fmt_bps(lo),
                    fmt_bps(hi),
                    flag
                );
                emit!(
                    rep,
                    "\n**Empirical Data Verification (Regime A, 1s Markout):** {}",
                    sign_check_summary
                );
            }
            None => emit!(rep, "\nSign check: insufficient bootstrap samples."),
        }
    }

    emit!(rep, "\n## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)");
    let mut csv = String::new();
    let html_grids = frontier(&mut rep, &mut csv, &specs, &full, &boot, &args);

    emit!(rep, "\n## Calibration context (public documentation, retrieved 2026-11-18; NOT simulation inputs)");
    for (topic, statement, source) in CALIBRATION_NOTES.iter() {
        emit!(rep, "- **{}:** {} _Source: {}_", topic, statement, source);
    }

    let out_dir = Path::new(&args.out_dir);
    let _ = std::fs::create_dir_all(out_dir);
    if let Err(e) = std::fs::write(out_dir.join("report.md"), &rep) {
        eprintln!("Failed to write report.md: {}", e);
    }
    if let Err(e) = std::fs::write(out_dir.join("breakeven.csv"), &csv) {
        eprintln!("Failed to write breakeven.csv: {}", e);
    }
    let html_path = out_dir.join("ablation_report.html");
    if let Err(e) = generate_html_report(
        &html_path,
        &args,
        days_ok,
        tasks.len(),
        days_reconstructed,
        total_mid_ticks,
        total_taker_orders,
        nblocks,
        &specs,
        &per_sym,
        &full,
        &html_grids,
        &sign_check_summary,
    ) {
        eprintln!("Failed to write ablation_report.html: {}", e);
    }

    println!(
        "\nOutputs written: {}/report.md, {}/breakeven.csv, {}/ablation_report.html",
        args.out_dir, args.out_dir, args.out_dir
    );
}