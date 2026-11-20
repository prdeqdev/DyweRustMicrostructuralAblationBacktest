
# Dywe Microstructural Ablation Backtest (`dywe-ablation-sim` · v0.3.4)

**Deterministic, Fixed-Point (`i64`/`i128`) Causal Orderbook & Execution Replay Engine** benchmarking the **Dywe Exchange** micro-batch architecture against continuous FIFO and stylized open batch-auction designs across **30 pair-days** of historical Binance USDⓈ-M Futures market data:

- **Primary Multi-Pair Benchmark (`BTCUSDT` & `ETHUSDT`, Nov 10–16, 2026 — 14 pair-days):** `6,384,781` distinct ms-level bid/ask midpoint proxy updates (`84,131,187` raw observations) and `11,472,331` directional aggregated taker executions (`~$168.2B` volume).
- **Native L1 `bookTicker` Benchmark (`BTCUSDT`, `ETHUSDT` & `SOLUSDT`, Mar 28–30, 2024 — 9 pair-days):** `74,023,564` raw L1 quote rows deduplicated to `3,269,988` distinct ms-level mid changes (median inter-update interval `3 ms`) and `4,244,321` directional taker executions (`~$64.4B` volume), paired with an exact same-day `BidAskProxy` validation run.
- **Altcoin / Mid-Cap Expansion (`SOLUSDT`, Nov 10–16, 2026 — 7 pair-days):** `636,535` distinct ms-level bid/ask midpoint proxy updates (`15,365,789` raw observations) and `1,997,052` directional taker executions (`~$15.1B` volume with synthetic probe; `$12.63B` pure historical taker flow with `--sniper-clip 0`).

---

## 1. Executive Thesis: Empirical Mechanics vs. Scenario Counterfactuals

Decentralized perpetual orderbooks face an architectural trade-off between **adverse-selection protection**, **execution responsiveness**, and **pre-trade order privacy**. To maintain strict scientific separation, this repository divides findings into **(A) Measured Mechanics at `MEV = 0`** and **(B) Parameterized Counterfactual MEV Scenarios**:

### A. Measured Mechanics at `MEV = 0` (Exogenous Historical Taker Flow)

1. **Continuous Transparent FIFO (`Regime A`):** Exposes resting market-maker (MM) quotes to stale-quote adverse selection (`-1.717 bps` 1s markout on `BTC/ETH` proxy; `-1.312 bps` on native L1 `bookTicker`), requiring a competitive equilibrium half-spread of `s*net = 1.517 bps` (`1.112 bps` on native L1) under baseline `-0.50 bps` maker rebate and `0.30 bps` hedging OpEx.
2. **Open Priority Micro-Batch (`Regime B` — `70ms` / `200ms` / `1000ms`, `--b-batch-order hl`):** Processes `New (ALO/post-only) -> Cancel -> Taker` inside discrete batch windows, cutting 1s MM markout by **48.5%** (`-0.885 bps`, `s*net = 0.685 bps` on `BTC/ETH`) and **49.2%** (`-0.667 bps`, `s*net = 0.467 bps` on native L1) with `0.0%` dropped taker volume. However, all orders sit in a transparent mempool prior to batch execution.
3. **Uniform-Price Frequent Batch Auction (`Regime C`, `100ms`):** Clears opposing flow internally at a single auction price (`-0.861 bps` 1s markout on `BTC/ETH`), but incurs `0.895 bps` volume-weighted waiting drift and `0.950 bps` All-In Net taker cost.
4. **Dywe Threshold-Encrypted Micro-Batch (`Regime D`, `50ms` batch + `35ms` threshold decryption, `--d-batch-order hl`):**
   - **Vs. Continuous FIFO (`Regime A`):** Cuts 1s MM toxic markout by **38.9%** on `BTC/ETH` (`-1.049` vs. `-1.717 bps`), **40.4%** on native L1 `bookTicker` (`-0.782` vs. `-1.312 bps`), and **23.8%** on pure organic `SOLUSDT` flow without synthetic sniper probes (`-1.533` vs. `-2.011 bps`), with `0.0%` dropped USD taker volume under `pending=drop`.
   - **Vs. `70ms` Open Priority Batch (`Regime B, hl`) — The Honest Latency Trade-Off:** Because threshold decryption adds `35ms` of quote exposure (`clear_ns = batch_close + 35ms`), Regime D exhibits **23.9%–24.6% wider MM equilibrium half-spreads** than Regime B (`0.849` vs. `0.685 bps` on `BTC/ETH`; `0.582` vs. `0.467 bps` on native L1). Under exogenous taker flow at `MEV = 0`, higher spread paid (`+0.115` to `+0.164 bps`) and higher waiting drift (`+0.047` to `+0.077 bps`) are offset by larger taker staleness capture (`-0.157` to `-0.242 bps`), yielding **no statistically meaningful difference in taker All-In Net cost** (`-0.002 bps [~]` on `BTC/ETH`; `+0.005 bps` on native L1; `+0.002 bps [~]` on `SOLUSDT`).

### B. Parameterized MEV Counterfactuals (`capture > 0%`)

In this model, **any net taker cost advantage of Dywe over an open `70ms` batch (`D < B`) arises strictly from the counterfactual MEV extraction grid** (because Binance USDⓈ-M Futures has no public mempool or on-chain sandwich MEV):

