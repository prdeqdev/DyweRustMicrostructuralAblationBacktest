# Dywe Microstructural Ablation Backtest (`dywe-ablation-sim`)

**Deterministic, 100% Fixed-Point (`i64`/`i128`) Causal Orderbook & Execution Replay Engine** benchmarking the **Dywe Exchange** market architecture against industry-standard decentralized CLOB and batch-auction designs on **14 pair-days of historical tick data** from Binance USDⓈ-M Futures (`BTCUSDT` & `ETHUSDT`, Nov 10–16, 2026: **6,384,781 L1 bid/ask mid-price updates** and **11,472,331 directional taker orders**)[cite: 15, 22].

---

## 1. Executive Thesis: Solving the Decentralized Orderbook Trilemma

Institutional market makers, quantitative funds, and vault managers evaluating decentralized perpetual exchanges face an architectural trilemma:

1. **Continuous Transparent CLOB (`Regime A`):** Offers sub-millisecond responsiveness, but leaves resting market-maker (MM) quotes exposed to latency arbitrageurs (`-1.715 bps` 1s toxic markout) and exposes large taker orders (`≥$50k`) to content-based sandwiching[cite: 1, 16]. MMs compensate by widening equilibrium half-spreads to `1.515 bps` (`~3.03 bps` full spread)[cite: 16].
2. **Long-Window Batch Auctions (`Regime B, 1000ms` — dYdX v4 cadence):** Protects MMs from sniping, but under realistic Immediate-or-Cancel (IOC) execution semantics (`pending=drop`), **53.6% of all incoming market orders (`4.09M` orders) are rejected** during MM requote windows, collapsing executed volume from `$167.71B` down to `$77.87B`[cite: 16, 20].
3. **Fast Open Micro-Batches (`Regime B, 70ms` — Hyperliquid cadence):** Prioritizes cancellations (`Cancel-Priority`) to protect MMs, but keeps the mempool and orderbook transparent—leaving large directional sweeps and ERC-4626 copy-vault rebalances visible to MEV searchers prior to execution[cite: 1, 16].

### How Dywe Solves This (`Regime D`)
**Dywe** decouples **liquidity protection** from **order intent privacy** via a dual-lane micro-batch pipeline (`50ms` window)[cite: 1, 16]:
- **Plaintext `Cancel-Header` Lane (Head-of-Batch Priority):** Market-maker cancellations travel with a plaintext routing header and are deterministically sorted to the front of every `50ms` batch (`0ms` cryptographic overhead on cancels), cutting toxic MM adverse selection by **36.3%** vs. transparent FIFO[cite: 1, 16].
- **Threshold-Encrypted Payload Lane (`IBTE BLS12-381`):** Order direction, limit price, and size remain encrypted under a 5-of-7 validator threshold key until `batch_close`, completing decryption at `clear_ns = batch_close + decrypt_ms` (`35ms` baseline / `70ms` stress)[cite: 1, 16].
- **Result:** Dywe preserves **64.8% more executed volume** (`$128.32B`) than `1000ms` auctions and **6.2% more** than `70ms` open batches, at an imperceptible **`+0.093 bps`** privacy cost in a zero-MEV environment—while breaking even for takers as soon as **`≥25%` of large orders (`≥$50k`) suffer `1.0 bps` MEV impact** (`-0.066 bps D<B`)[cite: 16, 20].

---

## 2. Architectural Comparison (4 Isolated Matching Regimes)

Every historical tick and taker order is replayed across four isolated matching engines under identical latency and order-flow streams[cite: 1, 22]:

| Regime | Reference Design | Ordering & Clearing Mechanics | Pre-Trade Privacy | Primary Trade-Off |
| :--- | :--- | :--- | :--- | :--- |
| **Regime A** | **Continuous FIFO** *(Baseline CLOB)* | Strict arrival-time (`arr_ns`) execution; no batching; no cancel priority. | None (Transparent) | Heavy stale-quote sniping (`71,462` hits) forces wide equilibrium spreads (`1.515 bps` half-spread)[cite: 16]. |
| **Regime B** | **Cancel-Priority Batch** *(Hyperliquid / dYdX)* | Plaintext `CANCEL` messages sort ahead of takers within discrete block windows (`70ms`, `200ms`, or `1000ms`)[cite: 1, 16, 18, 20]. | None (Transparent) | Large orders (`≥$50k`) remain exposed to MEV extraction; longer windows (`200–1000ms`) reject `19.5%–53.6%` of IOC takers[cite: 1, 18, 20]. |
| **Regime C** | **Uniform-Price FBA** *(100ms Auction)* | Orders accumulate over `100ms` and clear at a single uniform price; opposing flow nets internally. | None (Transparent) | `100ms` batch hold increases informed waiting drift (`1.114 bps`) and total taker cost (`1.168 bps`)[cite: 16]. |
| **Regime D** | **Dywe Protocol** *(IBTE + Cancel-Prio)* | Plaintext `Cancel-Header` executes at `50ms` batch head; encrypted order payloads decrypt (`35ms`/`70ms`) and clear atomically against the post-cancel book[cite: 1, 16]. | **Threshold BLS12-381** *(10% conservative header-leak modeled)*[cite: 1, 16] | Costs `+0.093 bps` in sterile zero-MEV conditions; strictly cheaper (`D<B`) across all realistic MEV regimes[cite: 16]. |

