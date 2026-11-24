# Dywe Microstructural Ablation Backtest (`dywe-ablation-sim` · v0.3.5)

**Deterministic, Fixed-Point (`i64`/`i128`) Causal Orderbook & Execution Replay Engine** benchmarking the **Dywe Exchange** threshold-encrypted micro-batch architecture against continuous FIFO and stylized open batch-auction designs across **30 pair-days** (**`$240.50B` pure historical exchange taker volume**, excluding synthetic sniper probes) of Binance USDⓈ-M Futures market data:

- **Primary Multi-Pair Benchmark (`BTCUSDT` & `ETHUSDT`, Nov 10–16, 2026 — 14 pair-days, proxy):** `$164.93B` pure historical taker volume (`$164,933,965,851`) across `33,665,163` raw `aggTrades` (`11,472,331` 1-ms directional bursts) and `6,384,781` distinct ms-level `BidAskProxy` updates (`84,131,187` raw observations).
- **Native L1 `bookTicker` Benchmark (`BTCUSDT`, `ETHUSDT` & `SOLUSDT`, Mar 28–30, 2024 — 9 pair-days, native L1):** `$62.93B` pure historical taker volume (`$62,931,377,194`) across `8,615,487` raw `aggTrades` (`4,244,321` 1-ms directional bursts) and `74,023,564` raw L1 quote rows deduplicated to `3,269,988` distinct ms-level mid changes (median inter-update interval `3 ms`, mean `237.79 ms`), complemented by a same-day `BidAskProxy` control comparison.
- **Altcoin Organic Flow Control (`SOLUSDT`, Nov 10–16, 2026 — 7 pair-days, proxy, `--sniper-clip 0`):** `$12.63B` pure historical taker volume (`$12,631,910,075`) across `2,035,536` raw `aggTrades` (`1,997,052` 1-ms directional bursts) and `636,535` distinct ms-level `BidAskProxy` updates (`15,365,789` raw observations).

> **Sample Heterogeneity & Volume Accounting Note:** The 30 pair-days span two distinct market regimes — 9 pair-days of native L1 `bookTicker` data from March 28–30, 2024 (spanning Good Friday) and 21 pair-days of trade-derived `BidAskProxy` data from November 10–16, 2026. Benchmarks are reported separately by dataset rather than pooled into a single average. When the synthetic latency-arb probe is active (`--sniper-clip 25000`), total executed MM volume includes an additional `~$1.2B–$1.5B` of probe volume on L1 and `~$2.8B–$3.5B` on `BTC/ETH`; both `Hist Taker Vol` and `MM Total Vol (incl. Probe)` are reported explicitly in all tables.

---

## 1. Executive Thesis: Empirical Mechanics vs. Scenario Counterfactuals

Decentralized perpetual orderbooks face an architectural trade-off between **adverse-selection protection**, **execution responsiveness**, and **pre-trade order privacy**. To maintain strict scientific separation, this repository divides all findings into **(A) Measured Mechanics at `MEV = 0`** and **(B) Parameterized Counterfactual MEV Scenarios**:

### A. Measured Mechanics at `MEV = 0` (Exogenous Historical Taker Flow)

1. **Continuous Transparent FIFO (`Regime A`):** Exposes resting market-maker (MM) quotes to stale-quote adverse selection (`-1.717 bps` 1s markout on `BTC/ETH` proxy; `-1.312 bps` on native L1 `bookTicker`; `-2.011 bps` on organic `SOLUSDT`), requiring a competitive equilibrium half-spread of `s*net = 1.517 bps` (`1.112 bps` on native L1) under baseline maker rebate `0.50 bps` (paid to MM) and `0.30 bps` hedging OpEx.

2. **Open Priority Micro-Batch (`Regime B` — `70ms`, `--b-batch-order hl`):** Processes `New (ALO/post-only) -> Cancel -> Taker` inside discrete open batch windows, cutting 1s MM markout by **`48.5%`** on `BTC/ETH` (`-0.885 bps`, `s*net = 0.685 bps`) and **`49.2%`** on native L1 (`-0.667 bps`, `s*net = 0.467 bps`) with `<0.05%` dropped USD taker volume (`0.026%` L1, `0.041%` BTC/ETH). However, all orders sit in a transparent mempool prior to batch execution.

3. **Uniform-Price Frequent Batch Auction (`Regime C`, `100ms`):** Clears opposing flow internally at a single uniform price (`-0.861 bps` 1s markout on `BTC/ETH`; `-0.622 bps` on native L1), but incurs higher waiting drift (`0.895 bps` proxy; `0.722 bps` L1) and the highest `MEV = 0` All-In Net taker cost (`0.950 bps` proxy; `0.671 bps` L1).

4. **Dywe Threshold-Encrypted Micro-Batch (`Regime D`, `50ms` or `70ms` batch + `35ms` threshold decryption, `--d-batch-order hl`):**

   - **Adverse-Selection Reduction vs. FIFO (Ranges Across Reaction Speed & Requote Threshold):** At baseline (`mm_react = 10ms`, `requote_thr = 1.50 bp`), Dywe (`50ms` window, `35ms` decrypt) cuts 1s MM toxic markout by **`40.4%`** on native L1 `bookTicker` (`-0.782` vs. `-1.312 bps`), **`38.9%`** on `BTC/ETH` proxy (`-1.049` vs. `-1.717 bps`), and **`23.8%`** on pure organic `SOLUSDT` flow (`--sniper-clip 0`, `-1.533` vs. `-2.011 bps`).

     Sweeping `mm_react` across `1–40ms` on native L1 (at `requote_thr = 1.50 bp`) yields **`8.2%–51.1%`** markout reduction — **strongest for fast institutional HFT desks** (`51.1%` at `1ms`, `50.5%` at `2ms`, `46.3%` at `5ms`).

     Notably, at `1ms`, FIFO incurs `0` stale sniper hits (`0/164`) yet still suffers `-1.271 bps` markout from organic informed flow; the `51.1%` reduction comes from `hl` batching applying fresh MM quotes ahead of incoming organic takers within the same batch rather than sniper suppression alone.

     Tightening the MM requote threshold from `1.50 bp` to `0.50 bp` and `0.15 bp` (at `react = 10ms`) increases markout reduction from **`40.4%`** to **`48.7%`** and **`51.0%`**, respectively.

   - **Window Length vs. Decryption Latency (Isolating the True Cost of Cryptography):** Comparing `D (50ms, 35ms dec)` (`s*net = 0.582 bps` on L1) against `B (70ms)` (`0.467 bps`) mixes two separate effects: window length (`50ms` vs. `70ms`) and threshold decryption (`35ms`). Isolating decryption at a matched `70ms` window (`--window-ms 70 --b-window-ms 70`) proves that **threshold decryption itself adds virtually zero MM spread cost**:

     - **BTC/ETH (14d, matched `70ms`):** `s*net = 0.685 bps` (`B`) vs. **`0.685 bps`** (`D, 35ms dec`, **`+0.000 bps`**) and `0.686 bps` (`D, 70ms dec`, `+0.001 bps`).
     - **Native L1 (9d, matched `70ms`):** `s*net = 0.467 bps` (`B`) vs. **`0.474 bps`** (`D, 35ms dec`, **`+0.007 bps` / `+1.5%`**) and `0.476 bps` (`D, 70ms dec`, `+0.009 bps`).
     - In the `D50` vs. `B70` comparison on native L1 (`+0.115 bps` total `s*net` gap), **`96.5%` (`+0.111 bps`) is caused by the shorter `50ms` responsiveness window, and only `3.5%` (`+0.004 bps`) by `35ms` decryption**.

   - **Taker All-In Cost Parity at `MEV = 0` & Low Drop Rates under `hl` Ordering:** Under exogenous taker flow and competitive zero-profit MM pricing (`MEV = 0`), **All-In Net taker cost is virtually identical across Regimes A–D** (`0.922–0.950 bps` on `BTC/ETH`; `0.651–0.671 bps` on native L1; `D − B` is `-0.002 bps [~]` on `BTC/ETH` and `+0.005 bps` on native L1).

     Lower equilibrium spreads under batching are offset by waiting drift and reduced taker staleness capture. Under `pending=drop`, post-decrypt `hl` ordering keeps dropped taker volume below **`0.041%` USD** at `requote_thr = 1.50 bp` (`0.036%` D50 / `0.026%` D70 on L1; `0.041%` on BTC/ETH; `0.039%` on SOL) and **`0.117%` USD** at `0.15 bp` — removing the dead-gap losses of `cancel-first` while matching open `hl` batches.