- Under baseline scenario assumptions (`50%` MM MEV loss allocation, `10%` residual metadata leakage on cancel-containing batches, `+0.50 bps` maker rebate, `0.30 bps` OpEx), Regime B exposes all large orders (`≥$50k`: `71.2%` of `BTC/ETH` volume; `68.2%` of native L1 volume) to `capture_ppm`, whereas Regime D exposes only large orders arriving inside batches that contain an MM cancel (`32.5%` on `BTC/ETH`; `26.7%` on native L1) to `leak_ppm`.
- By construction of this exposure ratio (`1.00 : 0.325` on `BTC/ETH`; `1.00 : 0.267` on L1), when `capture_ppm = leak_ppm = 10%` and `impact = 0.5 bps`, Dywe becomes cheaper for takers (`D < B` at 95% CI: `-0.026 bps` on `BTC/ETH`, `-0.019 bps` on native L1), and reduces All-In taker cost by **`16.8%–35.9%`** (`BTC/ETH`) and **`22.3%–48.2%`** (native L1) at `25%–50%` capture / `1.0 bps` impact. Higher side-channel leakage assumptions (`leak_ppm = 25%–50%`, included in `breakeven.csv`) shift the break-even threshold accordingly.

---

## 2. Architectural Comparison & Unified Threat Model

Every historical observation and directional taker execution is replayed across four isolated matching engines under identical latency and reaction parameters:

| Regime | Reference Architecture | Ordering & Clearing Mechanics | Pre-Trade Privacy | Primary Trade-Off |
| :--- | :--- | :--- | :--- | :--- |
| **Regime A** | **Continuous FIFO** *(Baseline CLOB)* | Strict arrival-time (`arr_ns`) execution; no batching; no cancel priority. | None (Transparent) | High 1s MM toxic markout (`-1.717 bps` proxy / `-1.312 bps` L1) forces wide equilibrium half-spreads (`1.517` / `1.112 bps`). |
| **Regime B** | **Open Priority Batch** *(`70ms` / `200ms` / `1000ms`, `hl` order)* | `New (ALO) -> Cancel -> Taker/Sniper` inside discrete batch windows. | None (Transparent) | Lowest MM markout among priority batches (`0.685` / `0.467 bps` half-spread), but large orders (`68%–71%` of volume) remain exposed in the mempool. |
| **Regime C** | **Uniform-Price FBA** *(`100ms` Auction)* | Orders accumulate over `100ms` and clear at a single uniform price; opposing flow nets internally. | None (Transparent) | `100ms` batch hold increases waiting drift (`0.895 bps` proxy / `0.722 bps` L1) and All-In Net cost (`0.950` / `0.671 bps`). |
| **Regime D** | **Dywe Protocol** *(IBTE + `50ms` Micro-Batch)* | All order types (`New ALO`, `Cancel`, `Taker`) are encrypted pre-commit, decrypt in memory at `batch_close + 35ms`, and sort (`hl` default; `cancel-first` stress). | **Full Pre-Commit IBTE (`BLS12-381`)** *(0%–50% side-channel leak grid)* | `+24%` wider MM half-spread vs. `B (70ms)` due to `35ms` decrypt wait; taker All-In parity at `MEV=0`; `D < B` under counterfactual MEV scenarios. |

### Unified Post-Decrypt Threat Model (Why No Plaintext Cancel Headers Are Needed)

- **Full Pre-Commit Encryption:** In Dywe's architecture, *all* order payloads (`New ALO`, `Cancel`, and `Taker`) are threshold-encrypted client-side under `BLS12-381` until `batch_close`.
- **Post-Decrypt In-Memory Priority (`--d-batch-order hl`):** Because the 5-of-7 validator quorum completes threshold decryption inside matching-engine memory at `clear_ns = batch_close + decrypt_ms` *before* any matching occurs, the engine applies deterministic `New (ALO) -> Cancel -> Taker` ordering in memory without leaking `ALO`/`IOC` flags or cancel headers prior to sequence lock.
- **Role of `leak_ppm`:** The `leak_ppm` parameter (`0%`, `10%` baseline sensitivity, `25%`, `50%`) models potential residual side-channel or traffic-analysis leakage on batches containing MM cancels, rather than plaintext cancel headers.
- **Calibration Disclaimer:** Regimes `B (70ms)`, `B (200ms)`, and `B (1000ms)` are stylized execution models calibrated to representative batching intervals. They do not constitute protocol-level or byte-for-byte reproductions of Hyperliquid or dYdX matching engines.

---

## 3. Simulation & Mathematical Methodology

### 3.1 Separation of Empirical Data, Mechanical Parameters & Scenario Assumptions

1. **Historical Component (Measured):** Raw Binance USDⓈ-M Futures `bookTicker`, `trades`, and directional `aggTrades` across 30 pair-days.
2. **Mechanical Component (Parameterized):** Feed latency (`5ms`), MM quote recomputation (`10, 25, 40ms`), synthetic latency-arb probe (`2ms` reaction, `2.0 bps` trigger, `$25,000` clip via `--sniper-clip`), client-to-sequencer transit (`30ms`), natural sub-ms jitter (`0–999,999 ns`), batch windows (`50, 70, 100, 200, 1000ms`), and post-commit decryption RTTs (`0, 35, 70ms`).
3. **Scenario Component (Parameterized Grid):** Ambient MEV capture rate (`0%–100%`), per-order price impact (`0.0–3.0 bps`), MM loss allocation (`50%` baseline), and residual metadata leakage (`0%–50%`, automatically clamped to `0%` when ambient `capture = 0%`).
4. **Derived Component (Calculated):** Competitive equilibrium half-spread (`s*net`) and All-In Net Taker Execution Cost.