---

## 3. Simulation & Mathematical Methodology

To prevent floating-point drift, look-ahead bias, or synthetic data artifacts, `dywe-ablation-sim` enforces five strict engineering principles:

### 3.1 100% Fixed-Point Integer Arithmetic (`i64` / `i128`)
- Prices and quantities are scaled by $10^8$ (`PRICE_SCALE = 100_000_000`).
- Relative price moves and markouts are tracked in **sub-bps** (`100 sub = 1.00 bps`) and multiplied by `Q = 1,000` during Phase 2 convolution (`i128`), guaranteeing **bit-identical determinism** across CPU architectures.

### 3.2 Causal Arrival-Time Pipeline & Decryption Timeline
- **Network & Reaction Parameters:** Market-data feed latency `feed_ms = 5ms`, MM quote recomputation `mm_react_ms = 10ms` (swept across `10, 25, 40ms`), sniper reaction `sniper_react_ms = 2ms`, and client-to-sequencer transit `net_ms = 30ms`[cite: 1, 22].
- **Regime D Execution Timeline:** Orders arrive at `arr_ns` inside `[batch_start, batch_close)`, commit at `batch_close`, complete threshold decryption at `clear_ns = batch_close + decrypt_ms`, and clear against the updated post-cancel quote state. Publicly visible quote state (`sim.visible`) updates strictly at `clear_ns`, preventing snipers from observing un-decrypted batch states.

### 3.3 Independent Bid/Ask Mid-Price Proxy (`--mid-mode bidask`)
- When L1 `bookTicker` archives are unavailable, reconstructing mid-price via `trade_price ± half_tick` can create a mechanical correlation where a single trade shifts the mid at the exact timestamp of execution[cite: 1].
- In `v0.3.1`, `--mid-mode bidask` tracks the last buyer-aggressor execution (`last_ask`) and last seller-aggressor execution (`last_bid`) separately, updating `mid = (last_bid + last_ask) / 2` only when opposing sides update[cite: 1].

### 3.4 Realistic IOC Order Semantics (`--pending-policy drop`)
- When an MM cancels a stale quote (`proc.live = false`) and has not yet posted the replacement quote (`New`), incoming taker orders arriving during that requote gap are **immediately expired (`drop`)** rather than held indefinitely[cite: 1]. This exposes the true liquidity cost (missed fills) of slow batch windows (`200ms` and `1000ms`)[cite: 18, 20].

### 3.5 Equilibrium Spread ($s^*_{\text{net}}$) & Exact All-In Taker Cost Identity
For every regime and scenario, the engine solves for the competitive equilibrium half-spread $s^*_{\text{net}}$ where a representative MM breaks even after adverse selection (`drift_q`), MEV inventory loss (`mev_mm_q`), maker rebate (`-0.50 bps`), and hedging OpEx (`0.30 bps`)[cite: 1, 16]:

$$s^*_{\text{net}} = \max\Big(0,\; -\text{Markout}_{1\text{s}} + \text{MEV}_{\text{MM}} - 0.50\text{ bp} + 0.30\text{ bp}\Big)$$

Total execution cost for takers (**All-In Net Cost**) is decomposed into an exact additive identity verified by unit test `t_accounting_reconciliation`[cite: 1, 16]:

$$\text{All-In Net} = \underbrace{[1]\;\text{Spread Paid}}_{\text{effective } s^*_{\text{net}}} + \underbrace{[2]\;\text{Taker Stale}}_{\text{quote staleness}} + \underbrace{[3]\;\text{Informed Wait}}_{\text{batch/decrypt delay}} + \underbrace{[4]\;\text{MEV}_{\text{Taker}}}_{\text{sandwich impact}}$$