### B. Parameterized MEV Counterfactuals (`capture > 0%`, `--d-exposure-mode all`)

Because Binance USDⓈ-M Futures has no public mempool or on-chain sandwich MEV, **any net taker cost advantage of Dywe over an open `70ms` batch (`D < B`) arises strictly from the parameterized counterfactual MEV extraction grid**:

- **Uniform Exposure Mode (`--d-exposure-mode all`, default in `v0.3.5`):** Consistent with full pre-commit payload encryption and uniform 256-byte frame padding, `eff_leak = min(leak_ppm, capture_ppm)` applies uniformly to *all* large bursts (`≥$50k`), removing dependence on MM cancel frequency (the legacy `v0.3.4` `cancel-batch` mode remains available via CLI for comparison).
- **Large-Order Share Bounds (`≥$50k`):** Unmerged raw `aggTrades` (lower bound) vs. 1-ms directional bursts via `merge_taker` (upper bound) represent **`57.8%–71.2%`** of `BTC/ETH` volume, **`47.9%–68.2%`** of native L1 volume, and **`39.8%–42.4%`** of `SOLUSDT` volume.
- **Break-Even Residual Leakage (`eff_leak*`) & Scenario Savings (Burst Upper Bound vs. Raw Lower Bound):** At `100%` uniform exposure (`all`) and `50%` MM MEV loss share, Dywe is cheaper for takers (`D < B`) whenever encryption reduces effective leakage below `eff_leak*`:

  - **BTC/ETH (`D70` vs `B70`):** Because zero-MEV parity holds (`Δ₀ = 0.000 bps`), `eff_leak* ≈ capture` (`10.0% / 25.0% / 50.0%` at `10% / 25% / 50%` capture). At `25%–50%` capture / `1.0 bp` impact / `10%` leak, scenario savings reach **`9.6%–22.0%`** (`-0.106` to `-0.282 bps`) at the `71.2%` 1-ms burst upper bound, or **`8.1%–19.1%`** (`-0.087` to `-0.231 bps`) at the `57.8%` unmerged `aggTrades` lower bound.

  - **Native L1 (`D50` vs `B70`):** Because `Δ₀ = +0.005 bps` at `MEV = 0`, `eff_leak*` is strictly below `capture` — `8.4%` (`10%` cap / `0.5 bp`), `9.2%` (`10%` cap / `1.0 bp`), `24.2%` (`25%` cap / `1.0 bp`), and `49.2%` (`50%` cap / `1.0 bp`) — meaning Dywe wins only when encryption trims at least **`~1.6 pp`** of capture at `0.5 bp` impact or **`~0.8 pp`** at `1.0 bp`.

    At `25%–50%` capture / `1.0 bp` / `10%` leak, scenario savings reach **`11.7%–26.7%`** (`-0.096` to `-0.265 bps`) at the `68.2%` burst upper bound, or **`8.7%–21.0%`** (`-0.067` to `-0.187 bps`) at the `47.9%` unmerged lower bound.

  - **Organic `SOLUSDT` (`D50` vs `B70`, `--sniper-clip 0`):** `eff_leak*` is `7.4%–49.3%`, with **`4.0%–10.6%`** (`-0.058` to `-0.164 bps`) savings at `25%–50%` capture / `1.0 bp` / `10%` leak (`42.4%` burst bound).

---

## 2. Architectural Comparison & Unified Threat Model

Every historical mid-price update and directional taker burst is replayed across four isolated matching engines under identical latency, reaction, and threshold parameters:

| Regime | Reference Architecture | Ordering & Clearing Mechanics | Pre-Trade Privacy | Primary Trade-Off (`MEV = 0`, L1 `10ms` react) |
| :--- | :--- | :--- | :--- | :--- |
| **Regime A** | **Continuous FIFO** *(Baseline CLOB)* | Strict arrival-time (`arr_ns`) execution; no batching; no cancel priority. | None (Transparent) | High 1s MM toxic markout (`-1.312 bps` L1 / `-1.717 bps` BTC/ETH) forces wide equilibrium half-spreads (`1.112` / `1.517 bps`). |
| **Regime B** | **Open Priority Batch** *(`70ms` / `200ms` / `1000ms`, `hl` order)* | `New (ALO) -> Cancel -> Taker/Sniper` inside discrete open batch windows. | None (Transparent) | Strong MM protection (`0.467` L1 / `0.685` BTC/ETH half-spread), but large bursts (`48%–71%` vol) remain exposed in the mempool. |
| **Regime C** | **Uniform-Price FBA** *(`100ms` Auction)* | Orders accumulate over `100ms` and clear at a single uniform price; opposing flow nets internally. | None (Transparent) | `100ms` hold increases waiting drift (`0.722` L1 / `0.895` BTC/ETH) and All-In Net taker cost (`0.671` / `0.950 bps`). |
| **Regime D** | **Dywe Protocol** *(IBTE `50ms`/`70ms` + `35ms` dec, `hl`)* | All order types (`New ALO`, `Cancel`, `Taker`) encrypted pre-commit, decrypt in memory at `batch_close + decrypt_ms`, then sort (`hl` default; `cancel-first` stress). | **Full Pre-Commit IBTE (`BLS12-381`)** *(parameterized `eff_leak` grid)* | At matched `70ms` window, `35ms` decrypt adds `+0.000 bps` (BTC/ETH) and `+0.007 bps` (L1) to `s*net`; at `50ms`, shorter cadence trades `+0.115 bp` `s*net` vs `B70`. |