### 3.2 Fixed-Point Integer Arithmetic (`i64` / `i128`) & Determinism

- Prices and quantities are scaled by `10^8` (`PRICE_SCALE = 100_000_000`, `QTY_SCALE = 100_000_000`) using widening `i128` arithmetic to prevent overflow.
- Relative moves and markouts use integer **sub-bps** (`100 sub = 1.00 bps`, truncating toward zero) and `Q = 1,000` in Phase 2 (`i128`).
- Parallel workers compute isolated per-day, per-block accumulators merged in deterministic `(symbol, day, block)` order.

### 3.3 Native L1 `bookTicker` Density & Same-Day Proxy Validation (`--mid-source book|proxy`)

- **Native L1 `bookTicker` Archive Availability & Density:** No daily USDⓈ-M `bookTicker` archives were found on `data.binance.vision` after `2024-03-30` (consistent with third-party Binance archive indices such as `nekojiru-binance`). Across the 9 pair-days of March 28–30, 2024 (`BTCUSDT`, `ETHUSDT`, `SOLUSDT`), raw `bookTicker` files contain **`74,023,564` raw quote rows** (`~8.22M` raw updates/pair-day). Because `mid_at()` is a step function of `(best_bid + best_ask) / 2`, `push_mid_tracked()` collapses `4,312,026` same-millisecond updates and skips `66,441,550` BBO quantity-only updates where the midpoint did not move, retaining **`3,269,988` distinct ms-level mid changes** with a **median inter-update interval of `3 ms`** (`2 ms` on `SOLUSDT`, `5–7 ms` on `BTCUSDT`/`ETHUSDT`).
- **Same-Day Direct Comparison (`book` vs. `proxy` on Mar 28–30, 2024):** Running `BidAskProxy` on the *exact same* March 28–30, 2024 dataset (`22,327,383` trade observations, `3.3%` crossed, mean valid `(ask - bid)` spread `0.758 bps`) confirms close alignment with native `bookTicker`:

| Metric (Mar 28–30, 2024 · `BTC+ETH+SOL`, 9 pair-days) | Native L1 `bookTicker` (`--mid-source book`) | Same-Day `BidAskProxy` (`--mid-source proxy`) | Delta (`Proxy − Book`) |
| :--- | :---: | :---: | :---: |
| **Regime A (FIFO) 1s MM Markout** | `-1.312 bps` `[-1.425, -1.216]` | `-1.325 bps` `[-1.439, -1.224]` | `-0.013 bps` |
| **Regime B (`70ms`, `hl`) 1s MM Markout** | `-0.667 bps` (`s*net = 0.467`) | `-0.796 bps` (`s*net = 0.596`) | `-0.129 bps` |
| **Regime D (`50ms`, `35ms` dec, `hl`) 1s MM Markout** | `-0.782 bps` (`s*net = 0.582`) | `-0.876 bps` (`s*net = 0.676`) | `-0.094 bps` |
| **Zero-MEV Taker All-In Delta (`D − B`)** | `+0.005 bps` (`B<D`) | `+0.002 bps` (`B<D`) | `-0.003 bps` |
| **MEV Scenario Delta (`10%` capture / `0.5 bps` impact)** | `-0.019 bps` (`D<B`) | `-0.024 bps` (`D<B`) | `-0.005 bps` |
| **MEV Scenario Delta (`25%` capture / `1.0 bps` impact)** | `-0.145 bps` (`D<B`) | `-0.152 bps` (`D<B`) | `-0.007 bps` |

- **2026 Proxy Diagnostics (`--mid-mode bidask`):**
  - `BTC/ETH` (14 pair-days, Nov 2026): `84,131,187` observations, `0.8%` crossed, mean valid `(ask - bid)` spread `0.708 bps` (reflecting cross-trade staleness of the passive side rather than true resting BBO tick width).
  - `SOLUSDT` (7 pair-days, Nov 2026): `15,365,789` observations, `0.0%` crossed (`1,305` obs), mean valid spread `1.903 bps`.

### 3.4 Equilibrium Spread (`s*net`), Single-Denominator All-In Identity & Placebo Control

For every regime and scenario, the engine solves for the competitive equilibrium half-spread `s*net` (configurable via `--rebate-sub`, default `+0.50 bps` revenue; `--opex-sub`, default `0.30 bps` hedging cost):

$$
s^*_{\text{net}} = \max\Big(0,\; -\text{Markout}_{1\text{s}} + \text{MEV}_{\text{MM}} - \text{Rebate} + \text{OpEx}\Big)
$$

Under `--wait-cohort all` (default), all four components of **All-In Net Cost** share a single volume denominator (`tk_n`), guaranteeing exact linear additivity:

$$
\text{All-In Net} =
\underbrace{[1]\;\text{Spread Paid}}_{\text{effective } s^*_{\text{net}}}
+
\underbrace{[2]\;\text{Taker Stale}}_{\text{quote staleness}}
+
\underbrace{[3]\;\text{Wait (All)}}_{\text{batch/decrypt drift}}
+
\underbrace{[4]\;\text{MEV}_{\text{Taker}}}_{\text{sandwich impact}}
$$