### 3.6 Circular-Shift Placebo Control (`--placebo-shifts 600,3600`)
- To prove that observed MM adverse selection is driven by genuine flow-to-price information rather than timestamp or reconstruction artifacts, the simulator circularly shifts taker timestamps by **600 seconds (10 min)** and **3,600 seconds (1 hour)** within each day while leaving the price trajectory untouched[cite: 1, 16].
- Under placebo shifting, **96.3% of the cancel-priority gap vanishes** (`-1.715 bps` $\rightarrow$ `-0.055 bps`), confirming that the empirical signal is endogenous to real order flow[cite: 16].

---

## 4. Pre-Computed Investor & Audit Reports (`results/`)

All four benchmark scenarios are pre-computed and committed under `results/` for immediate inspection without downloading raw CSV archives[cite: 14, 22]:

| Directory | Scenario & CLI Configuration | Key Empirical Takeaway |
| :--- | :--- | :--- |
| **[`results/01_hyperliquid_70ms_ioc/`](./results/01_hyperliquid_70ms_ioc/report.md)** | **Primary Institutional Benchmark** *(vs. 70ms Open CLOB)*<br>`--b-window-ms 70 --pending-policy drop --mid-mode bidask`[cite: 14, 16] | Dywe executes **`$7.46B` more volume** (`$128.32B` vs `$120.86B`), costs only **`+0.093 bps`** at `MEV=0`, and becomes strictly cheaper for takers (`D<B`) at **`≥25%` capture / `1.0 bps` impact** (`-0.066 bps`)[cite: 16]. |
| **[`results/02_hyperliquid_200ms_ioc/`](./results/02_hyperliquid_200ms_ioc/report.md)** | **Consensus Finality Stress** *(vs. 200ms Open CLOB)*<br>`--b-window-ms 200 --pending-policy drop --mid-mode bidask`[cite: 14, 18] | At `200ms` open block latency, Regime B drops `2.24M` IOC orders (`$103.29B` filled vs **`$128.32B`** in Dywe — a **`$25.03B` liquidity advantage** for Dywe)[cite: 18]. |
| **[`results/03_dydx_1000ms_ioc/`](./results/03_dydx_1000ms_ioc/report.md)** | **Long-Window Auction Trade-Off** *(vs. 1000ms dYdX Cadence)*<br>`--b-window-ms 1000 --pending-policy drop --mid-mode bidask`[cite: 14, 20] | A `1000ms` batch rejects **53.6% of all market orders** (`4.09M` dropped takers; `$77.87B` filled), proving that long batches destroy fill rate while Dywe preserves **64.8% more volume**[cite: 16, 20]. |
| **[`results/04_mm_latency_sweep_hold/`](./results/04_mm_latency_sweep_hold/report.md)** | **MM Reaction Latency Sweep & Placebo Control**<br>`--b-window-ms 70 --mm-react 10,25,40 --pending-policy hold`[cite: 14, 22] | Evaluates MM markout resilience as MM reaction slows (`10ms → 25ms → 40ms`) and verifies `96.9%` placebo signal decay[cite: 22]. |

Each directory contains:
- `report.md` — Complete Markdown tables (per-symbol, consolidated, cohort volume reconciliation, placebo control, and break-even frontier)[cite: 14].
- `ablation_report.html` — Interactive dark-mode visual dashboard with Chart.js markout decay trajectories (`0ms → 1000ms`)[cite: 14, 15].
- `breakeven.csv` — Full `6 × 5 × 4 × 4` parameterized MEV scenario grid with 95% block-bootstrap confidence intervals[cite: 1, 14].

---

## 5. Consolidated Benchmark Summary (`v0.3.1`, `pending=drop`, `mid=bidask`)