### Unified Post-Decrypt Threat Model (Why No Plaintext Cancel Headers Are Needed)

- **Full Pre-Commit Encryption:** In Dywe's architecture, *all* order payloads (`New ALO`, `Cancel`, and `Taker`) are threshold-encrypted client-side under `BLS12-381` with uniform frame padding until `batch_close`.
- **Post-Decrypt In-Memory Priority (`--d-batch-order hl`):** Because the 5-of-7 validator quorum completes threshold decryption inside matching-engine memory at `clear_ns = batch_close + decrypt_ms` *before* matching begins, the engine applies deterministic `New (ALO) -> Cancel -> Taker` ordering in memory without leaking `ALO`/`IOC` flags or cancel headers prior to sequence lock.
- **No Cryptography Inside the Replay Simulator:** This engine executes **zero `BLS12-381` pairing or decryption operations**. Regime D models the *microstructural timing and ordering* of an encrypted micro-batch (`window_ms` + `decrypt_ms` + post-decrypt `hl` ordering), while `leak_ppm` (`0%`, `10%`, `25%`, `50%`, clamped by `eff_leak = min(leak_ppm, capture_ppm)`) parameterizes potential residual side-channel leakage under `--d-exposure-mode all` (default) or `cancel-batch` (legacy).
- **Calibration Disclaimer:** Regimes `B (70ms)`, `B (200ms)`, and `B (1000ms)` are stylized batching models calibrated to representative intervals from public documentation (Hyperliquid and dYdX docs, accessed Nov 2026). They are not protocol-level or byte-for-byte reproductions of any exchange matching engine.

---

## 3. Simulation & Mathematical Methodology

### 3.1 Separation of Empirical Data, Mechanical Parameters & Scenario Assumptions

1. **Historical Component (Measured):** Raw Binance USDⓈ-M Futures `bookTicker`, `trades`, and directional `aggTrades` across 30 pair-days (`$240.50B` pure historical taker volume).
2. **Mechanical Component (Parameterized & CLI-Exposed):** Feed latency (`5ms`), MM quote recomputation (`--mm-react`, swept `1,2,5,10,25,40ms`), MM requote threshold (`--requote-thr-sub`, default `150` = `1.50 bp`, swept `15,50,150`), synthetic latency-arb probe (`2ms` reaction, `--sniper-thr-sub` default `200` = `2.00 bp`, `--sniper-clip` default `$25,000`, `0` disables), client-to-sequencer transit (`30ms`), natural sub-ms jitter (`0–999,999 ns`), batch windows (`--window-ms`, `--b-window-ms`, `--fba-window-ms`), and post-commit decryption RTTs (`--decrypt 0,35,70`).
3. **Scenario Component (Parameterized Grid):** Ambient MEV capture rate (`0%–100%`), per-order price impact (`0.0–3.0 bps`), MM loss allocation (`50%` baseline), Regime D exposure mode (`all` vs. `cancel-batch`), and residual metadata leakage (`0%–50%`, automatically clamped to `eff_leak = min(leak_ppm, capture_ppm)` so `capture = 0%` enforces strictly zero MEV).
4. **Derived Component (Calculated):** Competitive equilibrium half-spread (`s*net`), single-denominator All-In Net Taker Cost, and break-even residual leakage (`eff_leak*`).

### 3.2 Fixed-Point Integer Arithmetic (`i64` / `i128`) & Determinism

- All replay accounting uses fixed-point integer scaling (`PRICE_SCALE = 100_000_000`, `QTY_SCALE = 100_000_000`) with widening `i128` accumulators. Relative moves and markouts use integer **sub-bps** (`100 sub = 1.00 bps`, truncating toward zero) and `Q = 1,000` in Phase 2 (`i128`).

  *(Note: `f64` is used only for computing standard error inside the synthetic preflight unit test `shuffled_zero_spread` and mean interval display in `format_book_diag`.)*

- Parallel daily workers compute isolated per-day, per-block accumulators merged in deterministic `(symbol, day, block)` order with fixed SplitMix64 seeds.

### 3.3 Native L1 `bookTicker` Density & Same-Day Proxy Validation (`--mid-source book|proxy`)

- **Native L1 `bookTicker` Availability & Density:** No daily USDⓈ-M `bookTicker` archives exist on `data.binance.vision` after `2024-03-30`. Across the 9 pair-days of March 28–30, 2024 (`BTCUSDT`, `ETHUSDT`, `SOLUSDT`), raw `bookTicker` files contain **`74,023,564` raw quote rows** (`~8.22M` updates/pair-day). Because `mid_at()` is a step function of `(best_bid + best_ask) / 2`, `push_mid_tracked()` collapses `4,312,026` same-ms updates and skips `66,441,550` BBO quantity-only updates, retaining **`3,269,988` distinct ms-level mid changes** with a **median inter-update interval of `3 ms`** (`2 ms` on `SOLUSDT`, `5–7 ms` on `BTCUSDT`/`ETHUSDT`).

- **Same-Day Direct Comparison (`book` vs. `proxy` on Mar 28–30, 2024):** Running `BidAskProxy` on the identical March 28–30, 2024 dates (`22,327,383` observations, `3.3%` crossed, mean valid spread `0.758 bps`) shows that `BidAskProxy` matches native `bookTicker` within `0.013 bps` on FIFO, while **conservatively understating** batch markout reduction (`33.9%` vs. `40.4%` on `D`; `39.9%` vs. `49.2%` on `B`) because the proxy is constructed from the same trade stream replayed as takers (bid-ask bounce correlation):

| Metric (Mar 28–30, 2024 · `BTC+ETH+SOL`, 9 pair-days) | Native L1 `bookTicker` (`book`) | Same-Day `BidAskProxy` (`proxy`) | Delta (`Proxy − Book`) |
| :--- | :---: | :---: | :---: |
| **Regime A (FIFO) 1s MM Markout** | `-1.312 bps` `[-1.425, -1.216]` | `-1.325 bps` `[-1.439, -1.224]` | `-0.013 bps` |
| **Regime B (`70ms`, `hl`) 1s MM Markout** *(Reduction vs A)* | `-0.667 bps` *(`49.2%`)* | `-0.796 bps` *(`39.9%`)* | `-0.129 bps` |
| **Regime D (`50ms`, `35ms` dec, `hl`) 1s MM Markout** *(Reduction vs A)* | `-0.782 bps` *(`40.4%`)* | `-0.876 bps` *(`33.9%`)* | `-0.094 bps` |
| **Zero-MEV Taker All-In Delta (`D − B`)** | `+0.005 bps` (`B < D`) | `+0.002 bps` (`B < D`) | `-0.003 bps` |