- **Joint Block Bootstrap:** Synchronized 3-hour blocks resampled jointly across pairs (`56` blocks for 7-day runs, `24` blocks for 3-day L1 run; `1,000` draws, `95% CI`).
- **Circular-Shift Placebo Control (`600s` / `3600s`):** Shifting taker timestamps while holding the price path fixed destroys `96.8%` of Regime A's 1s markout on `BTC/ETH` (`-1.717` → `-0.055 bps`), `97.3%` on native L1 `bookTicker` (`-1.312` → `-0.036 bps`), and `99.7%` on pure organic `SOLUSDT` flow (`-2.011` → `-0.006 bps` under `--sniper-clip 0`). This confirms that simulated markout arises from temporal alignment between order flow and subsequent price moves rather than static drift.
- **Simulated Latency-Arb Probe (`StaleSnipe`):** Stale-quote sniping hits (`71,633` on `BTC/ETH` FIFO; `35,107` on L1 FIFO) are generated by a parameterized deterministic probe (`2.0 bps` trigger, `2ms` reaction, `$25,000` clip), not directly observed order IDs. Because the clip is fixed in USD, the probe accounts for `1.1%` of `BTC/ETH` FIFO volume (`1.4%` on L1) vs. `15.0%` on thinner `SOLUSDT` (`$2.26B / $15.09B`), which mechanically drives SOL's `-0.232 bps` FIFO-vs-Dywe All-In gap at `MEV = 0`. Disabling the probe (`--sniper-clip 0`) isolates pure historical `SOLUSDT` taker flow (`-2.011 bps` FIFO vs `-1.533 bps` Dywe 1s MM markout).

---

## 4. Pre-Computed Audit Reports (`results/`)

All 8 benchmark scenarios are committed under `results/` for inspection:

| Directory | Scenario & Data Source | Key Takeaway |
| :--- | :--- | :--- |
| [`results/B70_hl_vs_D50_hl_drop/`](./results/B70_hl_vs_D50_hl_drop/report.md) | **Primary Benchmark (`BTC+ETH`, 14d, proxy)**<br>B `70ms` `hl` vs D `50ms` `hl`, `drop`, `wait=all` | `0.0%` dropped USD in both B and D. Zero-MEV taker All-In delta is **`-0.002 bps (~)`** (`s*net`: `0.849` vs `0.685 bps`); under MEV scenarios, `D<B` from **`10%` / `0.5 bps`** (`-0.026 bps`). |
| [`results/L1_202403_book/`](./results/L1_202403_book/report.md) | **Native L1 `bookTicker` (`BTC+ETH+SOL`, 9d)**<br>B `70ms` `hl` vs D `50ms` `hl`, `drop`, `--mid-source book` | `74.02M` raw L1 quote rows (`3ms` median dt). Cuts 1s MM markout by **`-40.4%`** vs FIFO; zero-MEV taker delta **`+0.005 bps`** (`s*net`: `0.582` vs `0.467 bps`); `D<B` from **`10%` / `0.5 bps`** (`-0.019 bps`). |
| [`results/L1_202403_proxy/`](./results/L1_202403_proxy/report.md) | **Same-Day `BidAskProxy` Control (`BTC+ETH+SOL`, 9d)**<br>B `70ms` `hl` vs D `50ms` `hl`, `drop`, `--mid-source proxy` | Same Mar 28–30, 2024 dates as L1: FIFO 1s markout **`-1.325 bps`** vs `-1.312 bps` on native `bookTicker` (`0.013 bps` difference); zero-MEV delta `+0.002 bps`. |
| [`results/SOL_B70_hl_vs_D50_hl_drop/`](./results/SOL_B70_hl_vs_D50_hl_drop/report.md) | **Altcoin Expansion (`SOLUSDT`, 7d, proxy + probe)**<br>B `70ms` `hl` vs D `50ms` `hl`, `drop`, `wait=all` | Matches B at `MEV=0` (`+0.002 bps ~`) and wins (`D<B`) from **`10%` / `0.5 bps`** (`-0.010 bps`); `-0.232 bps` vs FIFO reflects the `15.0%` fixed `$25k` sniper probe share on SOL. |
| [`results/SOL_B70_hl_vs_D50_hl_drop_nosniper/`](./results/SOL_B70_hl_vs_D50_hl_drop_nosniper/report.md) | **Altcoin Pure Taker Flow (`SOLUSDT`, 7d, `--sniper-clip 0`)**<br>B `70ms` `hl` vs D `50ms` `hl`, `drop`, no probe | Isolates pure organic `SOLUSDT` flow (`$12.63B`): Dywe cuts 1s MM markout by **`-23.8%`** vs FIFO (`-1.533` vs `-2.011 bps`), `99.7%` disappears under 600s placebo, and `D<B` from **`10%` / `0.5 bps`** (`-0.008 bps`). |
| [`results/B200_hl_vs_D50_hl_drop/`](./results/B200_hl_vs_D50_hl_drop/report.md) | **200ms Window Stress (`BTC+ETH`, 14d, proxy)**<br>B `200ms` `hl` vs D `50ms` `hl`, `drop`, `wait=all` | `0.0%` dropped USD in both regimes. Zero-MEV delta is **`-0.008 bps (~)`**; `D<B` from **`10%` / `0.5 bps`** (`-0.032 bps`). |
| [`results/B1000_hl_vs_D50_hl_drop/`](./results/B1000_hl_vs_D50_hl_drop/report.md) | **1000ms Window Stress (`BTC+ETH`, 14d, proxy)**<br>B `1000ms` `hl` vs D `50ms` `hl`, `drop`, `wait=all` | Zero-MEV delta is **`+0.019 bps (~)`**; `D<B` from **`10%` / `2.0 bps`** (`-0.075 bps`) or **`25%` / `0.5 bps`** (`-0.057 bps`). |
| [`results/B70_hl_vs_D50_cancelfirst_hold_sweep/`](./results/B70_hl_vs_D50_cancelfirst_hold_sweep/report.md) | **MM Latency Sweep & `cancel-first` Stress (`BTC+ETH`, 14d)**<br>B `hl` vs D `cancel-first`, `hold` | Even if D is restricted to `cancel-first` under equal-volume `hold`, zero-MEV taker delta is **`+0.005 bps (B<D)`** and `D<B` from **`10%` / `0.5 bps`** (`-0.017 bps`). |