### 5.1 Execution Volume, Fill Retention & Pure Mechanics (`MEV = 0`, `react = 10ms`)
| Architecture Regime | Executed MM Vol | Dropped IOC Takers | 1s MM Markout | Net Half-Spread $s^*_{\text{net}}$ | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net Cost |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **A: Continuous FIFO** | `$167.71B` | `1,770` | `-1.715 bp` | `1.515 bp` | `1.515 bp` | `-1.264 bp` | `0.849 bp` | **`1.100 bp`**[cite: 16] |
| **B: Cancel-Prio (`70ms`)** | `$120.86B` | `1,402,764` | `-0.983 bp` | `0.783 bp` | `0.783 bp` | `-0.635 bp` | `0.566 bp` | **`0.714 bp`**[cite: 16] |
| **B: Cancel-Prio (`200ms`)** | `$103.29B` | `2,236,417` | `-0.737 bp` | `0.537 bp` | `0.537 bp` | `-0.448 bp` | `0.464 bp` | **`0.554 bp`**[cite: 18] |
| **B: Cancel-Prio (`1000ms`)** | `$77.87B` | `4,086,072` | `-0.459 bp` | `0.259 bp` | `0.259 bp` | `-0.312 bp` | `0.405 bp` | **`0.352 bp`**[cite: 20] |
| **C: Uniform-FBA (`100ms`)** | `$145.84B` | `0` | `-0.861 bp` | `0.661 bp` | `0.575 bp` | `-0.520 bp` | `1.114 bp` | **`1.168 bp`**[cite: 16] |
| **D: Dywe IBTE (`35ms` dec)** | **`$128.32B`** | **`1,154,326`** | **`-1.093 bp`** | **`0.893 bp`** | **`0.893 bp`** | **`-0.835 bp`** | **`0.749 bp`** | **`0.807 bp`**[cite: 16] |

### 5.2 Break-Even Frontier: `All-In(D, 35ms) − All-In(B, 70ms)` (bps)
*Scenario parameters: `50%` MM MEV loss share, `10%` Cancel-Header metadata leakage, `pending=drop`, `mid=bidask`[cite: 15, 16]. Verdict tags reflect 95% joint 3-hour block bootstrap confidence intervals (`56` blocks)[cite: 1, 15]:*
- **`D<B`**: Dywe strictly cheaper for takers (`95% CI < 0`)[cite: 1].
- **`B<D`**: Open Cancel-Priority cheaper (`95% CI > 0`)[cite: 1].
- **`~`**: Statistically indistinguishable (`95% CI` spans zero)[cite: 1].

| Large-Order MEV Capture (`≥$50k`) \ Impact | `0.000 bp` | `0.500 bp` | `1.000 bp` | `2.000 bp` | `3.000 bp` |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **0% Capture** *(Sterile Zero-MEV)* | `+0.093 B<D` | `+0.095 B<D` | `+0.097 B<D` | `+0.102 B<D` | `+0.106 B<D`[cite: 16] |
| **10% Capture** | `+0.093 B<D` | `+0.062 B<D` | `+0.031 ~` | `-0.029 ~` | **`-0.090 D<B`**[cite: 16] |
| **25% Capture** | `+0.093 B<D` | `+0.013 ~` | **`-0.066 D<B`** | **`-0.226 D<B`** | **`-0.386 D<B`**[cite: 16] |
| **50% Capture** | `+0.093 B<D` | **`-0.069 D<B`** | **`-0.231 D<B`** | **`-0.555 D<B`** | **`-0.880 D<B`**[cite: 16] |
| **75% Capture** | `+0.093 B<D` | **`-0.151 D<B`** | **`-0.395 D<B`** | **`-0.884 D<B`** | **`-1.373 D<B`**[cite: 16] |
| **100% Capture** | `+0.093 B<D` | **`-0.233 D<B`** | **`-0.560 D<B`** | **`-1.213 D<B`** | **`-1.867 D<B`**[cite: 16] |

---

## 6. Built-In Verification & Unit Test Suite (9 Invariant Checks)

Before replaying market data, the binary executes 9 automated invariant checks (`cargo test --release`)[cite: 1]:
1. `t_causality` — Verifies zero look-ahead bias: mutating future market ticks after timestamp $T$ leaves all fills prior to $T$ bit-for-bit identical across all 4 regimes[cite: 1].
2. `t_identity` — Verifies that `Regime D` with `decrypt_ms = 0`, `mev = 0`, and `leak = 0` matches `Regime B` bit-for-bit at the `i128` accumulator level[cite: 1].
3. `t_accounting_reconciliation` — Enforces exact conservation of volume (`Taker + StaleSnipe + LateSnipe == Total MM Vol`) and exact additive cost reconciliation (`Spread Paid + Taker Stale + Informed Wait + MEV Taker == All-In Net`)[cite: 1].
4. `t_placebo_breaks_link` — Confirms that circularly shifting taker timestamps destroys flow-price alignment while preserving order counts and notional volume[cite: 1].
5. `t_pending_drop_policy` — Confirms that `drop` expires gap takers whereas `hold` queues them[cite: 1].
6. `t_mev_share_sum`, `t_sniper_protection`, `t_shuffled_zero_spread`, `t_html_escape` — Validates MEV conservation across MM/taker splits, stale-quote sniper attribution, zero-drift randomized flow controls, and HTML output sanitization[cite: 1].