- **2026 Proxy Diagnostics (`--mid-mode bidask`):**
  - `BTC/ETH` (14 pair-days, Nov 2026): `84,131,187` observations, `0.8%` crossed (`695,039`), mean valid `(ask - bid)` spread `0.708 bps`.
  - `SOLUSDT` (7 pair-days, Nov 2026): `15,365,789` observations, `0.0%` crossed (`1,305`), mean valid spread `1.903 bps`.

### 3.4 Equilibrium Spread (`s*net`), Single-Denominator All-In Identity & Placebo Control

For every regime and scenario, the engine solves for the zero-profit competitive equilibrium half-spread `s*net` (configurable via `--rebate-sub`, default `50` = `0.50 bps` rebate paid to MM; `--opex-sub`, default `30` = `0.30 bps` hedging OpEx):

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

And the **break-even residual leakage** `eff_leak*` (where `All-In(D) == All-In(B)`) for a given ambient capture rate, per-order impact, and Regime D exposure fraction `exp` is solved analytically in `breakeven_leak_ppm()`:

$$
\text{eff\_leak}^* =
\frac{\text{MEV}_B(\text{capture}, \text{impact}) - \Delta_0}
{\text{UnitMEV}_D(\text{impact}, \text{exp})},
\qquad
\Delta_0 = \text{All-In}_0(D) - \text{All-In}_0(B)
$$

- **Joint Block Bootstrap:** Synchronized 3-hour blocks resampled jointly across pairs (`56` blocks for 7-day runs, `24` blocks for 3-day L1 run; `1,000` draws, `95% CI`).
- **Circular-Shift Placebo Control (`600s` / `3600s`):** Shifting taker timestamps while keeping the price path fixed destroys `96.8%` of Regime A's 1s markout on `BTC/ETH` (`-1.717` → `-0.055 bps`), `97.3%` on native L1 `bookTicker` (`-1.312` → `-0.036 bps`), and `99.7%` on pure organic `SOLUSDT` flow (`-2.011` → `-0.006 bps` under `--sniper-clip 0`), confirming that simulated markout stems from temporal alignment between order flow and price moves rather than static drift.
- **Simulated Latency-Arb Probe (`StaleSnipe`):** Stale-quote sniping hits are generated by a parameterized deterministic probe (`2.0 bps` trigger, `2ms` reaction, `$25,000` clip), not observed exchange order IDs. Setting `--sniper-clip 0` (as in `v035_SOL_nosniper_all`) or testing `mm_react = 1ms` (`0` stale hits on L1) isolates pure historical taker flow.

---

## 4. Pre-Computed Audit Reports (`results/`)

All 8 `v0.3.5` benchmark scenarios are committed under `results/` (each directory contains `report.md`, interactive `ablation_report.html`, and full `breakeven.csv` sensitivity grids, checksummed in `results/SHA256SUMS.txt`):

| Directory | Scenario & Data Source | Key Takeaway |
| :--- | :--- | :--- |
| [`results/v035_L1_book_all/`](./results/v035_L1_book_all/report.md) | **Native L1 `bookTicker` (`BTC+ETH+SOL`, 9d)**<br>B `70ms` vs D `50ms`, `exp=all`, `drop`, `req=1.50bp` | `$62.93B` pure taker vol (`47.9%–68.2%` `≥$50k`), `-40.4%` 1s markout vs FIFO, `0.036%` dropped USD, `-11.7%` to `-26.7%` scenario savings (`68.2%` burst bound; `8.7%–21.0%` raw bound), `eff_leak* = 8.4%–49.6%`. |
| [`results/v035_L1_win70_decrypt/`](./results/v035_L1_win70_decrypt/report.md) | **Native L1 Decrypt Isolation (matched `70ms`)**<br>B `70ms` vs D `70ms` (`0/35/70ms` dec), `exp=all`, `drop` | Isolates pure `35ms` decryption cost: **`+0.007 bp`** `s*net` (`0.467 -> 0.474 bp`, `+1.5%`); `0.026%` dropped USD (identical to B). |
| [`results/v035_L1_req15/`](./results/v035_L1_req15/report.md) | **Native L1 Tight Requote (`0.15 bp`)**<br>`--requote-thr-sub 15`, `exp=all`, `drop` | **`-51.0%`** 1s markout vs FIFO (`-0.591` vs `-1.207 bp`), `0.117%` dropped USD, `54.8%` cancel-batch share, `eff_leak* = 9.0%–49.8%`. |
| [`results/v035_L1_req50/`](./results/v035_L1_req50/report.md) | **Native L1 Medium Requote (`0.50 bp`)**<br>`--requote-thr-sub 50`, `exp=all`, `drop` | **`-48.7%`** 1s markout vs FIFO (`-0.629` vs `-1.227 bp`), `0.050%` dropped USD, `45.0%` cancel-batch share, `eff_leak* = 8.6%–49.6%`. |
| [`results/v035_L1_react_sweep_hold/`](./results/v035_L1_react_sweep_hold/report.md) | **Native L1 MM Reaction Sweep (`1–40ms`)**<br>`--mm-react 1,2,5,10,25,40`, `exp=all`, `hold` | `D` markout reduction ranges **`8.2%–51.1%`** (strongest at `1ms` with `0` stale sniper hits). *Note: Phase 2 frontier/placebo/CI blocks evaluate `mm_react[0]` (`1ms`).* |
| [`results/v035_BTCETH_all/`](./results/v035_BTCETH_all/report.md) | **Primary `BTC+ETH` (14d, Nov 2026 proxy)**<br>B `70ms` vs D `50ms`, `exp=all`, `drop` | `$164.93B` pure taker vol (`57.8%–71.2%` `≥$50k`), `-38.9%` 1s markout vs FIFO, `-0.002 bp [~]` at `MEV=0`, `-9.8%` to `-22.2%` scenario savings (`71.2%` burst bound). |
| [`results/v035_BTCETH_win70_decrypt/`](./results/v035_BTCETH_win70_decrypt/report.md) | **`BTC+ETH` Decrypt Isolation (matched `70ms`)**<br>B `70ms` vs D `70ms` (`0/35/70ms` dec), `exp=all`, `drop` | Pure `35ms` decryption cost is **`+0.000 bp`** (`0.685` vs `0.685 bp`; `All-In = 0.924` vs `0.924 bp`); `-9.6%` to `-22.0%` scenario savings (`71.2%` burst bound; `8.1%–19.1%` raw bound). |
| [`results/v035_SOL_nosniper_all/`](./results/v035_SOL_nosniper_all/report.md) | **Organic `SOLUSDT` (7d, `--sniper-clip 0`)**<br>B `70ms` vs D `50ms`, `exp=all`, `drop` | `$12.63B` pure organic flow (`39.8%–42.4%` `≥$50k`), **`-23.8%`** 1s markout vs FIFO (`-1.533` vs `-2.011 bp`), `0.039%` dropped USD, `99.7%` removed by 600s placebo. |