Each directory contains `report.md`, interactive `ablation_report.html` (Chart.js), and full `breakeven.csv` sensitivity grids.

---

## 5. Consolidated Benchmark Summary (`pending=drop`, `wait=all`)

### 5.1 Primary Benchmark (`BTC+ETH`, 14 Pair-Days, `MEV = 0`, `react = 10ms`)

| Architecture Regime | Executed MM Vol | Dropped Takers (USD%) | 1s MM Markout | Net Half-Spread `s*net` | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net | Probe Hit/Miss |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **A: Continuous FIFO** | `$167.72B` | `1,595 (0.0%)` | `-1.717 bp` | `1.517 bp` | `1.517 bp` | `-1.266 bp` | `0.677 bp` | **`0.928 bp`** | `71,633 / 97` |
| **B: Batch `70ms` (`hl`)** | `$167.90B` | `1,797 (0.0%)` | `-0.885 bp` | `0.685 bp` | `0.685 bp` | `-0.636 bp` | `0.874 bp` | **`0.924 bp`** | `6,308 / 60` |
| **B: Batch `200ms` (`hl`)** | `$168.10B` | `884 (0.0%)` | `-0.553 bp` | `0.353 bp` | `0.353 bp` | `-0.368 bp` | `0.946 bp` | **`0.930 bp`** | `1,900 / 21` |
| **B: Batch `1000ms` (`hl`)** | `$168.18B` | `511 (0.0%)` | `-0.236 bp` | `0.036 bp` | `0.036 bp` | `-0.182 bp` | `1.049 bp` | **`0.903 bp`** | `251 / 6` |
| **C: Uniform-FBA (`100ms`)** | `$145.80B` | `1,650 (0.0%)` | `-0.861 bp` | `0.661 bp` | `0.575 bp` | `-0.520 bp` | `0.895 bp` | **`0.950 bp`** | `4,223 / 43` |
| **D: Dywe IBTE `50ms` (`35ms` dec, `hl`)** | **`$168.20B`** | **`2,153 (0.0%)`** | **`-1.049 bp`** | **`0.849 bp`** | **`0.849 bp`** | **`-0.878 bp`** | **`0.951 bp`** | **`0.922 bp`** | **`8,523 / 79`** |

### 5.2 Native L1 `bookTicker` Benchmark (`BTC+ETH+SOL`, 9 Pair-Days, `MEV = 0`, `v0.3.4`)

| Architecture Regime | Executed MM Vol | Dropped Takers (USD%) | 1s MM Markout | Net Half-Spread `s*net` | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net | Probe Hit/Miss |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **A: Continuous FIFO** | `$64.11B` | `216 (0.0%)` | `-1.312 bp` | `1.112 bp` | `1.112 bp` | `-1.068 bp` | `0.626 bp` | **`0.670 bp`** | `35,107 / 48` |
| **B: Batch `70ms` (`hl`)** | `$64.26B` | `322 (0.0%)` | `-0.667 bp` | `0.467 bp` | `0.467 bp` | `-0.517 bp` | `0.701 bp` | **`0.651 bp`** | `3,357 / 26` |
| **C: Uniform-FBA (`100ms`)** | `$59.46B` | `269 (0.0%)` | `-0.622 bp` | `0.422 bp` | `0.390 bp` | `-0.441 bp` | `0.722 bp` | **`0.671 bp`** | `2,180 / 22` |
| **D: Dywe IBTE `50ms` (`35ms` dec, `hl`)** | **`$64.36B`** | **`572 (0.0%)`** | **`-0.782 bp`** | **`0.582 bp`** | **`0.582 bp`** | **`-0.674 bp`** | **`0.748 bp`** | **`0.656 bp`** | **`4,517 / 46`** |

### 5.3 Break-Even Frontier: `All-In(D, 35ms, hl) − All-In(B, 70ms, hl)` (bps)

*Left value: Primary `BTC/ETH` (14d proxy). Right value in brackets: Native L1 `bookTicker` (`BTC/ETH/SOL`, 9d). Scenario parameters: `50%` MM MEV share, `10%` residual metadata leakage (`0%` enforced when `capture = 0%`), `+0.50 bps` rebate, `0.30 bps` OpEx. **All rows with capture > 0% are parameterized counterfactual scenarios, not measurements.***

