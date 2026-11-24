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

const VERSION: &str = "v0.3.5";
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
enum BatchOrder {
    /// Class 0: Cancel, Class 1: New / Taker / Sniper (by arrival time).
    CancelFirst,
    /// Post-only first: Class 0: New (ALO/post-only), Class 1: Cancel, Class 2: Taker / Sniper.
    Hl,
}

impl BatchOrder {
    fn as_str(self) -> &'static str {
        match self {
            BatchOrder::CancelFirst => "cancel-first",
            BatchOrder::Hl => "hl",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExposureMode {
    /// Apply leak_ppm uniformly to all large orders (consistent with full payload encryption & uniform padding).
    All,
    /// Legacy v0.3.4 mode: apply leak_ppm only to large orders inside batches containing an MM cancel.
    CancelBatch,
}

impl ExposureMode {
    fn as_str(self) -> &'static str {
        match self {
            ExposureMode::All => "all",
            ExposureMode::CancelBatch => "cancel-batch",
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MidSource {
    Auto,
    BookTicker,
    Proxy,
}

impl MidSource {
    fn as_str(self) -> &'static str {
        match self {
            MidSource::Auto => "auto",
            MidSource::BookTicker => "book",
            MidSource::Proxy => "proxy",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WaitCohort {
    All,
    Large,
}

impl WaitCohort {
    fn as_str(self) -> &'static str {
        match self {
            WaitCohort::All => "all",
            WaitCohort::Large => "large",
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct ProxyDiag {
    total_obs: usize,
    crossed_obs: usize,
    valid_spread_obs: usize,
    spread_sum_sub: i128,
}

impl ProxyDiag {
    fn merge(&mut self, o: &ProxyDiag) {
        self.total_obs += o.total_obs;
        self.crossed_obs += o.crossed_obs;
        self.valid_spread_obs += o.valid_spread_obs;
        self.spread_sum_sub += o.spread_sum_sub;
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct RawTakerDiag {
    raw_rows: usize,
    raw_vol_usd: i128,
    raw_large_rows: usize,
    raw_large_vol_usd: i128,
    merged_bursts: usize,
    merged_vol_usd: i128,
    merged_large_bursts: usize,
    merged_large_vol_usd: i128,
}

impl RawTakerDiag {
    fn merge(&mut self, o: &RawTakerDiag) {
        self.raw_rows += o.raw_rows;
        self.raw_vol_usd += o.raw_vol_usd;
        self.raw_large_rows += o.raw_large_rows;
        self.raw_large_vol_usd += o.raw_large_vol_usd;
        self.merged_bursts += o.merged_bursts;
        self.merged_vol_usd += o.merged_vol_usd;
        self.merged_large_bursts += o.merged_large_bursts;
        self.merged_large_vol_usd += o.merged_large_vol_usd;
    }

    fn finalize_merged(&mut self, takers: &[Taker], large_usd: i64) {
        self.merged_bursts = takers.len();
        self.merged_vol_usd = 0;
        self.merged_large_bursts = 0;
        self.merged_large_vol_usd = 0;
        for t in takers {
            let n = t.notional_usd.max(1) as i128;
            self.merged_vol_usd += n;
            if t.notional_usd >= large_usd {
                self.merged_large_bursts += 1;
                self.merged_large_vol_usd += n;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct BookTickerDiag {
    files_loaded: usize,
    raw_rows: usize,
    same_ms_collapsed: usize,
    unchanged_mid_skipped: usize,
    distinct_mids: usize,
    dt_sum_ms: i128,
    dt_intervals: usize,
    dt_hist: Vec<usize>,
}

impl Default for BookTickerDiag {
    fn default() -> Self {
        Self {
            files_loaded: 0,
            raw_rows: 0,
            same_ms_collapsed: 0,
            unchanged_mid_skipped: 0,
            distinct_mids: 0,
            dt_sum_ms: 0,
            dt_intervals: 0,
            dt_hist: vec![0usize; 10_001],
        }
    }
}

impl BookTickerDiag {
    fn record_intervals(&mut self, mids: &[Mid]) {
        for w in mids.windows(2) {
            let dt = (w[1].ts_ms - w[0].ts_ms).max(0);
            self.dt_sum_ms += dt as i128;
            self.dt_intervals += 1;
            let bin = (dt as usize).min(10_000);
            self.dt_hist[bin] += 1;
        }
    }

    fn merge(&mut self, o: &BookTickerDiag) {
        self.files_loaded += o.files_loaded;
        self.raw_rows += o.raw_rows;
        self.same_ms_collapsed += o.same_ms_collapsed;
        self.unchanged_mid_skipped += o.unchanged_mid_skipped;
        self.distinct_mids += o.distinct_mids;
        self.dt_sum_ms += o.dt_sum_ms;
        self.dt_intervals += o.dt_intervals;
        if self.dt_hist.len() == o.dt_hist.len() {
            for (a, b) in self.dt_hist.iter_mut().zip(o.dt_hist.iter()) {
                *a += *b;
            }
        }
    }

    fn median_dt_ms(&self) -> Option<usize> {
        if self.dt_intervals == 0 {
            return None;
        }
        let target = self.dt_intervals / 2;
        let mut cum = 0usize;
        for (ms, &cnt) in self.dt_hist.iter().enumerate() {
            cum += cnt;
            if cum > target {
                return Some(ms);
            }
        }
        Some(10_000)
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
    batch_order: BatchOrder,
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
        batch_order: BatchOrder::Hl,
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

    fn dropped_usd_pm(&self) -> i128 {
        let tot = self.tk_n + self.tk_dropped_usd;
        if tot > 0 {
            self.tk_dropped_usd * 1000 / tot
        } else {
            0
        }
    }

    /// Dropped taker USD share in parts per 100,000 (1 = 0.001%, 100_000 = 100.000%).
    fn dropped_usd_p100k(&self) -> i128 {
        let tot = self.tk_n + self.tk_dropped_usd;
        if tot > 0 {
            self.tk_dropped_usd * 100_000 / tot
        } else {
            0
        }
    }

    fn cancel_batch_exp_ppm(&self) -> i128 {
        if self.tk_large_n > 0 {
            self.tk_large_exp_n * 1_000_000 / self.tk_large_n
        } else {
            0
        }
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
            let jitter = rng.range_i64(0, 999_999);
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
        let jitter = rng.range_i64(0, 999_999);
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

    fn bump_late_fresh(&mut self, ms: i64) {
        let b = self.block_of(ms);
        self.agg[b].sniper_late_fresh += 1;
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
        } else if self.cfg.batch_order == BatchOrder::CancelFirst {
            self.tomb.insert(target);
        }
    }

    fn apply_new(&mut self, id: u64, reff: i64) -> bool {
        if self.cfg.batch_order == BatchOrder::CancelFirst && self.tomb.remove(&id) {
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
        match self.cfg.batch_order {
            BatchOrder::CancelFirst => {
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
            }
            BatchOrder::Hl => {
                buf.sort_by_key(|e| {
                    let cls = match e.kind {
                        Kind::New { .. } => 0u8,
                        Kind::Cancel { .. } => 1u8,
                        Kind::Taker { .. } | Kind::Sniper { .. } => 2u8,
                    };
                    (cls, e.arr_ns)
                });
            }
        }
        for ev in buf.iter() {
            self.handle_event(ev, clear_ms, exposed);
        }
        self.bump_batch(clear_ms);
    }

    fn process_fba(&mut self, buf: &mut Vec<Ev>, clear_ns: i64) {
        let clear_ms = clear_ns / NS_PER_MS;
        for ev in buf.iter() {
            if let Kind::New { id, reff } = ev.kind {
                self.apply_new(id, reff);
            }
        }
        for ev in buf.iter() {
            if let Kind::Cancel { target } = ev.kind {
                self.apply_cancel(target);
            }
        }
        let mut orders: Vec<TakerOrd> = std::mem::take(&mut self.pending);
        let mut snipers: Vec<(i8, i64, u64)> = Vec::new();
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
                Kind::Sniper {
                    dir,
                    notional,
                    target_id,
                    ..
                } => snipers.push((dir, notional, target_id)),
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
        for &(d, n, _tid) in snipers.iter() {
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
        let current_id = self.proc.id;
        for &(_, _, tid) in snipers.iter() {
            if current_id == tid {
                self.bump_hit(clear_ms);
            } else {
                self.bump_late_fresh(clear_ms);
            }
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
    wait_cohort: WaitCohort,
    d_exposure: ExposureMode,
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
    let eff_leak = e.leak_ppm.min(e.capture_ppm);
    let (p, large_mm, large_tk) = if regime == Regime::IbteCancelPrio {
        match e.d_exposure {
            ExposureMode::All => (eff_leak, a.mm_large_n, a.tk_large_n),
            ExposureMode::CancelBatch => (eff_leak, a.mm_large_exp_n, a.tk_large_exp_n),
        }
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
    let wait_q = match e.wait_cohort {
        WaitCohort::All => a.tk_wait_all_w * Q / a.tk_n,
        WaitCohort::Large => {
            if a.tk_inf_n > 0 {
                a.tk_inf_wait_w * Q / a.tk_inf_n
            } else {
                0
            }
        }
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

/// Computes the exact break-even effective leakage rate `eff_leak_ppm` (where All-In(D) == All-In(B))
/// given an assumed fraction `exp_ppm` of Regime D large orders exposed to side-channel leakage.
/// Returns `None` if impact == 0 or denominator == 0.
fn breakeven_leak_ppm(
    b: &Agg,
    d: &Agg,
    e0: &Econ,
    cap_ppm: i128,
    imp_sub: i128,
    mm_share_ppm: i128,
    exp_ppm: i128,
) -> Option<i128> {
    if imp_sub <= 0 || exp_ppm <= 0 || b.mm_n <= 0 || b.tk_n <= 0 || d.mm_n <= 0 || d.tk_n <= 0 {
        return None;
    }
    let ob0 = evaluate(b, Regime::CancelPrio, e0)?;
    let od0 = evaluate(d, Regime::IbteCancelPrio, e0)?;
    let delta0_q = od0.all_in_net? - ob0.all_in_net?;

    let den: i128 = 1_000_000_000_000;
    let b_mm_q = cap_ppm * imp_sub * mm_share_ppm * Q * b.mm_large_n / (den * b.mm_n);
    let b_tk_q = cap_ppm * imp_sub * (1_000_000 - mm_share_ppm) * Q * b.tk_large_n / (den * b.tk_n);
    let mev_b_q = b_mm_q + b_tk_q;

    // Unit MEV in D at eff_leak = 1_000_000 (100%) scaled by exp_ppm / 1_000_000
    let d_mm_unit_q = exp_ppm * imp_sub * mm_share_ppm * Q * d.mm_large_n / (den * d.mm_n);
    let d_tk_unit_q =
        exp_ppm * imp_sub * (1_000_000 - mm_share_ppm) * Q * d.tk_large_n / (den * d.tk_n);
    let unit_d_q = d_mm_unit_q + d_tk_unit_q;
    if unit_d_q <= 0 {
        return None;
    }
    Some((mev_b_q - delta0_q) * 1_000_000 / unit_d_q)
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

/// Formats parts-per-100,000 as `X.YYY%` (0.001% resolution).
fn fmt_pct3(p100k: i128) -> String {
    let sign = if p100k < 0 { "-" } else { "" };
    let a = p100k.abs();
    format!("{}{}.{:03}%", sign, a / 1000, a % 1000)
}

/// Formats parts-per-million as `XX.X%`.
fn fmt_ppm_pct1(ppm: i128) -> String {
    fmt_permille(ppm / 1_000)
}

fn fmt_dropped_cell(a: &Agg) -> String {
    format!("{} ({})", a.tk_dropped, fmt_pct3(a.dropped_usd_p100k()))
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

enum PushMidEvent {
    Added,
    SameMsCollapsed,
    UnchangedMidSkipped,
}

fn push_mid_tracked(v: &mut Vec<Mid>, ts: i64, mid: i64) -> PushMidEvent {
    if let Some(l) = v.last_mut() {
        if l.ts_ms == ts {
            l.mid = mid;
            return PushMidEvent::SameMsCollapsed;
        }
        if l.mid == mid {
            return PushMidEvent::UnchangedMidSkipped;
        }
    }
    v.push(Mid { ts_ms: ts, mid });
    PushMidEvent::Added
}

fn push_mid(v: &mut Vec<Mid>, ts: i64, mid: i64) {
    let _ = push_mid_tracked(v, ts, mid);
}

fn notional_usd(price: i64, qty: i64) -> i64 {
    ((price as i128 * qty as i128) / (PRICE_SCALE as i128 * QTY_SCALE as i128)) as i64
}

fn load_book_ticker(path: &Path) -> Result<(Vec<Mid>, BookTickerDiag), String> {
    let file = File::open(path).map_err(|e| format!("open {}: {}", path.display(), e))?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut out: Vec<Mid> = Vec::with_capacity(4_000_000);
    let mut diag = BookTickerDiag {
        files_loaded: 1,
        ..BookTickerDiag::default()
    };
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
                diag.raw_rows += 1;
                match push_mid_tracked(&mut out, ts, (b + a) / 2) {
                    PushMidEvent::Added => {}
                    PushMidEvent::SameMsCollapsed => diag.same_ms_collapsed += 1,
                    PushMidEvent::UnchangedMidSkipped => diag.unchanged_mid_skipped += 1,
                }
            }
        }
    }
    out.sort_by_key(|m| m.ts_ms);
    diag.distinct_mids = out.len();
    diag.record_intervals(&out);
    Ok((out, diag))
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

fn load_agg_trades_csv(path: &Path, large_usd: i64) -> Result<(Vec<Taker>, RawTakerDiag), String> {
    let file = File::open(path).map_err(|e| format!("open {}: {}", path.display(), e))?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut out: Vec<Taker> = Vec::with_capacity(1_200_000);
    let mut tdiag = RawTakerDiag::default();
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
        let notional = notional_usd(price, qty);
        let n_eff = notional.max(1) as i128;
        tdiag.raw_rows += 1;
        tdiag.raw_vol_usd += n_eff;
        if notional >= large_usd {
            tdiag.raw_large_rows += 1;
            tdiag.raw_large_vol_usd += n_eff;
        }
        merge_taker(&mut out, ts, dir, notional);
    }
    out.sort_by_key(|t| t.ts_ms);
    tdiag.finalize_merged(&out, large_usd);
    Ok((out, tdiag))
}

fn reconstruct_bidask_step(
    last_bid: &mut Option<i64>,
    last_ask: &mut Option<i64>,
    price: i64,
    ibm: bool,
    diag: &mut ProxyDiag,
) -> Option<i64> {
    if ibm {
        *last_bid = Some(price);
    } else {
        *last_ask = Some(price);
    }
    if let (Some(b), Some(a)) = (*last_bid, *last_ask) {
        diag.total_obs += 1;
        if a >= b {
            diag.valid_spread_obs += 1;
            diag.spread_sum_sub += rel_sub(a, b) as i128;
            Some((b + a) / 2)
        } else {
            diag.crossed_obs += 1;
            Some(price)
        }
    } else {
        None
    }
}

fn load_trades_csv(
    path: &Path,
    half_tick: i64,
    mode: MidMode,
    large_usd: i64,
) -> Result<(Vec<Taker>, Vec<Mid>, ProxyDiag, RawTakerDiag), String> {
    let file = File::open(path).map_err(|e| format!("open {}: {}", path.display(), e))?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut takers: Vec<Taker> = Vec::with_capacity(2_000_000);
    let mut mids: Vec<Mid> = Vec::with_capacity(2_000_000);
    let mut last_bid: Option<i64> = None;
    let mut last_ask: Option<i64> = None;
    let mut diag = ProxyDiag::default();
    let mut tdiag = RawTakerDiag::default();

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
                if let Some(mid) =
                    reconstruct_bidask_step(&mut last_bid, &mut last_ask, price, ibm, &mut diag)
                {
                    push_mid(&mut mids, ts, mid);
                }
            }
        }
        let notional = notional_usd(price, qty);
        let n_eff = notional.max(1) as i128;
        tdiag.raw_rows += 1;
        tdiag.raw_vol_usd += n_eff;
        if notional >= large_usd {
            tdiag.raw_large_rows += 1;
            tdiag.raw_large_vol_usd += n_eff;
        }
        merge_taker(&mut takers, ts, dir, notional);
    }
    takers.sort_by_key(|t| t.ts_ms);
    mids.sort_by_key(|m| m.ts_ms);
    tdiag.finalize_merged(&takers, large_usd);
    Ok((takers, mids, diag, tdiag))
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
    b_batch_order: BatchOrder,
    d_batch_order: BatchOrder,
    d_exposure_mode: ExposureMode,
    wait_cohort: WaitCohort,
    mid_source: MidSource,
    mid_mode: MidMode,
    rebate_sub: i128,
    opex_sub: i128,
    large_usd: i64,
    sniper_clip_usd: i64,
    requote_thr_sub: i64,
    sniper_thr_sub: i64,
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

fn parse_batch_order(val: &str, flag: &str) -> BatchOrder {
    match val.trim().to_ascii_lowercase().as_str() {
        "hl" => BatchOrder::Hl,
        "cancel-first" => BatchOrder::CancelFirst,
        _ => {
            eprintln!("{} must be 'hl' or 'cancel-first'", flag);
            std::process::exit(2);
        }
    }
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
        b_batch_order: BatchOrder::Hl,
        d_batch_order: BatchOrder::Hl,
        d_exposure_mode: ExposureMode::All,
        wait_cohort: WaitCohort::All,
        mid_source: MidSource::Auto,
        mid_mode: MidMode::BidAskProxy,
        rebate_sub: 50,
        opex_sub: 30,
        large_usd: 50_000,
        sniper_clip_usd: 25_000,
        requote_thr_sub: 150,
        sniper_thr_sub: 200,
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
                    "--b-batch-order" | "--batch-order" => {
                        a.b_batch_order = parse_batch_order(&val, "--b-batch-order");
                    }
                    "--d-batch-order" => {
                        a.d_batch_order = parse_batch_order(&val, "--d-batch-order");
                    }
                    "--d-exposure-mode" => match val.trim().to_ascii_lowercase().as_str() {
                        "all" => a.d_exposure_mode = ExposureMode::All,
                        "cancel-batch" => a.d_exposure_mode = ExposureMode::CancelBatch,
                        _ => {
                            eprintln!("--d-exposure-mode must be 'all' or 'cancel-batch'");
                            std::process::exit(2);
                        }
                    },
                    "--wait-cohort" => match val.trim().to_ascii_lowercase().as_str() {
                        "all" => a.wait_cohort = WaitCohort::All,
                        "large" => a.wait_cohort = WaitCohort::Large,
                        _ => {
                            eprintln!("--wait-cohort must be 'all' or 'large'");
                            std::process::exit(2);
                        }
                    },
                    "--mid-source" => match val.trim().to_ascii_lowercase().as_str() {
                        "auto" => a.mid_source = MidSource::Auto,
                        "book" | "bookticker" => a.mid_source = MidSource::BookTicker,
                        "proxy" => {
                            a.mid_source = MidSource::Proxy;
                            a.allow_reconstructed = true;
                        }
                        _ => {
                            eprintln!("--mid-source must be 'auto', 'book', or 'proxy'");
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
                    "--rebate-sub" => a.rebate_sub = val.parse().unwrap_or(50),
                    "--opex-sub" => a.opex_sub = val.parse().unwrap_or(30),
                    "--large-usd" => a.large_usd = val.parse().unwrap_or(50_000),
                    "--sniper-clip" => {
                        a.sniper_clip_usd = val.parse::<i64>().unwrap_or(25_000).max(0)
                    }
                    "--requote-thr-sub" => {
                        a.requote_thr_sub = val.parse::<i64>().unwrap_or(150).max(1)
                    }
                    "--sniper-thr-sub" => {
                        a.sniper_thr_sub = val.parse::<i64>().unwrap_or(200).max(1)
                    }
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
    a.large_usd = a.large_usd.max(1);
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
        format!(", PLACEBO shift {}s", shift / 1000)
    };
    let mut base = default_cfg();
    base.mm_react_ms = r;
    base.pending_policy = args.pending_policy;
    base.large_usd = args.large_usd;
    base.sniper_clip_usd = args.sniper_clip_usd;
    base.requote_thr_sub = args.requote_thr_sub;
    base.sniper_thr_sub = args.sniper_thr_sub;
    let bw = args.b_window();

    let mut a = base;
    a.window_ms = args.window_ms;
    v.push(Spec {
        regime: Regime::Fifo,
        react: r,
        decrypt: 0,
        shift,
        cfg: a,
        label: format!("A: Continuous FIFO, react {}ms{}", r, suffix),
    });

    let mut b = base;
    b.window_ms = bw;
    b.batch_order = args.b_batch_order;
    v.push(Spec {
        regime: Regime::CancelPrio,
        react: r,
        decrypt: 0,
        shift,
        cfg: b,
        label: format!(
            "B: Batch {}ms ({}), react {}ms{}",
            bw,
            args.b_batch_order.as_str(),
            r,
            suffix
        ),
    });

    let mut c = base;
    c.window_ms = args.fba_window_ms;
    c.batch_order = BatchOrder::Hl;
    v.push(Spec {
        regime: Regime::UniformFba,
        react: r,
        decrypt: 0,
        shift,
        cfg: c,
        label: format!(
            "C: Uniform-Price FBA {}ms, react {}ms{}",
            args.fba_window_ms, r, suffix
        ),
    });

    for &d in &args.decrypt {
        let mut dd = base;
        dd.window_ms = args.window_ms;
        dd.decrypt_ms = d;
        dd.batch_order = args.d_batch_order;
        v.push(Spec {
            regime: Regime::IbteCancelPrio,
            react: r,
            decrypt: d,
            shift,
            cfg: dd,
            label: format!(
                "D: Dywe IBTE {}ms (dec {}ms, {}), react {}ms{}",
                args.window_ms,
                d,
                args.d_batch_order.as_str(),
                r,
                suffix
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
    proxy_diag: ProxyDiag,
    book_diag: BookTickerDiag,
    taker_diag: RawTakerDiag,
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
        proxy_diag: ProxyDiag::default(),
        book_diag: BookTickerDiag::default(),
        taker_diag: RawTakerDiag::default(),
    };
    let bt_s = format!("{}/{}-bookTicker-{}.csv", args.data_dir, sym, date);
    let tr_s = format!("{}/{}-trades-{}.csv", args.data_dir, sym, date);
    let ag_s = format!("{}/{}-aggTrades-{}.csv", args.data_dir, sym, date);
    let bt = Path::new(&bt_s);
    let tr = Path::new(&tr_s);
    let ag = Path::new(&ag_s);

    let mut trade_takers: Option<(Vec<Taker>, RawTakerDiag)> = None;
    let mut reconstructed = false;
    let mut proxy_diag = ProxyDiag::default();
    let mut book_diag = BookTickerDiag::default();

    let mids: Vec<Mid> = match args.mid_source {
        MidSource::BookTicker => {
            if bt.exists() {
                match load_book_ticker(bt) {
                    Ok((m, bd)) => {
                        book_diag = bd;
                        m
                    }
                    Err(e) => return skip(format!("{} {}: {}", sym, date, e)),
                }
            } else {
                return skip(format!("{} {}: missing bookTicker (--mid-source book)", sym, date));
            }
        }
        MidSource::Proxy => {
            if tr.exists() {
                match load_trades_csv(tr, half_tick_1e8(&sym), args.mid_mode, args.large_usd) {
                    Ok((t, m, diag, td)) => {
                        trade_takers = Some((t, td));
                        reconstructed = true;
                        proxy_diag = diag;
                        m
                    }
                    Err(e) => return skip(format!("{} {}: {}", sym, date, e)),
                }
            } else {
                return skip(format!("{} {}: missing trades CSV (--mid-source proxy)", sym, date));
            }
        }
        MidSource::Auto => {
            if bt.exists() {
                match load_book_ticker(bt) {
                    Ok((m, bd)) => {
                        book_diag = bd;
                        m
                    }
                    Err(e) => return skip(format!("{} {}: {}", sym, date, e)),
                }
            } else if args.allow_reconstructed && tr.exists() {
                match load_trades_csv(tr, half_tick_1e8(&sym), args.mid_mode, args.large_usd) {
                    Ok((t, m, diag, td)) => {
                        trade_takers = Some((t, td));
                        reconstructed = true;
                        proxy_diag = diag;
                        m
                    }
                    Err(e) => return skip(format!("{} {}: {}", sym, date, e)),
                }
            } else {
                return skip(format!(
                    "{} {}: missing bookTicker (and --allow-reconstructed not set)",
                    sym, date
                ));
            }
        }
    };

    let (takers, taker_diag): (Vec<Taker>, RawTakerDiag) = if ag.exists() {
        match load_agg_trades_csv(ag, args.large_usd) {
            Ok(x) => x,
            Err(e) => return skip(format!("{} {}: {}", sym, date, e)),
        }
    } else if let Some(x) = trade_takers {
        x
    } else if tr.exists() {
        match load_trades_csv(tr, 0, args.mid_mode, args.large_usd) {
            Ok((t, _, _, td)) => (t, td),
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
        format!(
            "bookTicker(raw={}, med_dt={}ms)",
            book_diag.raw_rows,
            book_diag.median_dt_ms().unwrap_or(0)
        )
    };
    let line = format!(
        "[OK] {:<8} {} | mid ticks {:>9} | bursts {:>8} (raw {:>9}) | source: {} | {:.1?}",
        sym,
        date,
        mids.len(),
        takers.len(),
        taker_diag.raw_rows,
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
        proxy_diag,
        book_diag,
        taker_diag,
    }
}

// ============================================================================
// Calibration & Design Notes
// ============================================================================

const EXTERNAL_CALIBRATION_NOTES: [(&str, &str, &str); 4] = [
    (
        "Hyperliquid: intra-batch priority order",
        "Official docs (hyperliquid-l1/order-book, accessed Nov 2026) state that within each consensus batch, non-GTC/IOC actions (i.e. post-only / ALO) are processed first, followed by cancels, followed by GTC/IOC orders. Model this with `--b-batch-order hl`.",
        "https://hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-l1/order-book",
    ),
    (
        "Hyperliquid: block interval & buckets",
        "Official docs (for-developers/api/priority-fees, accessed Nov 2026) state that IOC orders are sorted in 70 ms unix-time buckets, typically corresponding to one block (with 1-2 consensus batches per block). End-to-end co-located latency is cited around ~0.2 s median in third-party mirrors.",
        "https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/priority-fees ; https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview",
    ),
    (
        "dYdX Chain (v4): block time",
        "Official onboarding FAQ (accessed Nov 2026) states an average block time of ~1 second and best-effort short-term cancels until block finality. Use `--b-window-ms 1000` to evaluate a 1-second batching cadence.",
        "https://docs.dydx.xyz/concepts/onboarding-faqs",
    ),
    (
        "Not documented (parameterized assumptions)",
        "No venue documentation specifies MM reaction time, requote threshold, feed latency, client-to-sequencer transit, or sniper reaction time. Sweep `--mm-react` (e.g. 1,2,5,10,25,40) and `--requote-thr-sub` (e.g. 15,50,150) rather than trusting a single point.",
        "n/a",
    ),
];

const DYWE_DESIGN_NOTES: [(&str, &str); 3] = [
    (
        "Dywe micro-batch window & threshold decryption RTT",
        "The 50 ms micro-batch window and 35 ms / 70 ms threshold decryption RTTs are Dywe protocol design parameters, not live multi-region Devnet measurements. No BLS12-381 cryptographic operations are executed inside this microstructure simulator.",
    ),
    (
        "Unified post-decrypt threat model & exposure mode",
        "All order messages (New ALO, Cancel, Taker) are threshold-encrypted client-side (BLS12-381) with uniform frame padding until batch_close, decrypting in matching-engine memory at clear_ns = batch_close + decrypt_ms before New (ALO) -> Cancel -> Taker sorting (`--d-batch-order hl`). Under `--d-exposure-mode all` (default), `eff_leak = min(leak_ppm, capture_ppm)` applies uniformly across all large bursts; under `cancel-batch` (legacy v0.3.4), it applies only to batches containing an MM cancel.",
    ),
    (
        "Stylized model disclaimer",
        "Regimes B (70 ms / 200 ms / 1000 ms) are stylized batching models, not byte-for-byte reproductions of Hyperliquid or dYdX matching engines.",
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
        cb.batch_order = BatchOrder::CancelFirst;
        let mut cd = cb;
        cd.decrypt_ms = 0;
        let ob = run(&mids, &takers, Regime::CancelPrio, &cb);
        let od = run(&mids, &takers, Regime::IbteCancelPrio, &cd);
        if ob.agg != od.agg {
            return Err("identity: D(decrypt=0) != B(cancel-first) at Agg accumulator level".to_string());
        }
        let e = Econ {
            capture_ppm: 0,
            impact_sub: 200,
            mm_share_ppm: 500_000,
            leak_ppm: 0,
            rebate_sub: 50,
            opex_sub: 30,
            horizon: 2,
            wait_cohort: WaitCohort::All,
            d_exposure: ExposureMode::All,
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
                wait_cohort: WaitCohort::All,
                d_exposure: ExposureMode::All,
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
            wait_cohort: WaitCohort::All,
            d_exposure: ExposureMode::All,
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

    pub fn pending_drop_policy() -> Result<String, String> {
        let (mids, takers) = synth(61, 150_000);
        let mut cfg_hold = default_cfg();
        cfg_hold.window_ms = 200;
        cfg_hold.batch_order = BatchOrder::CancelFirst;
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

    pub fn hl_batch_order() -> Result<String, String> {
        let (mids, takers) = synth(73, 150_000);
        let mut cfg_hl = default_cfg();
        cfg_hl.window_ms = 200;
        cfg_hl.batch_order = BatchOrder::Hl;
        cfg_hl.pending_policy = PendingPolicy::Drop;

        let mut cfg_cf = cfg_hl;
        cfg_cf.batch_order = BatchOrder::CancelFirst;

        let a_hl = total(&run(&mids, &takers, Regime::CancelPrio, &cfg_hl).agg);
        let a_cf = total(&run(&mids, &takers, Regime::CancelPrio, &cfg_cf).agg);

        if a_cf.tk_dropped == 0 {
            return Err("hl_batch_order: expected cancel-first+drop to drop gap takers".to_string());
        }
        if a_hl.tk_dropped * 20 >= a_cf.tk_dropped {
            return Err(format!(
                "hl_batch_order: HL order dropped {} vs cancel-first {}",
                a_hl.tk_dropped, a_cf.tk_dropped
            ));
        }
        Ok(format!(
            "HL order (New->Cancel->Taker) dropped {} vs cancel-first {} under drop policy",
            a_hl.tk_dropped, a_cf.tk_dropped
        ))
    }

    pub fn d_post_decrypt_hl_order() -> Result<String, String> {
        let (mids, takers) = synth(79, 150_000);
        let mut cfg_b = default_cfg();
        cfg_b.window_ms = 50;
        cfg_b.batch_order = BatchOrder::Hl;
        cfg_b.pending_policy = PendingPolicy::Drop;

        let mut cfg_d_hl = cfg_b;
        cfg_d_hl.decrypt_ms = 0;

        let mut cfg_d_cf = cfg_d_hl;
        cfg_d_cf.batch_order = BatchOrder::CancelFirst;

        let ab = run(&mids, &takers, Regime::CancelPrio, &cfg_b);
        let ad_hl = run(&mids, &takers, Regime::IbteCancelPrio, &cfg_d_hl);
        let ad_cf = run(&mids, &takers, Regime::IbteCancelPrio, &cfg_d_cf);

        if ab.agg != ad_hl.agg {
            return Err("d_post_decrypt_hl: D(hl, dec=0) != B(hl) at Agg level".to_string());
        }
        let t_hl = total(&ad_hl.agg);
        let t_cf = total(&ad_cf.agg);
        if t_hl.tk_dropped * 10 >= t_cf.tk_dropped.max(1) {
            return Err(format!(
                "d_post_decrypt_hl: expected D(hl) drops ({}) << D(cancel-first) drops ({})",
                t_hl.tk_dropped, t_cf.tk_dropped
            ));
        }
        Ok(format!(
            "D(hl, dec=0)==B(hl) bit-for-bit; dropped takers reduced from {} (cancel-first) to {} (hl)",
            t_cf.tk_dropped, t_hl.tk_dropped
        ))
    }

    pub fn fba_sniper_attribution() -> Result<String, String> {
        let base_p = 100 * PRICE_SCALE;
        let jump_p = 100 * PRICE_SCALE + 30_000_000;
        let mids = vec![
            Mid { ts_ms: 0, mid: base_p },
            Mid { ts_ms: 10, mid: jump_p },
            Mid { ts_ms: 2000, mid: jump_p },
        ];
        let takers = vec![Taker {
            ts_ms: 15,
            dir: 1,
            notional_usd: 10_000,
        }];
        let mut cfg = default_cfg();
        cfg.window_ms = 100;
        cfg.batch_order = BatchOrder::Hl;
        let out = simulate(&mids, &takers, Regime::UniformFba, &cfg, 0, 60_000, 1);
        let a = total(&out.agg);
        if a.sniper_hit != 0 || a.sniper_late_fresh != 1 {
            return Err(format!(
                "fba_sniper_attribution: expected sniper_hit=0, late_fresh=1; got hit={}, late_fresh={}",
                a.sniper_hit, a.sniper_late_fresh
            ));
        }
        Ok("FBA sniper arriving after same-batch MM requote attributed to late_fresh (0 stale hits)".to_string())
    }

    pub fn wait_cohort_all_identity() -> Result<String, String> {
        let a = Agg {
            mm_n: 1_000_000,
            mm_w: [-10_000_000, -15_000_000, -20_000_000],
            tk_n: 1_000_000,
            tk_scoef_w: 1_000_000,
            tk_stale_w: -20_000_000,
            tk_wait_all_w: 50_000_000,
            tk_inf_n: 200_000,
            tk_inf_wait_w: 30_000_000,
            ..Agg::default()
        };
        let e_all = Econ {
            capture_ppm: 0,
            impact_sub: 0,
            mm_share_ppm: 500_000,
            leak_ppm: 0,
            rebate_sub: 50,
            opex_sub: 30,
            horizon: 2,
            wait_cohort: WaitCohort::All,
            d_exposure: ExposureMode::All,
        };
        let mut e_large = e_all;
        e_large.wait_cohort = WaitCohort::Large;

        let o_all = evaluate(&a, Regime::Fifo, &e_all).ok_or("o_all None")?;
        let o_large = evaluate(&a, Regime::Fifo, &e_large).ok_or("o_large None")?;

        if o_all.wait_q != 50_000 || o_large.wait_q != 150_000 {
            return Err(format!(
                "wait_cohort: expected all=50000q, large=150000q; got all={}, large={}",
                o_all.wait_q, o_large.wait_q
            ));
        }
        let sp = o_all.spread_paid_q.ok_or("sp None")?;
        let all_in = o_all.all_in_net.ok_or("all_in None")?;
        if all_in != sp + o_all.stale_q + o_all.wait_q + o_all.mev_tk_q {
            return Err("wait_cohort: exact additive identity failed under WaitCohort::All".to_string());
        }
        Ok("WaitCohort::All (single denominator tk_n) and WaitCohort::Large verified".to_string())
    }

    pub fn bidask_mid_proxy() -> Result<String, String> {
        let mut lb = None;
        let mut la = None;
        let mut diag = ProxyDiag::default();
        if reconstruct_bidask_step(&mut lb, &mut la, 100 * PRICE_SCALE, true, &mut diag).is_some() {
            return Err("bidask_proxy: emitted mid with only one side".to_string());
        }
        let m1 = reconstruct_bidask_step(&mut lb, &mut la, 102 * PRICE_SCALE, false, &mut diag);
        if m1 != Some(101 * PRICE_SCALE) {
            return Err(format!("bidask_proxy: expected 101, got {:?}", m1));
        }
        let m2 = reconstruct_bidask_step(&mut lb, &mut la, 104 * PRICE_SCALE, true, &mut diag);
        if m2 != Some(104 * PRICE_SCALE) || diag.crossed_obs != 1 || diag.valid_spread_obs != 1 {
            return Err(format!(
                "bidask_proxy: crossed fallback failed (m2={:?}, diag={:?})",
                m2, diag
            ));
        }
        Ok("normal midpoint and crossed-side fallback + diagnostics verified".to_string())
    }

    pub fn book_ticker_diag() -> Result<String, String> {
        let mut v: Vec<Mid> = Vec::new();
        let mut d = BookTickerDiag::default();
        let ticks = [
            (10i64, 100 * PRICE_SCALE),
            (10i64, 101 * PRICE_SCALE),
            (11i64, 101 * PRICE_SCALE),
            (12i64, 102 * PRICE_SCALE),
            (16i64, 103 * PRICE_SCALE),
        ];
        for (ts, mid) in ticks {
            d.raw_rows += 1;
            match push_mid_tracked(&mut v, ts, mid) {
                PushMidEvent::Added => {}
                PushMidEvent::SameMsCollapsed => d.same_ms_collapsed += 1,
                PushMidEvent::UnchangedMidSkipped => d.unchanged_mid_skipped += 1,
            }
        }
        d.distinct_mids = v.len();
        d.record_intervals(&v);
        if d.raw_rows != 5
            || d.same_ms_collapsed != 1
            || d.unchanged_mid_skipped != 1
            || d.distinct_mids != 3
            || d.median_dt_ms() != Some(4)
        {
            return Err(format!("book_ticker_diag mismatch: {:?}", d));
        }
        Ok("raw_rows, deduplication counters, and median_dt_ms verified".to_string())
    }

    pub fn zero_capture_zero_mev() -> Result<String, String> {
        let a = Agg {
            mm_n: 1_000_000,
            tk_n: 1_000_000,
            mm_large_n: 700_000,
            tk_large_n: 700_000,
            mm_large_exp_n: 500_000,
            tk_large_exp_n: 500_000,
            ..Agg::default()
        };
        let e = Econ {
            capture_ppm: 0,
            impact_sub: 300,
            mm_share_ppm: 500_000,
            leak_ppm: 500_000,
            rebate_sub: 50,
            opex_sub: 30,
            horizon: 2,
            wait_cohort: WaitCohort::All,
            d_exposure: ExposureMode::All,
        };
        let o = evaluate(&a, Regime::IbteCancelPrio, &e).ok_or("evaluate None")?;
        if o.mev_mm_q != 0 || o.mev_tk_q != 0 {
            return Err(format!(
                "zero_capture_zero_mev: expected 0 MEV when capture=0, got mm={}, tk={}",
                o.mev_mm_q, o.mev_tk_q
            ));
        }
        Ok("capture=0% enforces zero MEV extraction regardless of leak_ppm".to_string())
    }

    pub fn d_exposure_modes_and_breakeven() -> Result<String, String> {
        let a = Agg {
            mm_n: 1_000_000,
            mm_w: [-100_000_000, -100_000_000, -100_000_000], // -1.000 bp
            tk_n: 1_000_000,
            tk_scoef_w: 1_000_000,
            mm_large_n: 800_000,
            tk_large_n: 800_000,
            mm_large_exp_n: 200_000,
            tk_large_exp_n: 200_000,
            ..Agg::default()
        };
        let e_all = Econ {
            capture_ppm: 250_000,
            impact_sub: 100,
            mm_share_ppm: 500_000,
            leak_ppm: 250_000,
            rebate_sub: 50,
            opex_sub: 30,
            horizon: 2,
            wait_cohort: WaitCohort::All,
            d_exposure: ExposureMode::All,
        };
        let mut e_cb = e_all;
        e_cb.d_exposure = ExposureMode::CancelBatch;

        let ob = evaluate(&a, Regime::CancelPrio, &e_all).ok_or("ob None")?;
        let od_all = evaluate(&a, Regime::IbteCancelPrio, &e_all).ok_or("od_all None")?;
        let od_cb = evaluate(&a, Regime::IbteCancelPrio, &e_cb).ok_or("od_cb None")?;

        if ob.mev_mm_q != od_all.mev_mm_q || ob.mev_tk_q != od_all.mev_tk_q {
            return Err("d_exposure_mode::All should equal B when leak == capture".to_string());
        }
        if od_cb.mev_mm_q * 4 != od_all.mev_mm_q {
            return Err("d_exposure_mode::CancelBatch should scale by 200k/800k = 1/4".to_string());
        }

        let e0 = Econ {
            capture_ppm: 0,
            impact_sub: 0,
            mm_share_ppm: 500_000,
            leak_ppm: 0,
            rebate_sub: 50,
            opex_sub: 30,
            horizon: 2,
            wait_cohort: WaitCohort::All,
            d_exposure: ExposureMode::All,
        };
        let be = breakeven_leak_ppm(&a, &a, &e0, 250_000, 100, 500_000, 1_000_000)
            .ok_or("breakeven_leak_ppm returned None")?;
        if be != 250_000 {
            return Err(format!("expected breakeven leak 250_000 ppm at 100% exposure, got {}", be));
        }
        Ok("ExposureMode::All vs CancelBatch and break-even leak math verified".to_string())
    }

    pub fn dropped_precision() -> Result<String, String> {
        let a = Agg {
            tk_n: 99_995_000,
            tk_dropped: 10,
            tk_dropped_usd: 5_000, // 0.005%
            ..Agg::default()
        };
        let s = fmt_dropped_cell(&a);
        if s != "10 (0.005%)" {
            return Err(format!("dropped_precision: expected '10 (0.005%)', got '{}'", s));
        }
        Ok("fmt_dropped_cell resolves sub-0.1% dropped USD to 0.001% precision".to_string())
    }

    pub fn run_all() -> bool {
        let checks: [(&str, fn() -> Result<String, String>); 17] = [
            ("causality (no look-ahead)", causality),
            ("identity D==B (decrypt=0, mev=0, leak=0)", identity),
            ("shuffled-side zero-spread + sign control", shuffled_zero_spread),
            ("MEV share conservation (no double-count)", mev_share_sum),
            ("sniper protection via cancel-priority", sniper_protection),
            ("accounting reconciliation (volumes & All-In)", accounting_reconciliation),
            ("placebo breaks the flow-price link", placebo_breaks_link),
            ("pending-policy drop vs hold semantics", pending_drop_policy),
            ("Hyperliquid batch order (New->Cancel->Taker)", hl_batch_order),
            ("Regime D post-decrypt HL batch order", d_post_decrypt_hl_order),
            ("FBA sniper stale vs late_fresh attribution", fba_sniper_attribution),
            ("WaitCohort::All single-denominator identity", wait_cohort_all_identity),
            ("BidAskProxy midpoint & crossed diagnostics", bidask_mid_proxy),
            ("BookTicker raw/dedup/interval diagnostics", book_ticker_diag),
            ("Zero capture enforces zero MEV under leak>0", zero_capture_zero_mev),
            ("ExposureMode (all vs cancel-batch) & break-even leak", d_exposure_modes_and_breakeven),
            ("Dropped USD 0.001% display precision", dropped_precision),
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
    fn t_hl_batch_order() {
        selfcheck::hl_batch_order().unwrap();
    }
    #[test]
    fn t_d_post_decrypt_hl_order() {
        selfcheck::d_post_decrypt_hl_order().unwrap();
    }
    #[test]
    fn t_fba_sniper_attribution() {
        selfcheck::fba_sniper_attribution().unwrap();
    }
    #[test]
    fn t_wait_cohort_all_identity() {
        selfcheck::wait_cohort_all_identity().unwrap();
    }
    #[test]
    fn t_bidask_mid_proxy() {
        selfcheck::bidask_mid_proxy().unwrap();
    }
    #[test]
    fn t_book_ticker_diag() {
        selfcheck::book_ticker_diag().unwrap();
    }
    #[test]
    fn t_zero_capture_zero_mev() {
        selfcheck::zero_capture_zero_mev().unwrap();
    }
    #[test]
    fn t_d_exposure_modes_and_breakeven() {
        selfcheck::d_exposure_modes_and_breakeven().unwrap();
    }
    #[test]
    fn t_dropped_precision() {
        selfcheck::dropped_precision().unwrap();
    }
    #[test]
    fn t_html_escape() {
        assert_eq!(super::html_escape("D<B & B<D"), "D&lt;B &amp; B&lt;D");
    }
}

// ============================================================================
// Markdown & HTML Report Generation
// ============================================================================

fn base_econ(args: &Args) -> Econ {
    Econ {
        capture_ppm: 0,
        impact_sub: 0,
        mm_share_ppm: 500_000,
        leak_ppm: 0,
        rebate_sub: args.rebate_sub,
        opex_sub: args.opex_sub,
        horizon: 2,
        wait_cohort: args.wait_cohort,
        d_exposure: args.d_exposure_mode,
    }
}

fn markout1s(a: &Agg) -> Option<i128> {
    if a.mm_n > 0 {
        Some(a.mm_w[2] * Q / a.mm_n)
    } else {
        None
    }
}

fn table(rep: &mut String, title: &str, specs: &[Spec], totals: &[Agg], shift: i64, args: &Args) {
    let e0 = base_econ(args);
    let wait_col = match args.wait_cohort {
        WaitCohort::All => "[3] Wait (All)",
        WaitCohort::Large => "[3] Wait (Large)",
    };
    emit!(rep, "\n### {}", title);
    emit!(
        rep,
        "| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | {} | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |",
        wait_col
    );
    emit!(
        rep,
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
    );
    for (sp, a) in specs.iter().zip(totals.iter()) {
        if sp.shift != shift {
            continue;
        }
        match evaluate(a, sp.regime, &e0) {
            None => {
                emit!(rep, "| {} | n/a | | | | | | | | | | | | | |", sp.label);
            }
            Some(o) => {
                let m0 = a.mm_w[0] * Q / a.mm_n;
                let m1 = a.mm_w[1] * Q / a.mm_n;
                let m2 = a.mm_w[2] * Q / a.mm_n;
                emit!(
                    rep,
                    "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}/{} | {} | {} |",
                    sp.label,
                    fmt_usd(a.mm_n),
                    fmt_usd(a.tk_n),
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
                    fmt_dropped_cell(a),
                    a.batches
                );
            }
        }
    }
    let drop_note = if args.pending_policy == PendingPolicy::Drop {
        " Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias)."
    } else {
        ""
    };
    emit!(
        rep,
        "\nUnits: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`{}`; requote_thr=`{} bp`, sniper_thr=`{} bp`).{}",
        args.wait_cohort.as_str(),
        fmt_bps((args.requote_thr_sub as i128) * Q),
        fmt_bps((args.sniper_thr_sub as i128) * Q),
        drop_note
    );
}

fn cohort_table(rep: &mut String, title: &str, specs: &[Spec], totals: &[Agg], args: &Args) {
    emit!(rep, "\n### {} — MM Flow Cohort Decomposition & Volume Reconciliation", title);
    emit!(
        rep,
        "| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | 1-ms Burst Vol % (>={}) | Cancel-Batch Large % |",
        fmt_usd(args.large_usd as i128)
    );
    emit!(rep, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
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
        let large_pm = if a.tk_n > 0 {
            a.tk_large_n * 1000 / a.tk_n
        } else {
            0
        };
        let exp_pm = if a.tk_large_n > 0 {
            a.tk_large_exp_n * 1000 / a.tk_large_n
        } else {
            0
        };
        emit!(
            rep,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            sp.label,
            fmt_usd(a.mm_taker_n),
            fmt_usd(a.mm_sniper_stale_n),
            fmt_usd(a.mm_sniper_late_n),
            fmt_usd(a.mm_n),
            fmt_usd(diff_v),
            m_tk,
            m_st,
            m_lt,
            fmt_permille(large_pm),
            fmt_permille(exp_pm)
        );
    }
    emit!(
        rep,
        "\nReconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). `1-ms Burst Vol %` reflects directional trades merged within the same millisecond (`merge_taker`); see Raw Taker Diagnostics above for the unmerged lower bound."
    );
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
    selection_bias_warn: Option<String>,
}

fn frontier(
    rep: &mut String,
    csv: &mut String,
    specs: &[Spec],
    full: &[Agg],
    boot: &[Vec<Agg>],
    args: &Args,
) -> (Vec<FrontierGrid>, String) {
    let mut html_grids = Vec::new();
    let mut be_html = String::new();
    csv.push_str("react_ms,decrypt_ms,b_window_ms,d_window_ms,b_batch_order,d_batch_order,d_exposure_mode,requote_thr_sub,sniper_thr_sub,pending_policy,mid_source,mid_mode,wait_cohort,rebate_sub,opex_sub,b_dropped_usd_p100k,d_dropped_usd_p100k,capture_ppm,impact_sub,mm_share_ppm,leak_ppm,allin_B_q,allin_D_q,delta_D_minus_B_q,ci_lo_q,ci_hi_q,verdict\n");
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
            let db_p100k = full[ib].dropped_usd_p100k();
            let dd_p100k = full[id].dropped_usd_p100k();
            let bias_warn = if (db_p100k - dd_p100k).abs() >= 1_000 {
                Some(format!(
                    "Dropped taker USD share differs materially between B ({}) and D ({}). Direct All-In comparison is confounded by fill-rate selection bias; use `--pending-policy hold` or matching batch orders (`--b-batch-order hl --d-batch-order hl`).",
                    fmt_pct3(db_p100k),
                    fmt_pct3(dd_p100k)
                ))
            } else {
                None
            };

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
                                rebate_sub: args.rebate_sub,
                                opex_sub: args.opex_sub,
                                horizon: 2,
                                wait_cohort: args.wait_cohort,
                                d_exposure: args.d_exposure_mode,
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
                                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                                react,
                                dec,
                                bw,
                                dw,
                                args.b_batch_order.as_str(),
                                args.d_batch_order.as_str(),
                                args.d_exposure_mode.as_str(),
                                args.requote_thr_sub,
                                args.sniper_thr_sub,
                                args.pending_policy.as_str(),
                                args.mid_source.as_str(),
                                args.mid_mode.as_str(),
                                args.wait_cohort.as_str(),
                                args.rebate_sub,
                                args.opex_sub,
                                db_p100k,
                                dd_p100k,
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
                    "\n#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B {}ms ({}), D {}ms (dec {}ms, {}, exp={}), react {}ms | pending={} | src={} | wait={}",
                    bw,
                    args.b_batch_order.as_str(),
                    dw,
                    dec,
                    args.d_batch_order.as_str(),
                    args.d_exposure_mode.as_str(),
                    react,
                    args.pending_policy.as_str(),
                    args.mid_source.as_str(),
                    args.wait_cohort.as_str()
                );
                if let Some(ref warn_msg) = bias_warn {
                    emit!(rep, "> **WARNING (Selection Bias):** {}", warn_msg);
                }
                emit!(
                    rep,
                    "Rows: MEV capture rate on 1-ms bursts (>={}) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**",
                    fmt_usd(args.large_usd as i128)
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
                    selection_bias_warn: bias_warn,
                });

                // Break-Even Leakage Table for the primary decrypt delay (35 ms or first non-zero)
                if dec == 35 || (args.decrypt.len() == 1 && dec == args.decrypt[0]) {
                    let obs_exp_ppm = full[id].cancel_batch_exp_ppm();
                    let exp_cols: [(String, i128); 4] = [
                        (format!("Cancel-Batch ({})", fmt_ppm_pct1(obs_exp_ppm)), obs_exp_ppm),
                        ("50.0% Exposed".to_string(), 500_000),
                        ("75.0% Exposed".to_string(), 750_000),
                        ("100.0% Exposed (All)".to_string(), 1_000_000),
                    ];
                    let e0 = base_econ(args);
                    let pairs: [(i128, i128); 5] = [
                        (100_000, 50),
                        (100_000, 100),
                        (250_000, 100),
                        (500_000, 100),
                        (500_000, 200),
                    ];
                    emit!(
                        rep,
                        "\n#### Break-Even Residual Leakage `eff_leak*` (where All-In(D, dec {}ms) == All-In(B, {}ms))",
                        dec, bw
                    );
                    emit!(
                        rep,
                        "Dywe is cheaper for takers (`D < B`) whenever actual residual leakage `eff_leak < eff_leak*` (note `eff_leak <= capture` by definition; values `>= capture` mean D stays cheaper for any `leak <= capture`)."
                    );
                    emit!(
                        rep,
                        "| Ambient Capture / Impact | {} | {} | {} | {} |",
                        exp_cols[0].0, exp_cols[1].0, exp_cols[2].0, exp_cols[3].0
                    );
                    emit!(rep, "|---|---:|---:|---:|---:|");

                    let mut tbody_be = String::new();
                    for &(cap, imp) in pairs.iter() {
                        let row_lbl = format!("{}% / {} bp", cap / 10_000, fmt_bps(imp * Q));
                        let mut md_row = format!("| {} |", row_lbl);
                        tbody_be.push_str(&format!("<tr><td><strong>{}</strong></td>", html_escape(&row_lbl)));
                        for &(_, exp_ppm) in exp_cols.iter() {
                            let val_s = match breakeven_leak_ppm(&full[ib], &full[id], &e0, cap, imp, 500_000, exp_ppm) {
                                Some(be) => {
                                    if be > 1_000_000 {
                                        ">100.0%".to_string()
                                    } else if be < 0 {
                                        "<0.0% (never)".to_string()
                                    } else {
                                        fmt_ppm_pct1(be)
                                    }
                                }
                                None => "n/a".to_string(),
                            };
                            md_row.push_str(&format!(" {} |", val_s));
                            tbody_be.push_str(&format!("<td>{}</td>", html_escape(&val_s)));
                        }
                        tbody_be.push_str("</tr>\n");
                        emit!(rep, "{}", md_row);
                    }

                    be_html = format!(
                        r#"<div class="table-card" style="margin-bottom: 20px;">
                            <h2>Break-Even Residual Leakage eff_leak* (All-In(D, {}ms) == All-In(B, {}ms))</h2>
                            <p style="font-size: 12px; color: var(--text-muted); margin-bottom: 12px;">
                                Maximum tolerable effective leakage rate in Regime D before All-In(D) reaches parity with Regime B (50% MM MEV share). Dywe is cheaper whenever <code>eff_leak &lt; eff_leak*</code>.
                            </p>
                            <table>
                                <thead>
                                    <tr>
                                        <th>Ambient Capture / Impact</th>
                                        <th>{}</th>
                                        <th>{}</th>
                                        <th>{}</th>
                                        <th>{}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {}
                                </tbody>
                            </table>
                        </div>"#,
                        dec,
                        bw,
                        html_escape(&exp_cols[0].0),
                        html_escape(&exp_cols[1].0),
                        html_escape(&exp_cols[2].0),
                        html_escape(&exp_cols[3].0),
                        tbody_be
                    );
                }
            }
        }
    }
    (html_grids, be_html)
}

fn render_mech_rows_html(specs: &[Spec], totals: &[Agg], shift: i64, args: &Args) -> String {
    let e0 = base_econ(args);
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
                    "<tr{}><td>{}</td><td colspan=\"14\">n/a</td></tr>\n",
                    tr_cls, label
                ));
            }
            Some(o) => {
                let m0 = a.mm_w[0] * Q / a.mm_n;
                let m1 = a.mm_w[1] * Q / a.mm_n;
                let m2 = a.mm_w[2] * Q / a.mm_n;
                rows.push_str(&format!(
                    "<tr{}><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><strong>{}</strong></td><td>{}/{}</td><td>{}</td><td>{}</td></tr>\n",
                    tr_cls,
                    label,
                    fmt_usd(a.mm_n),
                    fmt_usd(a.tk_n),
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
                    html_escape(&fmt_dropped_cell(a)),
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
        let large_pm = if a.tk_n > 0 {
            a.tk_large_n * 1000 / a.tk_n
        } else {
            0
        };
        let exp_pm = if a.tk_large_n > 0 {
            a.tk_large_exp_n * 1000 / a.tk_large_n
        } else {
            0
        };
        rows.push_str(&format!(
            "<tr{}><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            tr_cls,
            html_escape(&sp.label),
            fmt_usd(a.mm_taker_n),
            fmt_usd(a.mm_sniper_stale_n),
            fmt_usd(a.mm_sniper_late_n),
            fmt_usd(a.mm_n),
            fmt_usd(diff_v),
            m_tk,
            m_st,
            m_lt,
            fmt_permille(large_pm),
            fmt_permille(exp_pm)
        ));
    }
    rows
}

fn mech_table_html(title: &str, rows_html: &str, wait_col_hdr: &str) -> String {
    format!(
        r#"<div class="table-card" style="margin-bottom: 22px;">
                <h2>{}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>Architecture Regime</th>
                            <th>MM Vol (incl. Probe)</th>
                            <th>Hist Taker Vol</th>
                            <th>Markout 0ms</th>
                            <th>Markout 100ms</th>
                            <th>Markout 1s</th>
                            <th>s* Gross</th>
                            <th>s* Net</th>
                            <th>[1] Spread Paid</th>
                            <th>[2] Taker Stale</th>
                            <th>{}</th>
                            <th>[=] All-In Net</th>
                            <th>StaleSnipe Hit/Miss</th>
                            <th>Dropped Takers (USD%)</th>
                            <th>Batches</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>"#,
        html_escape(title),
        html_escape(wait_col_hdr),
        rows_html
    )
}

fn render_placebo_html(specs: &[Spec], full: &[Agg], args: &Args, wait_col_hdr: &str) -> String {
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
    out.push_str("<li><strong>Reading:</strong> circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not test midpoint reconstruction accuracy.</li></ul></div>");
    for &s in &args.placebo_shifts {
        let rows = render_mech_rows_html(specs, full, s, args);
        if !rows.is_empty() {
            out.push_str(&mech_table_html(
                &format!("Placebo: takers shifted {}s (consolidated, MEV = 0)", s / 1000),
                &rows,
                wait_col_hdr,
            ));
        }
    }
    out
}

fn render_calibration_html() -> String {
    let mut out = String::from(
        "<div class=\"bounds-box\"><h3>External Venue Documentation &amp; Parameterized Assumptions</h3><ul>",
    );
    for (topic, statement, source) in EXTERNAL_CALIBRATION_NOTES.iter() {
        out.push_str(&format!(
            "<li><strong>{}:</strong> {} <em>Source: {}</em></li>",
            html_escape(topic),
            html_escape(statement),
            html_escape(source)
        ));
    }
    out.push_str("</ul><h3 style=\"margin-top: 14px;\">Dywe Protocol Design Parameters &amp; Threat Model</h3><ul>");
    for (topic, statement) in DYWE_DESIGN_NOTES.iter() {
        out.push_str(&format!(
            "<li><strong>{}:</strong> {}</li>",
            html_escape(topic),
            html_escape(statement)
        ));
    }
    out.push_str("</ul></div>");
    out
}

fn format_proxy_diag(diag: &ProxyDiag) -> String {
    if diag.total_obs == 0 {
        return String::new();
    }
    let crossed_pm = (diag.crossed_obs as i128) * 1000 / (diag.total_obs as i128);
    let mean_spread_q = if diag.valid_spread_obs > 0 {
        diag.spread_sum_sub * Q / (diag.valid_spread_obs as i128)
    } else {
        0
    };
    format!(
        "BidAskProxy diagnostics: {} total observations, {} crossed ({} where ask < bid), mean valid (ask - bid) spread = {} bps.",
        diag.total_obs,
        diag.crossed_obs,
        fmt_permille(crossed_pm),
        fmt_bps(mean_spread_q)
    )
}

fn format_book_diag(diag: &BookTickerDiag) -> String {
    if diag.files_loaded == 0 {
        return String::new();
    }
    let med = diag
        .median_dt_ms()
        .map_or("n/a".to_string(), |m| format!("{} ms", m));
    let mean_dt = if diag.dt_intervals > 0 {
        format!("{:.2} ms", (diag.dt_sum_ms as f64) / (diag.dt_intervals as f64))
    } else {
        "n/a".to_string()
    };
    format!(
        "Native L1 bookTicker diagnostics ({} files): {} raw quote rows -> {} distinct ms-level mid changes ({} same-ms collapsed, {} unchanged-mid skipped); inter-update interval median = {}, mean = {}.",
        diag.files_loaded,
        diag.raw_rows,
        diag.distinct_mids,
        diag.same_ms_collapsed,
        diag.unchanged_mid_skipped,
        med,
        mean_dt
    )
}

fn format_taker_diag(diag: &RawTakerDiag, large_usd: i64) -> String {
    if diag.raw_rows == 0 {
        return String::new();
    }
    let raw_sh = if diag.raw_vol_usd > 0 {
        diag.raw_large_vol_usd * 1000 / diag.raw_vol_usd
    } else {
        0
    };
    let mrg_sh = if diag.merged_vol_usd > 0 {
        diag.merged_large_vol_usd * 1000 / diag.merged_vol_usd
    } else {
        0
    };
    format!(
        "Historical taker flow diagnostics (pure exchange volume {}, threshold >={}): unmerged raw rows = {} ({} large rows, {} of USD volume); 1-ms directional bursts (`merge_taker`) = {} ({} large bursts, {} of USD volume).",
        fmt_usd(diag.raw_vol_usd),
        fmt_usd(large_usd as i128),
        diag.raw_rows,
        diag.raw_large_rows,
        fmt_permille(raw_sh),
        diag.merged_bursts,
        diag.merged_large_bursts,
        fmt_permille(mrg_sh)
    )
}

fn generate_html_report(
    output_path: &Path,
    args: &Args,
    days_ok: usize,
    total_tasks: usize,
    days_reconstructed: usize,
    total_mid_ticks: usize,
    total_taker_orders: usize,
    proxy_diag: &ProxyDiag,
    book_diag: &BookTickerDiag,
    taker_diag: &RawTakerDiag,
    nblocks: usize,
    specs: &[Spec],
    per_sym: &[Vec<Vec<Agg>>],
    full: &[Agg],
    html_grids: &[FrontierGrid],
    breakeven_leak_html: &str,
    sign_check_text: &str,
) -> Result<(), String> {
    let file = File::create(output_path)
        .map_err(|e| format!("Failed to create {}: {}", output_path.display(), e))?;
    let mut w = BufWriter::new(file);

    let e0 = base_econ(args);
    let wait_col_hdr = match args.wait_cohort {
        WaitCohort::All => "[3] Wait (All)",
        WaitCohort::Large => "[3] Wait (Large)",
    };
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
        rebate_sub: args.rebate_sub,
        opex_sub: args.opex_sub,
        horizon: 2,
        wait_cohort: args.wait_cohort,
        d_exposure: args.d_exposure_mode,
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

    let cons_rows_html = render_mech_rows_html(specs, full, 0, args);
    let cons_cohort_html = render_cohort_rows_html(specs, full);

    let mut per_sym_sections_html = String::new();
    for (s_idx, sym_name) in args.symbols.iter().enumerate() {
        let tot: Vec<Agg> = per_sym[s_idx].iter().map(|c| total(c)).collect();
        let rows_html = render_mech_rows_html(specs, &tot, 0, args);
        per_sym_sections_html.push_str(&mech_table_html(
            &format!("{} — Pure Microstructure Mechanics (MEV = 0)", sym_name),
            &rows_html,
            wait_col_hdr,
        ));
    }

    let placebo_html = render_placebo_html(specs, full, args, wait_col_hdr);
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

    let rebate_bps = fmt_bps(args.rebate_sub * Q);
    let opex_bps = fmt_bps(args.opex_sub * Q);
    let req_bps = fmt_bps((args.requote_thr_sub as i128) * Q);
    let snp_bps = fmt_bps((args.sniper_thr_sub as i128) * Q);
    let large_usd_str = fmt_usd(args.large_usd as i128);

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
        let bias_html = match fg.selection_bias_warn {
            Some(ref msg) => format!(
                "<div class=\"warn-banner\" style=\"margin-bottom: 12px;\"><strong>WARNING (Selection Bias):</strong> {}</div>",
                html_escape(msg)
            ),
            None => String::new(),
        };
        frontier_cards_html.push_str(&format!(
            r#"<div class="table-card" style="margin-bottom: 20px;">
                <h2>Break-Even Frontier: All-In(D) &minus; All-In(B) (bps) &mdash; Decrypt {}ms | React {}ms</h2>
                {}
                <p style="font-size: 12px; color: var(--text-muted); margin-bottom: 12px;">
                    Assumptions: MM MEV Share = 50%, Leak = 10% (<code>eff_leak = min(leak, capture)</code>), D Exposure = <code>{}</code>, Rebate = {} bp (paid to MM), OpEx = {} bp, Burst Threshold = {}, B/D Order = <code>{}/{}</code>, Pending = <code>{}</code>, Source = <code>{}</code>, Wait = <code>{}</code>.<br>
                    <strong>D&lt;B</strong> (purple): Dywe cheaper (95% CI &lt; 0); <strong>B&lt;D</strong> (amber): Regime B cheaper; <strong>~</strong>: indistinguishable.
                    <strong>Every cell with capture &gt; 0% is a parameterized scenario output, not a measurement.</strong>
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
            bias_html,
            args.d_exposure_mode.as_str(),
            rebate_bps,
            opex_bps,
            large_usd_str,
            args.b_batch_order.as_str(),
            args.d_batch_order.as_str(),
            args.pending_policy.as_str(),
            args.mid_source.as_str(),
            args.wait_cohort.as_str(),
            tbody
        ));
    }

    let data_source_badge = if days_reconstructed > 0 {
        format!("Reconstructed ({}, {}/{}d)", args.mid_mode.as_str(), days_reconstructed, days_ok)
    } else {
        "L1 BookTicker Mid".to_string()
    };

    let mut banner_parts: Vec<String> = Vec::new();
    if taker_diag.raw_rows > 0 {
        banner_parts.push(html_escape(&format_taker_diag(taker_diag, args.large_usd)));
    }
    if book_diag.files_loaded > 0 {
        banner_parts.push(html_escape(&format_book_diag(book_diag)));
    }
    if days_reconstructed > 0 {
        banner_parts.push(format!(
            "<strong>Methodological Note on Mid-Price Source:</strong> {} of {} days used a trade-derived midpoint proxy (mode: <code>{}</code>) rather than L1 <code>bookTicker</code> quotes. {}",
            days_reconstructed,
            days_ok,
            args.mid_mode.as_str(),
            html_escape(&format_proxy_diag(proxy_diag))
        ));
    }
    let recon_banner = if banner_parts.is_empty() {
        String::new()
    } else {
        format!(
            "<div class=\"warn-banner\">{}</div>",
            banner_parts.join("<br>")
        )
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dywe Protocol — Causal Microstructure &amp; Break-Even Ablation Report ({ver})</title>
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
        <span class="brand-pill"><span class="brand-dot"></span>DYWE PROTOCOL // CAUSAL MICROSTRUCTURE RESEARCH ({ver})</span>
        <span class="engine-tag">B={b_order} &bull; D={d_order} &bull; D-EXP={d_exp} &bull; REQ={req_bps}bp &bull; PENDING={pending_pol} &bull; DETERMINISTIC i128</span>
    </div>

    <header>
        <div class="title-group">
            <h1>DYWE EXCHANGE &mdash; CAUSAL MICROSTRUCTURE &amp; BREAK-EVEN ABLATION STUDY</h1>
            <p>Scenario-Based Counterfactual Replay on Binance Futures flow ({syms}, {start} + {days}d | {nblocks} Joint {bh}h Bootstrap Blocks | Rebate {rebate_bps}bp to MM, OpEx {opex_bps}bp)</p>
        </div>
        <div class="meta-badges">
            <span class="badge">Pair-Days: <strong>{days_ok}/{total_tasks}</strong></span>
            <span class="badge">Mid Ticks: <strong>{total_mid_ticks}</strong></span>
            <span class="badge">1-ms Bursts: <strong>{total_taker_orders}</strong></span>
            <span class="badge">Requote Thr: <strong>{req_bps}bp</strong></span>
            <span class="badge">D Exposure: <strong>{d_exp}</strong></span>
            <span class="badge">Pending: <strong>{pending_pol}</strong></span>
            <span class="badge">Source: <strong>{data_source_badge}</strong></span>
        </div>
    </header>

    {recon_banner}

    <div class="kpi-grid">
        <div class="kpi-card">
            <div class="label">Net Half-Spread s* (MEV=0), Regime D ({d_dec}ms decrypt)</div>
            <div class="value">{d35_snet} bps</div>
            <span class="kpi-pill">vs {a_snet} bps (FIFO) &amp; {b_snet} bps (Regime B {b_win}ms)</span>
        </div>
        <div class="kpi-card">
            <div class="label">1s MM Markout (MEV=0), Regime D</div>
            <div class="value">{d35_m1s} bps</div>
            <span class="kpi-pill">vs {a_m1s} bps (FIFO); see cohort table</span>
        </div>
        <div class="kpi-card">
            <div class="label">{d_dec}ms Decrypt Taker Delta vs Regime B (Zero-MEV)</div>
            <div class="value">{zero_mev_cost}</div>
            <span class="kpi-pill">All-In Taker Cost delta (wait={wait_cohort})</span>
        </div>
        <div class="kpi-card">
            <div class="label">Scenario &Delta; All-In (25% cap, 1bp, 10% leak, exp={d_exp})</div>
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
                    <th>MM Vol (incl. Probe)</th>
                    <th>Hist Taker Vol</th>
                    <th>Markout 0ms</th>
                    <th>Markout 100ms</th>
                    <th>Markout 1s</th>
                    <th>s* Gross</th>
                    <th>s* Net</th>
                    <th>[1] Spread Paid</th>
                    <th>[2] Taker Stale</th>
                    <th>{wait_col_hdr}</th>
                    <th>[=] All-In Net</th>
                    <th>StaleSnipe Hit/Miss</th>
                    <th>Dropped Takers (USD%)</th>
                    <th>Batches</th>
                </tr>
            </thead>
            <tbody>
                {cons_rows_html}
            </tbody>
        </table>
        <p style="margin-top: 10px; font-size: 11.5px; color: var(--text-muted);">
            <strong>Identity (MEV = 0):</strong> <code>[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net</code> (wait cohort: <code>{wait_cohort}</code>; requote_thr: <code>{req_bps}bp</code>, sniper_thr: <code>{snp_bps}bp</code>).
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
                    <th>1-ms Burst Vol % (&ge;{large_usd_str})</th>
                    <th>Cancel-Batch Large %</th>
                </tr>
            </thead>
            <tbody>
                {cons_cohort_html}
            </tbody>
        </table>
    </div>

    {per_sym_sections_html}

    <h2 style="font-size: 18px; margin: 26px 0 12px 0; color: #ffffff;">Phase 2: Break-Even Frontier &amp; Break-Even Leakage Across MEV Scenarios</h2>
    {frontier_cards_html}
    {breakeven_leak_html}

    {calibration_html}

    <div class="bounds-box">
        <h3>Methodological Scope &amp; Statistical Boundaries</h3>
        <ul>
            <li><strong>Regime D timeline &amp; exposure:</strong> orders commit at <code>batch_close</code>, decrypt at <code>clear_ns = batch_close + decrypt_ms</code>, and clear using <code>{d_order}</code> ordering. Windows: B {b_win}ms ({b_order}), C {c_win}ms, D {d_win}ms ({d_order}). Exposure mode: <code>{d_exp}</code>.</li>
            <li><strong>MM requote &amp; sniper probe parameters:</strong> representative MM requotes when mid moves &ge;<code>{req_bps} bp</code> (<code>--requote-thr-sub</code>); synthetic probe triggers at &ge;<code>{snp_bps} bp</code> (<code>--sniper-thr-sub</code>) with 2 ms reaction and ${sniper_clip} clip (<code>--sniper-clip</code>).</li>
            <li><strong>1-ms burst aggregation:</strong> <code>merge_taker</code> aggregates same-millisecond, same-direction executions; inspect Raw Taker Diagnostics above for the unmerged lower bound.</li>
            <li><strong>95% bootstrap CI:</strong> joint {bh}-hour block resampling ({nblocks} blocks). Quantifies drift sampling variance under fixed scenario assumptions and does not account for volatility autocorrelation or parameter uncertainty.</li>
        </ul>
    </div>
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
                        label: 'Regime D: IBTE ({d_win}ms, {d_dec}ms dec, {d_order})',
                        data: dD,
                        borderColor: '#c28aff',
                        backgroundColor: 'rgba(194, 138, 255, 0.12)',
                        borderWidth: 3,
                        pointRadius: 4.5,
                        tension: 0.2
                    }},
                    {{
                        label: 'Regime B: Batch ({b_win}ms, {b_order})',
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
        ver = VERSION,
        syms = html_escape(&args.symbols.join(", ")),
        start = html_escape(&args.start),
        days = args.days,
        nblocks = nblocks,
        bh = args.block_hours,
        days_ok = days_ok,
        total_tasks = total_tasks,
        total_mid_ticks = total_mid_ticks,
        total_taker_orders = total_taker_orders,
        b_order = args.b_batch_order.as_str(),
        d_order = args.d_batch_order.as_str(),
        d_exp = args.d_exposure_mode.as_str(),
        pending_pol = args.pending_policy.as_str(),
        wait_cohort = args.wait_cohort.as_str(),
        wait_col_hdr = html_escape(wait_col_hdr),
        rebate_bps = rebate_bps,
        opex_bps = opex_bps,
        req_bps = req_bps,
        snp_bps = snp_bps,
        large_usd_str = html_escape(&large_usd_str),
        sniper_clip = args.sniper_clip_usd,
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
        breakeven_leak_html = breakeven_leak_html,
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

    emit!(
        rep,
        "# Dywe Microstructure Ablation — Scenario-Based Causal Backtest ({})",
        VERSION
    );
    emit!(
        rep,
        "Window: {} + {}d | Pairs: {} | Bootstrap Block: {}h ({} blocks) | Runs per (Pair, Day): {} | B {}ms ({}), D {}ms ({}, exp={}), FBA {}ms | requote_thr={}bp, sniper_thr={}bp | pending={} | src={} | mid={} | wait={} | sniper_clip={} | rebate={}bp (paid to MM), opex={}bp, large>={} | placebo shifts (s): {:?}",
        args.start,
        args.days,
        args.symbols.join(","),
        args.block_hours,
        nblocks,
        specs.len(),
        args.b_window(),
        args.b_batch_order.as_str(),
        args.window_ms,
        args.d_batch_order.as_str(),
        args.d_exposure_mode.as_str(),
        args.fba_window_ms,
        fmt_bps((args.requote_thr_sub as i128) * Q),
        fmt_bps((args.sniper_thr_sub as i128) * Q),
        args.pending_policy.as_str(),
        args.mid_source.as_str(),
        args.mid_mode.as_str(),
        args.wait_cohort.as_str(),
        fmt_usd(args.sniper_clip_usd as i128),
        fmt_bps(args.rebate_sub * Q),
        fmt_bps(args.opex_sub * Q),
        fmt_usd(args.large_usd as i128),
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
    let mut total_proxy_diag = ProxyDiag::default();
    let mut total_book_diag = BookTickerDiag::default();
    let mut total_taker_diag = RawTakerDiag::default();

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
                    total_proxy_diag.merge(&r.proxy_diag);
                    total_book_diag.merge(&r.book_diag);
                    total_taker_diag.merge(&r.taker_diag);
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
    if total_taker_diag.raw_rows > 0 {
        emit!(
            rep,
            "\n**TAKER FLOW DIAGNOSTICS:** {}",
            format_taker_diag(&total_taker_diag, args.large_usd)
        );
    }
    if total_book_diag.files_loaded > 0 {
        emit!(rep, "\n**BOOKTICKER DIAGNOSTICS:** {}", format_book_diag(&total_book_diag));
    }
    if days_reconstructed > 0 {
        emit!(
            rep,
            "\n**PROXY NOTE:** {} pair-days used a trade-derived midpoint proxy (`--mid-source {}`, mode `{}`). {}",
            days_reconstructed,
            args.mid_source.as_str(),
            args.mid_mode.as_str(),
            format_proxy_diag(&total_proxy_diag)
        );
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
        table(
            &mut rep,
            &format!("{} (MEV = 0)", args.symbols[s]),
            &specs,
            &tot,
            0,
            &args,
        );
    }
    table(
        &mut rep,
        "Consolidated Multi-Pair (MEV = 0)",
        &specs,
        &full,
        0,
        &args,
    );
    cohort_table(&mut rep, "Consolidated Multi-Pair", &specs, &full, &args);

    if !args.placebo_shifts.is_empty() {
        emit!(rep, "\n## Placebo test: circularly shifted taker flow");
        emit!(
            rep,
            "The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed."
        );
        for &s in &args.placebo_shifts {
            table(
                &mut rep,
                &format!("Placebo: takers shifted {}s (consolidated, MEV = 0)", s / 1000),
                &specs,
                &full,
                s,
                &args,
            );
        }
        emit!(rep, "\n### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)");
        for l in placebo_lines(&specs, &full, &args) {
            emit!(rep, "- {}", l);
        }
        emit!(
            rep,
            "- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy."
        );
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

    emit!(rep, "\n## Phase 2: Break-Even Frontier & Break-Even Leakage (MEV Scenario Assumptions)");
    emit!(
        rep,
        "All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × eff_leak`, where `eff_leak = min(leak, capture)`), not measurements. Exposure mode for Regime D: `{}`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.",
        args.d_exposure_mode.as_str()
    );
    let mut csv = String::new();
    let (html_grids, breakeven_leak_html) =
        frontier(&mut rep, &mut csv, &specs, &full, &boot, &args);

    emit!(rep, "\n## External Venue Documentation & Parameterized Assumptions (NOT simulation inputs)");
    for (topic, statement, source) in EXTERNAL_CALIBRATION_NOTES.iter() {
        emit!(rep, "- **{}:** {} _Source: {}_", topic, statement, source);
    }

    emit!(rep, "\n## Dywe Protocol Design Parameters & Threat Model");
    for (topic, statement) in DYWE_DESIGN_NOTES.iter() {
        emit!(rep, "- **{}:** {}", topic, statement);
    }

    emit!(rep, "\n## Methodological Boundaries & Scope");
    emit!(rep, "- Regime D timeline: orders commit at `batch_close`, decrypt at `clear_ns = batch_close + decrypt_ms`, and clear using `--d-batch-order {}` (exposure mode: `--d-exposure-mode {}`).", args.d_batch_order.as_str(), args.d_exposure_mode.as_str());
    emit!(rep, "- MM requote threshold: `--requote-thr-sub {}` ({} bp); synthetic sniper probe threshold: `--sniper-thr-sub {}` ({} bp), clip: `{}` (`--sniper-clip 0` disables probe).", args.requote_thr_sub, fmt_bps((args.requote_thr_sub as i128) * Q), args.sniper_thr_sub, fmt_bps((args.sniper_thr_sub as i128) * Q), fmt_usd(args.sniper_clip_usd as i128));
    emit!(rep, "- Single-denominator All-In Net: `--wait-cohort {}` (`all` uses total executed taker volume `tk_n` across all 4 components).", args.wait_cohort.as_str());
    emit!(rep, "- Under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net; inspect `Dropped Takers (USD%)` (0.001% precision) before comparing costs.");
    emit!(rep, "- 95% CI reflects block-resampling drift variance under fixed scenario assumptions; volatility autocorrelation and parameter uncertainty are not covered.");
    emit!(rep, "- Exogenous taker flow; single representative MM; model-derived `s*net` is not directly comparable to real exchange tick spreads.");

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
        &total_proxy_diag,
        &total_book_diag,
        &total_taker_diag,
        nblocks,
        &specs,
        &per_sym,
        &full,
        &html_grids,
        &breakeven_leak_html,
        &sign_check_summary,
    ) {
        eprintln!("Failed to write ablation_report.html: {}", e);
    }

    println!(
        "\nOutputs written: {}/report.md, {}/breakeven.csv, {}/ablation_report.html",
        args.out_dir, args.out_dir, args.out_dir
    );
}