---

## 5. Consolidated Benchmark Summary (`v0.3.5`, `exp=all`, `wait=all`)

### 5.1 Native L1 `bookTicker` Benchmark (`BTC+ETH+SOL`, 9 Pair-Days, `MEV = 0`, `react = 10ms`, `req = 1.50bp`)

| Architecture Regime | Hist Taker Vol | MM Vol (incl. Probe) | Dropped Takers (USD%) | 1s MM Markout | `s*net` | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net | Probe Hit/Miss |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **A: Continuous FIFO** | `$62.93B` | `$64.11B` | `216 (0.004%)` | `-1.312 bp` | `1.112 bp` | `1.112 bp` | `-1.068 bp` | `0.626 bp` | **`0.670 bp`** | `35,107 / 48` |
| **B: Batch `70ms` (`hl`)** | `$62.91B` | `$64.26B` | `322 (0.026%)` | `-0.667 bp` | `0.467 bp` | `0.467 bp` | `-0.517 bp` | `0.701 bp` | **`0.651 bp`** | `3,357 / 26` |
| **C: Uniform-FBA (`100ms`)** | `$62.92B` | `$59.46B` | `269 (0.013%)` | `-0.622 bp` | `0.422 bp` | `0.390 bp` | `-0.441 bp` | `0.722 bp` | **`0.671 bp`** | `2,180 / 22` |
| **D: Dywe `50ms` (`0ms` dec)** | `$62.91B` | `$64.21B` | `572 (0.036%)` | `-0.778 bp` | `0.578 bp` | `0.578 bp` | `-0.609 bp` | `0.683 bp` | **`0.653 bp`** | `4,605 / 45` |
| **D: Dywe `50ms` (`35ms` dec)** | **`$62.91B`** | **`$64.36B`** | **`572 (0.036%)`** | **`-0.782 bp`** | **`0.582 bp`** | **`0.582 bp`** | **`-0.674 bp`** | **`0.748 bp`** | **`0.656 bp`** | **`4,517 / 46`** |
| **D: Dywe `50ms` (`70ms` dec)** | `$62.91B` | `$64.45B` | `572 (0.036%)` | `-0.785 bp` | `0.585 bp` | `0.585 bp` | `-0.718 bp` | `0.792 bp` | **`0.659 bp`** | `4,444 / 48` |
| **D: Dywe `70ms` (`35ms` dec)** | **`$62.91B`** | **`$64.39B`** | **`322 (0.026%)`** | **`-0.674 bp`** | **`0.474 bp`** | **`0.474 bp`** | **`-0.577 bp`** | **`0.761 bp`** | **`0.658 bp`** | **`3,339 / 25`** |

### 5.2 Primary `BTC+ETH` Benchmark & Matched `70ms` Decrypt Isolation (14 Pair-Days, Nov 2026 Proxy, `MEV = 0`)

| Architecture Regime | Hist Taker Vol | MM Vol (incl. Probe) | Dropped Takers (USD%) | 1s MM Markout | `s*net` | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net | Probe Hit/Miss |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **A: Continuous FIFO** | `$164.90B` | `$167.72B` | `1,595 (0.022%)` | `-1.717 bp` | `1.517 bp` | `1.517 bp` | `-1.266 bp` | `0.677 bp` | **`0.928 bp`** | `71,633 / 97` |
| **B: Batch `70ms` (`hl`)** | `$164.87B` | `$167.90B` | `1,797 (0.041%)` | `-0.885 bp` | `0.685 bp` | `0.685 bp` | `-0.636 bp` | `0.874 bp` | **`0.924 bp`** | `6,308 / 60` |
| **C: Uniform-FBA (`100ms`)** | `$164.88B` | `$145.80B` | `1,650 (0.029%)` | `-0.861 bp` | `0.661 bp` | `0.575 bp` | `-0.520 bp` | `0.895 bp` | **`0.950 bp`** | `4,223 / 43` |
| **D: Dywe `70ms` (`0ms` dec)** | `$164.87B` | `$167.90B` | `1,797 (0.041%)` | `-0.885 bp` | `0.685 bp` | `0.685 bp` | `-0.636 bp` | `0.874 bp` | **`0.924 bp`** | `6,308 / 60` |
| **D: Dywe `70ms` (`35ms` dec)** | **`$164.87B`** | **`$168.18B`** | **`1,797 (0.041%)`** | **`-0.885 bp`** | **`0.685 bp`** | **`0.685 bp`** | **`-0.717 bp`** | **`0.956 bp`** | **`0.924 bp`** | **`6,080 / 56`** |
| **D: Dywe `70ms` (`70ms` dec)** | `$164.87B` | `$168.39B` | `1,797 (0.041%)` | `-0.886 bp` | `0.686 bp` | `0.686 bp` | `-0.769 bp` | `1.008 bp` | **`0.924 bp`** | `5,728 / 55` |
| **D: Dywe `50ms` (`35ms` dec)** | **`$164.85B`** | **`$168.20B`** | **`2,153 (<0.06%)`** | **`-1.049 bp`** | **`0.849 bp`** | **`0.849 bp`** | **`-0.878 bp`** | **`0.951 bp`** | **`0.922 bp`** | **`8,523 / 79`** |

### 5.3 Sensitivity Sweeps on Native L1 `bookTicker`

#### A) MM Reaction Latency Sweep (`--mm-react 1,2,5,10,25,40`, `pending=hold`, `requote_thr = 1.50 bp`)

| `mm_react` | `A (FIFO)` 1s Markout | `B (70ms, hl)` 1s Markout | `B` Reduction vs A | `D (50ms, 35ms dec)` 1s Markout | `D` Reduction vs A | `A` StaleSnipe Hits |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`1 ms`** | `-1.271 bp` | `-0.551 bp` | `56.6%` | **`-0.622 bp`** | **`51.1%`** | `0` |
| **`2 ms`** | `-1.291 bp` | `-0.569 bp` | `55.9%` | **`-0.639 bp`** | **`50.5%`** | `19,963` |
| **`5 ms`** | `-1.301 bp` | `-0.613 bp` | `52.9%` | **`-0.698 bp`** | **`46.3%`** | `29,520` |
| **`10 ms`** | `-1.312 bp` | `-0.667 bp` | `49.2%` | **`-0.782 bp`** | **`40.4%`** | `35,107` |
| **`25 ms`** | `-1.332 bp` | `-0.818 bp` | `38.6%` | **`-0.991 bp`** | **`25.6%`** | `41,499` |
| **`40 ms`** | `-1.337 bp` | `-0.983 bp` | `26.5%` | **`-1.228 bp`** | **`8.2%`** | `44,959` |