| Large-Order MEV Capture (`≥$50k`) \ Impact | `0.000 bp` | `0.500 bp` | `1.000 bp` | `2.000 bp` | `3.000 bp` |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **0% Capture** *(Sterile Zero-MEV)* | `-0.002 ~` `[+0.005 B<D]` | `-0.002 ~` `[+0.005 B<D]` | `-0.002 ~` `[+0.005 B<D]` | `-0.002 ~` `[+0.005 B<D]` | `-0.002 ~` `[+0.005 B<D]` |
| **10% Capture** | `-0.002 ~` `[+0.005 B<D]` | **`-0.026 D<B`** `[-0.019]` | **`-0.049 D<B`** `[-0.044]` | **`-0.097 D<B`** `[-0.093]` | **`-0.144 D<B`** `[-0.143]` |
| **25% Capture** | `-0.002 ~` `[+0.005 B<D]` | **`-0.078 D<B`** `[-0.070]` | **`-0.155 D<B`** `[-0.145]` | **`-0.309 D<B`** `[-0.296]` | **`-0.462 D<B`** `[-0.447]` |
| **50% Capture** | `-0.002 ~` `[+0.005 B<D]` | **`-0.167 D<B`** `[-0.154]` | **`-0.332 D<B`** `[-0.314]` | **`-0.661 D<B`** `[-0.634]` | **`-0.991 D<B`** `[-0.953]` |
| **75% Capture** | `-0.002 ~` `[+0.005 B<D]` | **`-0.255 D<B`** `[-0.239]` | **`-0.508 D<B`** `[-0.483]` | **`-1.014 D<B`** `[-0.971]` | **`-1.520 D<B`** `[-1.460]` |
| **100% Capture** | `-0.002 ~` `[+0.005 B<D]` | **`-0.343 D<B`** `[-0.323]` | **`-0.684 D<B`** `[-0.652]` | **`-1.367 D<B`** `[-1.309]` | **`-2.050 D<B`** `[-1.966]` |

---

## 6. Built-In Verification & Unit Test Suite (15 Pre-Flight Checks + 16 Unit Tests)

Executed via `cargo test --release` (all 16 unit tests pass in `<0.1s`) and automatically on pre-flight (15 synthetic invariant checks):

1. `t_causality` — Zero look-ahead bias: mutating future ticks after `T` leaves all fills before `T` bit-for-bit identical across all 4 regimes.
2. `t_identity` — `Regime D (dec=0, cancel-first)` equals `Regime B (cancel-first)` bit-for-bit at the `i128` accumulator level.
3. `t_d_post_decrypt_hl_order` — `Regime D (dec=0, hl)` equals `Regime B (hl)` bit-for-bit and eliminates artificial same-batch cancel-gap drops.
4. `t_hl_batch_order` — Verifies Hyperliquid-style `New (ALO) -> Cancel -> Taker` ordering removes dead gaps vs. `cancel-first`.
5. `t_accounting_reconciliation` — Enforces exact volume conservation and exact additive `All-In Net` component identity.
6. `t_wait_cohort_all_identity` — Validates single-denominator (`tk_n`) linear additivity under `--wait-cohort all` vs. `large`.
7. `t_fba_sniper_attribution` — Confirms late FBA snipers arriving after same-batch MM requotes are attributed to `late_fresh` (`0` false stale hits).
8. `t_placebo_breaks_link` — Confirms circular timestamp shifting destroys flow-price alignment while preserving order counts and notional.
9. `t_book_ticker_diag` — Validates raw `bookTicker` row counting, same-ms/unchanged-mid deduplication counters, and exact 1-ms histogram median inter-update interval.
10. `t_zero_capture_zero_mev` — Enforces zero MEV extraction when ambient `capture = 0%`, regardless of `leak_ppm`.
11. `t_pending_drop_policy`, `t_bidask_mid_proxy`, `t_mev_share_sum`, `t_sniper_protection`, `t_shuffled_zero_spread`, `t_html_escape` — Validates IOC drop vs. hold semantics, `BidAskProxy` normal/crossed diagnostics, MEV share conservation, stale-snipe protection, randomized-flow zero-bias controls, and HTML sanitization.

---

## 7. Data Sources & How to Reproduce Locally

### 7.1 Historical Market Data Source

Raw daily USDⓈ-M Futures archives from **Binance Public Data Vision** (`https://data.binance.vision/data/futures/um/daily/`):

- **2026 Dataset (`./data/`):** `BTCUSDT`, `ETHUSDT`, `SOLUSDT` (`2026-11-10` through `2026-11-16`: `trades/` and `aggTrades/`).
- **Native L1 & Same-Day Proxy Dataset (`./data_l1/`):** `BTCUSDT`, `ETHUSDT`, `SOLUSDT` (`2024-03-28` through `2024-03-30`: `bookTicker/`, `trades/`, and `aggTrades/`).

### 7.2 Step-by-Step Local Reproduction

**Prerequisites:** Rust toolchain (`rustc 1.82+` / `cargo`), `curl`, `shasum`, and `unzip`.

#### Step 1: Download & Verify Checksums