---

## 7. Data Sources & How to Reproduce Locally

### 7.1 Historical Market Data Source (Binance Public Data Vision)
All benchmarks use raw, unsmoothed USDⓈ-M Futures daily archives from **Binance Public Data Vision** (`https://data.binance.vision/data/futures/um/daily/`) for `BTCUSDT` and `ETHUSDT` (`2026-11-10` through `2026-11-16`):
- **`trades/`** (`14` daily CSVs) — Individual L1 trade executions (`id, price, qty, quote_qty, time, is_buyer_maker`) used to reconstruct millisecond L1 bid/ask mid-prices (`--mid-mode bidask`)[cite: 1].
- **`aggTrades/`** (`14` daily CSVs) — Aggregated directional taker order flow (`agg_trade_id, price, quantity, first_trade_id, last_trade_id, transact_time, is_buyer_maker`)[cite: 1].
- *(Optional)* **`bookTicker/`** — If L1 top-of-book quote files (`{SYMBOL}-bookTicker-{YYYY-MM-DD}.csv`) are present in `./data/`, the engine automatically uses exact `(best_bid_price + best_ask_price) / 2` without needing `--allow-reconstructed`[cite: 1].

### 7.2 Step-by-Step Local Reproduction

**Prerequisites:** Rust toolchain (`rustc 1.82+` / `cargo`), `curl`, and `unzip`.

#### Step 1: Download the 7-Day Binance Futures Dataset into `./data/`
```bash
mkdir -p data && cd data && \
for sym in BTCUSDT ETHUSDT; do \
  for d in 10 11 12 13 14 15 16; do \
    for t in aggTrades trades; do \
      curl -s -fLO "[https://data.binance.vision/data/futures/um/daily/$](https://data.binance.vision/data/futures/um/daily/$){t}/${sym}/${sym}-${t}-2026-11-${d}.zip" && \
      echo "Downloaded: ${sym}-${t}-2026-11-${d}"; \
    done; \
  done; \
done && \
unzip -oq "*.zip" && rm -f *.zip && cd ..

```
#### Step 2: Run the 9-Test Causal & Accounting Invariant Suite
```bash
cargo test --release
```
#### Step 3: Regenerate All 4 Benchmark Scenarios
```bash
# 1. Primary Benchmark vs. 70ms Open CLOB (IOC drop, bid/ask mid proxy)
cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 70 --pending-policy drop --mid-mode bidask --out-dir results/01_hyperliquid_70ms_ioc

# 2. Consensus Finality Stress vs. 200ms Open CLOB
cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 200 --pending-policy drop --mid-mode bidask --out-dir results/02_hyperliquid_200ms_ioc

# 3. Long-Window Comparison vs. 1000ms Cadence (dYdX v4)
cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 1000 --pending-policy drop --mid-mode bidask --out-dir results/03_dydx_1000ms_ioc

# 4. MM Reaction Latency Sweep (10ms / 25ms / 40ms) & Placebo Control
cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 70 --mm-react 10,25,40 --pending-policy hold --mid-mode bidask --out-dir results/04_mm_latency_sweep_hold
```

### 7.3 CLI Reference

* `--data-dir <PATH>` — Directory containing extracted Binance CSVs (default: `./data`).
* `--out-dir <PATH>` — Output directory for `report.md`, `ablation_report.html`, and `breakeven.csv` (default: `.`).
* `--window-ms <MS>` — Dywe Regime D micro-batch window duration in milliseconds (default: `50`).
* `--b-window-ms <MS>` — Competitor Regime B block/batch window duration in milliseconds (`70`, `200`, `1000`).
* `--decrypt <LIST>` — Comma-separated post-commit threshold decryption RTTs in milliseconds (default: `0,35,70`).
* `--mm-react <LIST>` — Comma-separated market-maker quote recomputation latencies in milliseconds (default: `10`).
* `--pending-policy <hold|drop>` — Order handling during MM cancel-replace gaps (`drop` = realistic IOC expiry; `hold` = queue until requote).
* `--mid-mode <bidask|trade>` — L1 mid-price reconstruction method from `trades` (`bidask` = separate last-bid/last-ask midpoint; `trade` = `price ± half_tick`).
* `--placebo-shifts <LIST>` — Circular time shifts in milliseconds for placebo alignment verification (default: `600000,3600000`).