#### B) MM Requote Threshold Sweep (`--requote-thr-sub 15, 50, 150`, `pending=drop`, `react = 10ms`)

| `requote_thr` | `A (FIFO)` 1s Markout | `B (70ms)` 1s Markout | `D (50ms, 35ms dec)` 1s Markout | `D` Reduction vs A | `D` Dropped USD% | Cancel-Batch Large % (`D`) |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`1.50 bp`** (`150`) | `-1.312 bp` | `-0.667 bp` | **`-0.782 bp`** | **`40.4%`** | `0.036%` | `26.7%` |
| **`0.50 bp`** (`50`) | `-1.227 bp` | `-0.506 bp` | **`-0.629 bp`** | **`48.7%`** | `0.050%` | `45.0%` |
| **`0.15 bp`** (`15`) | `-1.207 bp` | `-0.463 bp` | **`-0.591 bp`** | **`51.0%`** | `0.117%` | `54.8%` |

### 5.4 Break-Even Frontier under Uniform Exposure (`--d-exposure-mode all`, `All-In(D) − All-In(B)`, bps)

*Left value: `BTC/ETH` (`D70, 35ms dec` vs. `B70`, 14d proxy). Right value in brackets: Native L1 `bookTicker` (`D50, 35ms dec` vs. `B70`, 9d). Scenario parameters: `50%` MM MEV share, `10%` leak (`eff_leak = min(leak, capture)`), `0.50 bps` rebate paid to MM, `0.30 bps` OpEx. **All rows with capture > 0% are parameterized counterfactual scenarios, not measurements.***

| Large-Burst MEV Capture (`≥$50k`) \ Impact | `0.000 bp` | `0.500 bp` | `1.000 bp` | `2.000 bp` | `3.000 bp` |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **0% Capture** *(Sterile Zero-MEV)* | `0.000 ~` `[+0.005 B < D]` | `0.000 ~` `[+0.005 B < D]` | `0.000 ~` `[+0.005 B < D]` | `0.000 ~` `[+0.005 B < D]` | `0.000 ~` `[+0.005 B < D]` |
| **10% Capture** *(`eff_leak = 10%`)* | `0.000 ~` `[+0.005 B < D]` | `0.000 ~` `[+0.005 B < D]` | `0.000 ~` `[+0.005 B < D]` | `0.000 ~` `[+0.005 B < D]` | `0.000 ~` `[+0.004 B < D]` |
| **25% Capture** | `0.000 ~` `[+0.005 B < D]` | **`-0.053 D < B`** `[-0.045]` | **`-0.106 D < B`** `[-0.096]` | **`-0.212 D < B`** `[-0.197]` | **`-0.318 D < B`** `[-0.298]` |
| **50% Capture** | `0.000 ~` `[+0.005 B < D]` | **`-0.141 D < B`** `[-0.129]` | **`-0.282 D < B`** `[-0.265]` | **`-0.564 D < B`** `[-0.535]` | **`-0.847 D < B`** `[-0.805]` |
| **75% Capture** | `0.000 ~` `[+0.005 B < D]` | **`-0.229 D < B`** `[-0.214]` | **`-0.459 D < B`** `[-0.433]` | **`-0.917 D < B`** `[-0.872]` | **`-1.376 D < B`** `[-1.311]` |
| **100% Capture** | `0.000 ~` `[+0.005 B < D]` | **`-0.317 D < B`** `[-0.298]` | **`-0.635 D < B`** `[-0.602]` | **`-1.270 D < B`** `[-1.210]` | **`-1.905 D < B`** `[-1.818]` |

### 5.5 Break-Even Residual Leakage `eff_leak*` (`All-In(D, 35ms) == All-In(B, 70ms)`)

*Dywe is cheaper for takers (`D < B`) whenever actual residual leakage `eff_leak < eff_leak*` (`100%` uniform exposure, `50%` MM MEV share). Full multi-exposure tables (`Cancel-Batch`, `50%`, `75%`, `100%`) are printed in each `report.md` and `ablation_report.html` (note: the `Cancel-Batch` column in `breakeven_leak_ppm()` is an analytical approximation scaling `mm_large_n`/`tk_large_n` by the observed cancel-batch share).*

| Ambient Capture / Impact | Native L1 `D50` (`req=1.50bp`) | Native L1 `D50` (`req=0.15bp`) | Native L1 `D70` (`req=1.50bp`) | BTC/ETH `D70` (`req=1.50bp`) | SOL Organic `D50` (`sniper=0`) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **`10%` / `0.50 bp`** | `8.4%` | `9.0%` | `8.0%` | `10.1%` | `7.4%` |
| **`10%` / `1.00 bp`** | `9.2%` | `9.5%` | `9.0%` | `10.0%` | `8.7%` |
| **`25%` / `1.00 bp`** | `24.2%` | `24.5%` | `24.0%` | `25.0%` | `23.7%` |
| **`50%` / `1.00 bp`** | `49.2%` | `49.5%` | `49.0%` | `50.0%` | `48.7%` |
| **`50%` / `2.00 bp`** | `49.6%` | `49.8%` | `49.5%` | `50.0%` | `49.3%` |

---

## 6. Built-In Verification & Unit Test Suite (17 Pre-Flight Invariants + 18 Unit Tests)

Executed via `cargo test --release` (all **18 unit tests** pass in `<0.1s`) and automatically on every pre-flight run (**17 synthetic invariant checks**):

1. `t_causality` — Zero look-ahead bias: mutating future ticks after `T` leaves all fills before `T` bit-for-bit identical across all 4 regimes.
2. `t_identity` — `Regime D (dec=0, cancel-first)` equals `Regime B (cancel-first)` bit-for-bit at the `i128` accumulator level.
3. `t_d_post_decrypt_hl_order` — `Regime D (dec=0, hl)` equals `Regime B (hl)` bit-for-bit and eliminates artificial same-batch cancel-gap drops.
4. `t_hl_batch_order` — Verifies `New (ALO) -> Cancel -> Taker` ordering removes dead gaps vs. `cancel-first`.
5. `t_accounting_reconciliation` — Enforces exact volume conservation and exact additive `All-In Net` component identity.
6. `t_wait_cohort_all_identity` — Validates single-denominator (`tk_n`) linear additivity under `--wait-cohort all` vs. `large`.
7. `t_fba_sniper_attribution` — Confirms late FBA snipers arriving after same-batch MM requotes are attributed to `late_fresh` (`0` false stale hits).
8. `t_placebo_breaks_link` — Confirms circular timestamp shifting destroys flow-price alignment while preserving order counts and notional.
9. `t_book_ticker_diag` — Validates raw `bookTicker` row counting, same-ms/unchanged-mid deduplication counters, and exact 1-ms histogram median inter-update interval.
10. `t_zero_capture_zero_mev` — Enforces zero MEV extraction when ambient `capture = 0%`, regardless of `leak_ppm`.
11. `t_d_exposure_modes_and_breakeven` — Verifies `ExposureMode::All` vs. `ExposureMode::CancelBatch` scaling and analytical `breakeven_leak_ppm()` math.
12. `t_dropped_precision` — Confirms `fmt_dropped_cell()` resolves sub-`0.1%` dropped USD shares to `0.001%` precision.
13. `t_pending_drop_policy`, `t_bidask_mid_proxy`, `t_mev_share_sum`, `t_sniper_protection`, `t_shuffled_zero_spread`, `t_html_escape` — Validates IOC drop vs. hold semantics, `BidAskProxy` normal/crossed diagnostics, MEV share conservation, stale-snipe protection, randomized-flow zero-bias controls, and HTML sanitization.