```bash
set -e
BASE="https://data.binance.vision/data/futures/um/daily"

# A) 7-day 2026 dataset (BTCUSDT, ETHUSDT, SOLUSDT)
mkdir -p data && cd data
for sym in BTCUSDT ETHUSDT SOLUSDT; do
  for day in 10 11 12 13 14 15 16; do
    for type in aggTrades trades; do
      file="${sym}-${type}-2026-11-${day}.zip"
      curl --fail --silent --show-error --location "${BASE}/${type}/${sym}/${file}" -o "${file}"
      curl --fail --silent --show-error --location "${BASE}/${type}/${sym}/${file}.CHECKSUM" -o "${file}.CHECKSUM"
      shasum -a 256 -c "${file}.CHECKSUM"
      unzip -oq "${file}" -d . && rm -f "${file}" "${file}.CHECKSUM"
    done
  done
done
cd ..

# B) 3-day Native L1 bookTicker + same-day trades dataset (Mar 28-30, 2024)
mkdir -p data_l1 && cd data_l1
for sym in BTCUSDT ETHUSDT SOLUSDT; do
  for day in 28 29 30; do
    for type in bookTicker trades aggTrades; do
      file="${sym}-${type}-2024-03-${day}.zip"
      curl --fail --silent --show-error --location "${BASE}/${type}/${sym}/${file}" -o "${file}"
      curl --fail --silent --show-error --location "${BASE}/${type}/${sym}/${file}.CHECKSUM" -o "${file}.CHECKSUM"
      shasum -a 256 -c "${file}.CHECKSUM"
      unzip -oq "${file}" -d . && rm -f "${file}" "${file}.CHECKSUM"
    done
  done
done
cd ..
```

#### Step 2: Run the 16-Test Suite & Regenerate All 8 Benchmark Reports

```bash
cargo test --release
mkdir -p results

# 1. Primary BTC+ETH benchmark (70ms)
cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 70 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/B70_hl_vs_D50_hl_drop

# 2. 200ms open-batch stress
cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 200 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/B200_hl_vs_D50_hl_drop

# 3. 1000ms open-batch stress
cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 1000 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/B1000_hl_vs_D50_hl_drop

# 4. MM reaction latency sweep (10/25/40ms) & cancel-first stress
cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 70 --b-batch-order hl --d-batch-order cancel-first --mm-react 10,25,40 --pending-policy hold --wait-cohort all --mid-mode bidask --out-dir results/B70_hl_vs_D50_cancelfirst_hold_sweep

# 5. Altcoin expansion (SOLUSDT with synthetic probe)
cargo run --release -- --threads 6 --allow-reconstructed --symbols SOLUSDT --b-window-ms 70 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/SOL_B70_hl_vs_D50_hl_drop

# 6. Altcoin pure organic flow control (SOLUSDT without synthetic sniper probe)
cargo run --release -- --threads 6 --allow-reconstructed --symbols SOLUSDT --sniper-clip 0 --b-window-ms 70 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/SOL_B70_hl_vs_D50_hl_drop_nosniper

# 7. Native L1 bookTicker benchmark (Mar 28-30, 2024)
cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --b-window-ms 70 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --out-dir results/L1_202403_book

# 8. Same-day BidAskProxy control on Mar 28-30, 2024
cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source proxy --mid-mode bidask --b-window-ms 70 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --out-dir results/L1_202403_proxy
```

### 7.3 CLI Reference

| Argument | Description |
| :--- | :--- |
| `--data-dir <PATH>` | Directory containing extracted Binance CSVs (default: `./data`). |
| `--out-dir <PATH>` | Output directory for `report.md`, `ablation_report.html`, and `breakeven.csv` (default: `.`). |
| `--symbols <LIST>` | Comma-separated symbols to replay (default: `BTCUSDT,ETHUSDT`). |
| `--start <YYYY-MM-DD>` / `--days <N>` | Start date and number of consecutive days to replay (default: `2026-11-10`, `7`). |
| `--window-ms <MS>` | Dywe Regime D micro-batch window duration in ms (default: `50`). |
| `--b-window-ms <MS>` | Competitor Regime B batch window duration in ms (`70`, `200`, `1000`). |
| `--b-batch-order <hl\|cancel-first>` | Intra-batch ordering for Regime B (default: `hl`). |
| `--d-batch-order <hl\|cancel-first>` | Post-decrypt in-memory ordering for Regime D (default: `hl`). |
| `--decrypt <LIST>` | Post-commit threshold decryption RTTs in ms (default: `0,35,70`). |
| `--mm-react <LIST>` | MM quote recomputation latencies in ms (default: `10`). |
| `--pending-policy <hold\|drop>` | Handling of takers arriving during an MM requote gap (default: `hold`). |
| `--wait-cohort <all\|large>` | Denominator for waiting drift in All-In Net (default: `all`). |
| `--mid-source <auto\|book\|proxy>` | Force native `bookTicker`, trade-derived `proxy`, or `auto` fallback (default: `auto`). |
| `--mid-mode <bidask\|trade>` | Trade-derived midpoint proxy formula when using `proxy` (default: `bidask`). |
| `--sniper-clip <USD>` | Notional clip size for the synthetic latency-arb probe; `0` disables probe (default: `25000`). |
| `--rebate-sub <INT>` / `--opex-sub <INT>` | MM maker rebate & hedging OpEx in sub-bps (`100 sub = 1.00 bps`; defaults: `50`, `30`). |
| `--large-usd <INT>` | Notional threshold in USD for MEV-eligible large orders (default: `50000`). |
| `--placebo-shifts <LIST>` | Circular timestamp shifts in ms for placebo verification (default: `600000,3600000`). |

---

## 8. Repository Structure