---

## 7. Data Sources & How to Reproduce Locally

### 7.1 Historical Market Data Source & Build Environment

- **Market Data:** Raw daily USDⓈ-M Futures archives from **Binance Public Data Vision** (`https://data.binance.vision/data/futures/um/daily/`):
  - **2026 Dataset (`./data/`):** `BTCUSDT`, `ETHUSDT`, `SOLUSDT` (`2026-11-10` through `2026-11-16`: `trades/` and `aggTrades/`).
  - **Native L1 & Same-Day Proxy Dataset (`./data_l1/`):** `BTCUSDT`, `ETHUSDT`, `SOLUSDT` (`2024-03-28` through `2024-03-30`: `bookTicker/`, `trades/`, and `aggTrades/`).
- **Verified Build Environment:** Apple Silicon (`macOS`), `rustc 1.100.0 (0ef1bc386 2026-11-09)`, zero external crate dependencies (`Cargo.lock` committed), artifact checksums recorded in `results/SHA256SUMS.txt`.

### 7.2 Step-by-Step Local Reproduction

**Prerequisites:** Rust toolchain (`rustc 1.82+` / `cargo`), `curl`, `shasum`, and `unzip`.

#### Step 1: Download & Verify Checksums

```bash
set -e
BASE="https://data.binance.vision/data/futures/um/daily"

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

#### Step 2: Run the 18-Test Suite & Regenerate All 8 `v0.3.5` Reports

```bash
cargo test --release
mkdir -p results

cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 70 --b-batch-order hl --d-batch-order hl --d-exposure-mode all --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/v035_BTCETH_all

cargo run --release -- --threads 6 --allow-reconstructed --window-ms 70 --b-window-ms 70 --decrypt 0,35,70 --d-exposure-mode all --pending-policy drop --mid-mode bidask --out-dir results/v035_BTCETH_win70_decrypt

cargo run --release -- --threads 6 --allow-reconstructed --symbols SOLUSDT --sniper-clip 0 --b-window-ms 70 --b-batch-order hl --d-batch-order hl --d-exposure-mode all --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/v035_SOL_nosniper_all

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --b-window-ms 70 --d-exposure-mode all --pending-policy drop --out-dir results/v035_L1_book_all

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --window-ms 70 --b-window-ms 70 --decrypt 0,35,70 --d-exposure-mode all --pending-policy drop --out-dir results/v035_L1_win70_decrypt

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --b-window-ms 70 --requote-thr-sub 15 --d-exposure-mode all --pending-policy drop --out-dir results/v035_L1_req15

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --b-window-ms 70 --requote-thr-sub 50 --d-exposure-mode all --pending-policy drop --out-dir results/v035_L1_req50

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --b-window-ms 70 --mm-react 1,2,5,10,25,40 --d-exposure-mode all --pending-policy hold --out-dir results/v035_L1_react_sweep_hold
```

### 7.3 Complete CLI Reference

| Argument | Description |
| --- | --- |
| `--data-dir <PATH>` | Directory containing extracted Binance CSVs (default: `./data`). |
| `--out-dir <PATH>` | Output directory for `report.md`, `ablation_report.html`, and `breakeven.csv` (default: `.`). |
| `--symbols <LIST>` | Comma-separated symbols to replay (default: `BTCUSDT,ETHUSDT`). |
| `--start <YYYY-MM-DD>` / `--days <N>` | Start date and consecutive days to replay (default: `2026-11-10`, `7`). |
| `--window-ms <MS>` | Dywe Regime D micro-batch window duration in ms (default: `50`). |
| `--b-window-ms <MS>` | Competitor Regime B batch window duration in ms (default: `0` = matches `--window-ms`). |
| `--fba-window-ms <MS>` | Regime C Uniform-Price FBA window duration in ms (default: `100`). |
| `--b-batch-order <hl\|cancel-first>` | Intra-batch ordering for Regime B (default: `hl`). |
| `--d-batch-order <hl\|cancel-first>` | Post-decrypt in-memory ordering for Regime D (default: `hl`). |
| `--d-exposure-mode <all\|cancel-batch>` | Regime D MEV exposure mode (`all` = uniform across all large bursts; `cancel-batch` = legacy; default: `all`). |
| `--decrypt <LIST>` | Post-commit threshold decryption RTTs in ms (default: `0,35,70`). |
| `--mm-react <LIST>` | MM quote recomputation latencies in ms (default: `10`). |
| `--requote-thr-sub <INT>` | MM mid-move requote threshold in sub-bps (`100 sub = 1.00 bp`; default: `150`). |
| `--sniper-thr-sub <INT>` | Synthetic sniper trigger threshold in sub-bps (default: `200`). |
| `--sniper-clip <USD>` | Notional clip size for the synthetic latency-arb probe; `0` disables probe (default: `25000`). |
| `--large-usd <INT>` | Notional threshold in USD for MEV-eligible large bursts (default: `50000`). |
| `--pending-policy <hold\|drop>` | Handling of takers arriving during an MM requote gap (default: `hold`). |
| `--wait-cohort <all\|large>` | Denominator for waiting drift in All-In Net (default: `all`). |
| `--mid-source <auto\|book\|proxy>` | Force native `bookTicker`, trade-derived `proxy`, or `auto` fallback (default: `auto`). |
| `--mid-mode <bidask\|trade>` | Trade-derived midpoint proxy formula when using `proxy` (default: `bidask`). |
| `--allow-reconstructed` | Required flag to allow `auto` mode to fall back from missing `bookTicker` to trade proxy. |
| `--rebate-sub <INT>` / `--opex-sub <INT>` | Maker rebate paid to MM & hedging OpEx in sub-bps (defaults: `50`, `30`). |
| `--placebo-shifts <LIST>` | Circular timestamp shifts in ms for placebo verification (default: `600000,3600000`). |
| `--block-hours <H>` | Block length in hours for joint block bootstrap (default: `3`). |
| `--bootstrap <N>` | Number of bootstrap draws for 95% CIs (default: `1000`). |
| `--threads <N>` | Parallel worker threads for daily replay (default: `4`). |
| `--skip-preflight` / `--preflight-only` | Skip the 17 synthetic pre-flight checks, or exit immediately after running them. |

---

## 8. Repository Structure

```text
Dywe_Rust_Microstructural_Ablation_Backtest/
├── data/                                    # Local 2026 Binance CSVs (gitignored)
├── data_l1/                                 # Local 2024 Native L1 bookTicker & trades CSVs (gitignored)
├── results/                                 # Pre-computed v0.3.5 benchmark reports
│   ├── SHA256SUMS.txt                       # Reference SHA-256 hashes of source & reports
│   ├── v035_BTCETH_all/                     # Primary BTC+ETH benchmark (B70 vs D50, exp=all)
│   ├── v035_BTCETH_win70_decrypt/           # BTC+ETH matched 70ms decrypt isolation
│   ├── v035_L1_book_all/                    # Native L1 bookTicker benchmark (B70 vs D50, exp=all)
│   ├── v035_L1_win70_decrypt/               # Native L1 matched 70ms decrypt isolation
│   ├── v035_L1_req15/                       # Native L1 tight requote sweep (0.15 bp)
│   ├── v035_L1_req50/                       # Native L1 medium requote sweep (0.50 bp)
│   ├── v035_L1_react_sweep_hold/            # Native L1 MM reaction sweep (1–40ms, pending=hold)
│   └── v035_SOL_nosniper_all/               # Organic SOLUSDT control (--sniper-clip 0, exp=all)
├── src/
│   └── main.rs                              # Fixed-point simulation engine, CLI, reporting & 18 tests
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