```text
Dywe_Rust_Microstructural_Ablation_Backtest/
├── data/                                        # Local 2026 Binance CSVs (gitignored)
├── data_l1/                                     # Local 2024 Native L1 bookTicker & trades CSVs (gitignored)
├── results/                                     # Pre-computed v0.3.4 benchmark reports
│   ├── B70_hl_vs_D50_hl_drop/                   # Primary BTC+ETH benchmark (70ms)
│   ├── B200_hl_vs_D50_hl_drop/                  # 200ms open-batch stress benchmark
│   ├── B1000_hl_vs_D50_hl_drop/                 # 1000ms open-batch stress benchmark
│   ├── B70_hl_vs_D50_cancelfirst_hold_sweep/    # MM latency sweep (10/25/40ms) & cancel-first stress
│   ├── SOL_B70_hl_vs_D50_hl_drop/               # Altcoin expansion (SOLUSDT, 7d, with sniper probe)
│   ├── SOL_B70_hl_vs_D50_hl_drop_nosniper/      # Altcoin organic flow control (SOLUSDT, --sniper-clip 0)
│   ├── L1_202403_book/                          # Native L1 bookTicker benchmark (BTC+ETH+SOL, 3d)
│   └── L1_202403_proxy/                         # Same-day BidAskProxy control (BTC+ETH+SOL, 3d)
├── src/
│   └── main.rs                                  # Fixed-point simulation engine, CLI, reporting & 16 tests
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## 9. Scope, Assumptions & Limitations

- **Representative Single-MM & Top-of-Book Abstraction:** Models a single representative market maker clearing the entire replayed Binance taker stream at an endogenous competitive spread `s*net`, rather than multi-MM competition or full multi-level L2 orderbook depth replenishment.
- **Model-Derived vs. Exchange Tick Spreads:** Equilibrium half-spreads (`s*net = 0.467–1.517 bps`) are zero-profit model outputs under stylized fee/OpEx assumptions and are not directly comparable to real Binance Futures tick spreads (`~0.01–0.10 bps` on BTC/ETH).
- **Exogenous Order Flow:** Historical taker executions (`aggTrades`) are replayed causally and do not endogenously shrink or reroute in response to wider quoted spreads (such as Dywe's `+24%` wider `s*net` vs. `Regime B`).
- **Parameterized Latency-Arb Probe:** Stale-quote sniping events are generated by a synthetic probe (`$25,000` clip, `2.0 bps` trigger, `2ms` reaction) uniform across pairs, making the probe a larger volume share on thinner pairs (`15.0%` on `SOLUSDT` vs. `1.1%` on `BTC/ETH`; see `SOL_B70_hl_vs_D50_hl_drop_nosniper` for pure organic taker flow).
- **Bootstrap CI Scope & Holiday L1 Sample:** Joint block bootstrap CIs (`56` blocks for 7-day runs; `24` blocks for the 3-day March 28–30, 2024 L1 window spanning Good Friday) quantify drift sampling variance under *fixed* parameters and do not cover uncertainty in `capture`, `impact`, `leak`, reaction times, or batch windows.
- **Stylized Competitor Models & Cryptographic Timing:** Regimes B (`70ms`, `200ms`, `1000ms`) are stylized batching models, not protocol-level reproductions of named exchanges. Dywe's `50ms` batch window, `35ms`/`70ms` decryption RTTs, and MEV extraction grids are parameterized counterfactual inputs rather than live Devnet benchmarks or direct measurements of sandwich attacks on Binance Futures.

## 10. Research Conclusion

**What this replay experiment measures (`MEV = 0`):**

- **Adverse-selection reduction vs. FIFO with zero dropped taker volume:** Post-decrypt in-memory `New (ALO) -> Cancel -> Taker` ordering (`--d-batch-order hl`) enables Dywe (`Regime D`) to achieve `0.0%` dropped USD taker volume under `pending=drop` while cutting 1s MM markout by `38.9%` (`BTC/ETH` proxy), `40.4%` (native L1 `bookTicker`), and `23.8%` (organic `SOLUSDT` flow without sniper probes) vs. continuous FIFO (`Regime A`).
- **Explicit MM-vs-taker cost structure vs. open priority batches:** Against a `70ms` open batch (`Regime B, hl`), Dywe's `35ms` threshold decryption delay widens the MM equilibrium half-spread by `~24%` (`+0.115 bps` on native L1; `+0.164 bps` on `BTC/ETH`), while leaving static taker All-In Net cost statistically unchanged at `MEV = 0` (`+0.005 bps` on native L1; `-0.002 bps` on `BTC/ETH`).
- **Proxy fidelity confirmed on same-day L1 data:** Running `BidAskProxy` on the identical March 28–30, 2024 dataset reproduces native `bookTicker` FIFO 1s markout within `0.013 bps` (`-1.325` vs. `-1.312 bps`).

**What depends on scenario assumptions & remains to be validated in production:**

- Any net taker All-In cost advantage (`D < B`) depends on external counterfactual assumptions regarding ambient MEV capture (`≥10%`), per-order MEV impact (`≥0.5 bps`), and residual metadata leakage (`leak_ppm`), requiring calibration against live on-chain CLOB MEV measurements.
- Empirical p50/p95/p99 threshold decryption latency across geographically distributed validator nodes on the Dywe multi-region Devnet, alongside endogenous MM spread competition and multi-level L2 book depth under live order flow.