---

## 9. Scope, Assumptions & Limitations

- **Representative Single-MM & Top-of-Book Abstraction:** Models a single competitive zero-profit MM clearing the replayed Binance taker stream at an endogenous spread `s*net`, rather than multi-MM competition or multi-level L2 orderbook depth replenishment.
- **Model-Derived Spreads & Requote Threshold Sensitivity:** Equilibrium half-spreads (`s*net = 0.26–1.52 bps`) are zero-profit model outputs under stylized fee/OpEx assumptions and are not directly comparable to real Binance Futures tick spreads (`~0.01–0.10 bps`). Baseline runs use `--requote-thr-sub 150` (`1.50 bp`); tighter thresholds (`0.50 bp` and `0.15 bp`) are explicitly swept in `v035_L1_req50` and `v035_L1_req15`.
- **Exogenous Order Flow:** Historical taker executions (`aggTrades`) are replayed causally and do not endogenously shrink or reroute in response to quoted spread width.
- **1-ms Burst Aggregation (`merge_taker`):** Same-millisecond, same-direction executions are aggregated into 1-ms directional bursts. Because `aggTrades` already groups single-price fills, unmerged raw rows (`47.9%` L1 / `57.8%` BTC/ETH) provide a lower bound and 1-ms bursts (`68.2%` / `71.2%`) provide an upper bound on `≥$50k` volume share.
- **Organic Flow vs. Synthetic Sniper Effect:** At `mm_react = 1ms` on native L1, FIFO records `0` stale sniper hits (`0/164`) yet still experiences `-1.271 bps` 1s markout from organic informed flow; the `51.1%` markout reduction under `D` stems from `hl` batching applying fresh MM quotes ahead of incoming organic takers within the same batch.
- **Sweep Evaluation Scope:** In multi-value `--mm-react` sweeps (`v035_L1_react_sweep_hold`), Phase 2 frontier tables, placebo checks, and bootstrap sign checks evaluate `mm_react[0]` (`1ms`), while all reaction speeds appear in Phase 1 mechanics and cohort tables.
- **Bootstrap CI Scope & Holiday L1 Sample:** Joint 3-hour block bootstrap CIs (`56` blocks for 7-day runs; `24` blocks for the 3-day March 28–30, 2024 L1 window spanning Good Friday) quantify drift sampling variance under *fixed* parameters and do not account for volatility autocorrelation or parameter uncertainty (`capture`, `impact`, `leak`, reaction times, windows).
- **Offline HTML Rendering & Stylized Models:** Interactive `ablation_report.html` files load `Chart.js 4.4.1` via CDN (requiring internet access for charts; all numeric tables render offline). Regimes B/C are stylized models, not reproductions of Hyperliquid or dYdX engines; external docs were accessed Nov 2026.

---

## 10. Research Conclusion

**What this replay experiment measures (`MEV = 0`):**

- **Adverse-selection reduction vs. FIFO across realistic MM speeds and thresholds:** Post-decrypt in-memory `New (ALO) -> Cancel -> Taker` (`hl`) ordering enables Dywe (`Regime D`) to cut 1s MM toxic markout by **`8.2%–51.1%`** across `1–40ms` MM reaction latencies on native L1 `bookTicker` at `requote_thr = 1.50 bp` (`40.4%` at `10ms`, `51.1%` at `1ms` with `0` stale sniper hits; `48.7%–51.0%` at `0.50–0.15 bp` requote thresholds), **`38.9%`** on `BTC/ETH` proxy, and **`23.8%`** on pure organic `SOLUSDT` flow (`--sniper-clip 0`), while keeping dropped taker volume `<0.041%` USD at baseline (`<0.12%` at `0.15 bp`).

- **Threshold decryption adds near-zero MM spread cost:** At a matched `70ms` window, `35ms` decryption adds **`+0.000 bps`** to `s*net` on `BTC/ETH` (`0.685` vs. `0.685 bps`) and **`+0.007 bps`** (`+1.5%`) on native L1 (`0.474` vs. `0.467 bps`). Wider `s*net` in `D50` vs. `B70` (`+0.115 bps` on L1) is `96.5%` attributable to choosing a shorter `50ms` responsiveness window.

- **Static taker parity at `MEV = 0`:** Without mempool MEV, All-In Net taker cost is virtually identical across Regimes A–D (`0.651–0.671 bps` L1; `0.922–0.950 bps` BTC/ETH).

**What depends on scenario assumptions & remains to be validated in production:**

- Any net taker All-In cost advantage (`D < B`) depends on external counterfactual assumptions regarding ambient mempool MEV capture (`capture`), per-order impact (`impact`), and residual metadata leakage (`eff_leak < eff_leak*`), requiring calibration against live on-chain CLOB MEV telemetry.
- Empirical p50/p95/p99 `BLS12-381` threshold decryption latency across geographically distributed validator nodes on the Dywe multi-region Devnet, alongside endogenous MM spread competition and multi-level L2 book depth under live order flow.