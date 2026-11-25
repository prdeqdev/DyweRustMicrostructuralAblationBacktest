# Dywe Microstructural Ablation Backtest (`dywe-ablation-sim` · v0.3.5)

**Deterministic, Fixed-Point (`i64`/`i128`) Causal Orderbook & Execution Replay Engine** benchmarking the **Dywe Exchange** threshold-encrypted micro-batch architecture against continuous FIFO and stylized open batch-auction designs across **30 pair-days** (**`$240.50B` pure historical exchange taker volume**, excluding synthetic sniper probes) of Binance USDⓈ-M Futures market data.

> **This is a simulation, not a production measurement.** The replay engine executes no `BLS12-381` cryptography. Regime D models the timing and ordering of an encrypted micro-batch. Any taker-cost advantage over an open batch is a parameterized scenario output (see Section 1.B and "Claims Not Supported by This Study" in Section 11).

- **Primary Multi-Pair Benchmark (`BTCUSDT` & `ETHUSDT`, Nov 10–16, 2026 — 14 pair-days, proxy):** `$164.93B` pure historical taker volume (`$164,933,965,851`) across `33,665,163` raw `aggTrades` (`11,472,331` 1-ms directional bursts) and `6,384,781` distinct ms-level `BidAskProxy` updates (`84,131,187` raw observations).
- **Native L1 `bookTicker` Benchmark (`BTCUSDT`, `ETHUSDT` & `SOLUSDT`, Mar 28–30, 2024 — 9 pair-days, native L1):** `$62.93B` pure historical taker volume (`$62,931,377,194`) across `8,615,487` raw `aggTrades` (`4,244,321` 1-ms directional bursts) and `74,023,564` raw L1 quote rows deduplicated to `3,269,988` distinct ms-level mid changes (median inter-update interval `3 ms`, mean `237.79 ms`), complemented by a same-day `BidAskProxy` control.
- **Altcoin Organic Flow Control (`SOLUSDT`, Nov 10–16, 2026 — 7 pair-days, proxy, `--sniper-clip 0`):** `$12.63B` pure historical taker volume (`$12,631,910,075`) across `2,035,536` raw `aggTrades` (`1,997,052` 1-ms directional bursts) and `636,535` distinct ms-level `BidAskProxy` updates (`15,365,789` raw observations).

> **Sample Heterogeneity & Volume Accounting Note:** The 30 pair-days span two distinct market regimes: 9 pair-days of native L1 `bookTicker` data from March 28–30, 2024 (spanning Good Friday) and 21 pair-days of trade-derived `BidAskProxy` data from November 10–16, 2026. Native L1 data exists only for 2024; all 2026 results rely on the trade-derived proxy. Benchmarks are reported separately by dataset and never pooled into a single average. With the synthetic latency-arb probe active (`--sniper-clip 25000`), executed MM volume includes an additional `~$1.2B–$1.5B` of probe volume on L1 and `~$2.8B–$3.5B` on `BTC/ETH`. Both `Hist Taker Vol` and `MM Total Vol (incl. Probe)` are reported in all tables.

---

## 1. Executive Thesis: Measured Mechanics vs. Scenario Counterfactuals

Decentralized perpetual orderbooks face a trade-off between **adverse-selection protection**, **execution responsiveness**, and **pre-trade order privacy**. This repository separates all findings into **(A) Measured Mechanics at `MEV = 0`** and **(B) Parameterized Counterfactual MEV Scenarios**.

### A. Measured Mechanics at `MEV = 0` (Exogenous Historical Taker Flow)

1. **Continuous Transparent FIFO (`Regime A`):** Exposes resting market-maker (MM) quotes to stale-quote adverse selection (`-1.717 bps` 1s markout on `BTC/ETH` proxy; `-1.312 bps` on native L1; `-2.011 bps` on organic `SOLUSDT`), requiring a competitive equilibrium half-spread of `s*net = 1.517 bps` (`1.112 bps` on native L1) under a `0.50 bps` maker rebate (paid to the MM) and `0.30 bps` hedging OpEx.

2. **Open Priority Micro-Batch (`Regime B`, `70ms`, `--b-batch-order hl`):** Processes `New (ALO/post-only) -> Cancel -> Taker` inside discrete open batch windows, cutting 1s MM markout by **`48.5%`** on `BTC/ETH` (`-0.885 bps`, `s*net = 0.685 bps`) and **`49.2%`** on native L1 (`-0.667 bps`, `s*net = 0.467 bps`). Dropped taker volume is `0.026%` USD on native L1 and `0.041%` USD on `BTC/ETH` at the baseline requote threshold (`1.50 bp`). All orders sit in a transparent mempool before batch execution.

3. **Uniform-Price Frequent Batch Auction (`Regime C`, `100ms`):** Clears opposing flow at a single uniform price (`-0.861 bps` 1s markout on `BTC/ETH`; `-0.622 bps` on native L1), but incurs higher waiting drift (`0.895 bps` proxy; `0.722 bps` L1) and the highest `MEV = 0` All-In Net taker cost (`0.950 bps` proxy; `0.671 bps` L1).

4. **Dywe Threshold-Encrypted Micro-Batch (`Regime D`, `50ms` or `70ms` batch + `35ms` threshold decryption, `--d-batch-order hl`):**

   - **Adverse-selection reduction vs. FIFO (ranges across MM reaction speed and requote threshold):** At baseline (`mm_react = 10ms`, `requote_thr = 1.50 bp`), Dywe (`50ms` window, `35ms` decrypt) cuts 1s MM toxic markout by **`40.4%`** on native L1 (`-0.782` vs. `-1.312 bps`), **`38.9%`** on `BTC/ETH` proxy (`-1.049` vs. `-1.717 bps`), and **`23.8%`** on organic `SOLUSDT` (`--sniper-clip 0`, `-1.533` vs. `-2.011 bps`).

     Sweeping `mm_react` over `1–40ms` on native L1 (`requote_thr = 1.50 bp`) yields **`8.2%–51.1%`** (`51.1%` at `1ms`, `50.5%` at `2ms`, `46.3%` at `5ms`, `40.4%` at `10ms`, `25.6%` at `25ms`, `8.2%` at `40ms`). Tightening the requote threshold from `1.50 bp` to `0.50 bp` and `0.15 bp` (at `react = 10ms`) raises the reduction from `40.4%` to **`48.7%`** and **`51.0%`**.

     *Mechanism at fast MM speeds:* At `1ms`, the MM is faster than the synthetic probe (`2ms`), so FIFO records `0` stale sniper hits (`0/164`) yet still suffers `-1.271 bps` from organic informed flow. The `51.1%` reduction comes from `hl` ordering placing the MM's fresh quote ahead of incoming organic takers inside the same batch. **This effect is shared by Regime B (`56.6%` at `1ms`) and Regime D.** It is a property of priority batching with new-quote-first ordering rather than encryption. The jump from `0` to `19,963` stale hits between `1ms` and `2ms` reflects the reaction handicap (`mm_react - sniper_react`) crossing zero.

   - **Window length vs. decryption latency (isolating the cost of cryptography):** Comparing `D (50ms, 35ms dec)` (`s*net = 0.582 bps` on L1) against `B (70ms)` (`0.467 bps`) mixes two separate effects: window length (`50ms` vs. `70ms`) and decryption (`35ms`). Isolating decryption at a matched `70ms` window (`--window-ms 70 --b-window-ms 70`) demonstrates that **threshold decryption itself adds near-zero MM spread overhead**:
     - **BTC/ETH (14d, matched `70ms`):** `s*net = 0.685 bps` (`B`) vs. **`0.685 bps`** (`D, 35ms dec`, **`+0.000 bps`**) and `0.686 bps` (`D, 70ms dec`, `+0.001 bps`).
     - **Native L1 (9d, matched `70ms`):** `s*net = 0.467 bps` (`B`) vs. **`0.474 bps`** (`D, 35ms dec`, **`+0.007 bps` / `+1.5%`**) and `0.476 bps` (`D, 70ms dec`, `+0.009 bps`).
     - **At the product window (`50ms`, native L1):** `D` with `0ms` dec `s*net = 0.578 bps`, and with `35ms` dec `0.582 bps` (**`+0.004 bps`**).
     - Of the `+0.115 bps` `s*net` gap between `D50` and `B70` on L1, **`96.5%` (`+0.111 bps`) is caused by the shorter `50ms` responsiveness window, and `3.5%` (`+0.004 bps`) by `35ms` decryption.**

   - **Taker All-In cost parity at `MEV = 0`:** Under exogenous flow and zero-profit MM pricing, **All-In Net taker cost is nearly identical across Regimes A–D** (`0.922–0.950 bps` on `BTC/ETH`; `0.651–0.671 bps` on native L1; `D − B` is `-0.002 bps [~]` on `BTC/ETH` and `+0.005 bps` on native L1). Lower equilibrium spreads under batching are offset by waiting drift and reduced staleness capture. Any taker benefit in this study is therefore scenario-driven (Section 1.B).

   - **Dropped takers:** Under `pending=drop`, post-decrypt `hl` ordering keeps dropped taker volume at **`0.026%–0.041%` USD** at `requote_thr = 1.50 bp` (`0.036%` D50 / `0.026%` D70 on L1; `0.041%` D70 on BTC/ETH; `0.039%` D50 on SOL) and **`0.117%` USD** at `0.15 bp` (`D50` on L1). This eliminates the dead-gap losses of `cancel-first` while remaining comparable across FIFO, B, and D.

### B. Parameterized MEV Counterfactuals (`capture > 0%`, `--d-exposure-mode all`)

Binance USDⓈ-M Futures has no public mempool or on-chain sandwich MEV, so **any net taker cost advantage of Dywe over an open `70ms` batch (`D < B`) arises strictly from the parameterized counterfactual MEV grid**:

- **Uniform exposure mode (`--d-exposure-mode all`, default in `v0.3.5`):** Consistent with full pre-commit payload encryption and uniform 256-byte frame padding, `eff_leak = min(leak_ppm, capture_ppm)` applies uniformly to *all* large bursts (`≥$50k`), removing dependence on MM cancel frequency. The legacy `v0.3.4` `cancel-batch` mode remains available via CLI for comparison.
- **Large-order share bounds (`≥$50k`):** Unmerged raw `aggTrades` (lower bound) vs. 1-ms directional bursts via `merge_taker` (upper bound) account for **`57.8%–71.2%`** of `BTC/ETH` volume, **`47.9%–68.2%`** of native L1 volume, and **`39.8%–42.4%`** of `SOLUSDT` volume.
- **Break-even residual leakage (`eff_leak*`) and scenario savings:** At `100%` uniform exposure and `50%` MM MEV loss share, Dywe is cheaper for takers (`D < B`) only when encryption reduces effective leakage below `eff_leak*`:
  - **BTC/ETH (`D70` vs `B70`):** Zero-MEV parity (`Δ₀ = 0.000 bps`) yields `eff_leak* ≈ capture` (`10.0% / 25.0% / 50.0%` at `10% / 25% / 50%` capture). At `25%–50%` capture / `1.0 bp` impact / `10%` leak, scenario savings reach **`9.6%–22.0%`** (`-0.106` to `-0.282 bps`) at the `71.2%` burst upper bound, or **`8.1%–19.1%`** (`-0.087` to `-0.231 bps`) at the `57.8%` raw lower bound.
  - **Native L1 (`D50` vs `B70`):** Because `Δ₀ = +0.005 bps` at `MEV = 0`, `eff_leak*` is strictly below `capture`: `8.4%` (`10%` cap / `0.5 bp`), `9.2%` (`10%` / `1.0 bp`), `24.2%` (`25%` / `1.0 bp`), and `49.2%` (`50%` / `1.0 bp`). **For `D50`**, Dywe wins only if encryption trims at least `~1.6 pp` of capture at `0.5 bp` impact or `~0.8 pp` at `1.0 bp`. **For `D70`**, the thresholds are `8.0%` / `9.0%` (requiring `~2.0 pp` / `~1.0 pp` reduction). At `25%–50%` capture / `1.0 bp` / `10%` leak, savings reach **`11.7%–26.7%`** (`-0.096` to `-0.265 bps`) at the `68.2%` burst bound, or **`8.7%–21.0%`** (`-0.067` to `-0.187 bps`) at the `47.9%` raw bound.
  - **Organic `SOLUSDT` (`D50` vs `B70`, `--sniper-clip 0`):** `eff_leak*` is `7.4%–49.3%`; savings are **`4.0%–10.6%`** (`-0.058` to `-0.164 bps`) at `25%–50%` capture / `1.0 bp` / `10%` leak (`42.4%` burst bound).

  *Savings percentages equal `Δ(D − B) / All-In(B)`, where `All-In(B)` includes the scenario MEV term. Capture, impact, and leak are scenario inputs, not measurements.*

---

## 2. Architectural Comparison & Unified Threat Model

Every mid-price update and taker burst is replayed across four isolated matching engines under identical latency, reaction, and threshold parameters:

| Regime | Reference Architecture | Ordering & Clearing Mechanics | Pre-Trade Privacy | Primary Trade-Off (`MEV = 0`, L1, `10ms` react) |
| :--- | :--- | :--- | :--- | :--- |
| **Regime A** | **Continuous FIFO** *(baseline CLOB)* | Strict arrival-time execution; no batching; no cancel priority. | None (transparent) | High 1s MM markout (`-1.312` L1 / `-1.717` BTC/ETH) forces wide half-spreads (`1.112` / `1.517 bps`). |
| **Regime B** | **Open Priority Batch** *(`70ms` / `200ms` / `1000ms`, `hl` order)* | `New (ALO) -> Cancel -> Taker/Sniper` in discrete open windows. | None (transparent) | Strong MM protection (`0.467` L1 / `0.685` BTC/ETH half-spread), but large bursts (`48%–71%` of volume) are visible before execution. |
| **Regime C** | **Uniform-Price FBA** *(`100ms`)* | Orders accumulate and clear at one price; opposing flow nets internally. | None (transparent) | `100ms` hold raises waiting drift (`0.722` L1 / `0.895` BTC/ETH) and All-In cost (`0.671` / `0.950 bps`). |
| **Regime D** | **Dywe Protocol** *(IBTE `50ms`/`70ms` + `35ms` dec, `hl`)* | All order types encrypted pre-commit, decrypted in memory at `batch_close + decrypt_ms`, then sorted (`hl` default; `cancel-first` stress). | **Full pre-commit IBTE (`BLS12-381`)** *(parameterized `eff_leak` grid)* | At a matched `70ms` window, `35ms` decrypt adds `+0.000 bps` (BTC/ETH) and `+0.007 bps` (L1) to `s*net`. At `50ms`, the shorter window costs `+0.115 bps` `s*net` vs. `B70` on L1. |

### Unified Post-Decrypt Threat Model

- **Full pre-commit encryption:** All order payloads (`New ALO`, `Cancel`, `Taker`) are threshold-encrypted client-side under `BLS12-381` with uniform frame padding until `batch_close`.
- **Post-decrypt in-memory priority (`--d-batch-order hl`):** After the 5-of-7 quorum completes decryption in matching-engine memory at `clear_ns = batch_close + decrypt_ms`, the engine applies `New (ALO) -> Cancel -> Taker` ordering without exposing `ALO`/`IOC` flags or cancel headers before sequence lock.
- **No cryptography inside the simulator:** Zero `BLS12-381` pairing or decryption operations are executed. Regime D models `window_ms` + `decrypt_ms` + post-decrypt `hl` ordering. `leak_ppm` (`0%`, `10%`, `25%`, `50%`, clamped by `eff_leak = min(leak_ppm, capture_ppm)`) parameterizes potential residual side-channel leakage.
- **Calibration disclaimer:** Regimes `B (70ms)`, `B (200ms)`, and `B (1000ms)` are stylized batching models calibrated to representative intervals from public documentation (Hyperliquid and dYdX docs, accessed Nov 2026). They are not reproductions of any exchange matching engine.

---

## 3. Simulation & Mathematical Methodology

### 3.1 Separation of Empirical Data, Mechanical Parameters & Scenario Assumptions

1. **Historical component (measured):** Binance USDⓈ-M `bookTicker`, `trades`, and directional `aggTrades` across 30 pair-days (`$240.50B` pure taker volume).
2. **Mechanical component (parameterized, CLI-exposed):** Feed latency (`5ms`), MM quote recomputation (`--mm-react`, swept `1,2,5,10,25,40ms`), MM requote threshold (`--requote-thr-sub`, default `150` = `1.50 bp`, swept `15,50,150`), synthetic latency-arb probe (`2ms` reaction, `--sniper-thr-sub` default `200` = `2.00 bp`, `--sniper-clip` default `$25,000`, `0` disables), client-to-sequencer transit (`30ms`), sub-ms jitter (`0–999,999 ns`), batch windows (`--window-ms`, `--b-window-ms`, `--fba-window-ms`), decryption RTTs (`--decrypt 0,35,70`).
3. **Scenario component (parameterized grid):** Ambient MEV capture (`0%–100%`), per-order impact (`0.0–3.0 bps`), MM loss share (`50%` baseline), Regime D exposure mode (`all` vs. `cancel-batch`), residual leakage (`0%–50%`, clamped by `eff_leak = min(leak_ppm, capture_ppm)` so `capture = 0%` enforces zero MEV).
4. **Derived component:** Equilibrium half-spread `s*net`, single-denominator All-In Net taker cost, break-even leakage `eff_leak*`.

### 3.2 Fixed-Point Integer Arithmetic & Determinism

- All replay accounting uses fixed-point integers (`PRICE_SCALE = QTY_SCALE = 100_000_000`) with widening `i128` accumulators. Relative moves use integer **sub-bps** (`100 sub = 1.00 bps`, truncating toward zero) and `Q = 1,000` in Phase 2.
  *(`f64` is used only for the standard-error computation in the synthetic preflight test `shuffled_zero_spread` and for the mean-interval display in `format_book_diag`.)*
- Parallel daily workers compute isolated per-day, per-block accumulators merged in deterministic `(symbol, day, block)` order with fixed SplitMix64 seeds.

### 3.3 Native L1 `bookTicker` Density & Same-Day Proxy Validation (`--mid-source book|proxy`)

- **Availability and density:** No daily USDⓈ-M `bookTicker` archives exist on `data.binance.vision` after `2024-03-30`. Across the 9 pair-days of March 28–30, 2024, raw files contain **`74,023,564` quote rows** (`~8.22M` per pair-day). `push_mid_tracked()` collapses `4,312,026` same-ms updates and skips `66,441,550` quantity-only BBO updates, retaining **`3,269,988` distinct ms-level mid changes** (median interval `3 ms`; `2 ms` on `SOLUSDT`, `5–7 ms` on `BTCUSDT`/`ETHUSDT`).
- **Same-day comparison (`book` vs. `proxy`, Mar 28–30, 2024):** `BidAskProxy` on the identical dates (`22,327,383` observations, `3.3%` crossed, mean valid spread `0.758 bps`) matches native `bookTicker` within `0.013 bps` on FIFO while showing a smaller batch markout reduction (`33.9%` vs. `40.4%` on `D`; `39.9%` vs. `49.2%` on `B`) due to bid-ask bounce correlation when the proxy is constructed from the same trade stream replayed as takers. Relative cross-regime reductions on 2026 proxy data should therefore be interpreted as an approximate range (`~34%–40%` for `D`) rather than an exact point estimate.

| Metric (Mar 28–30, 2024, `BTC+ETH+SOL`, 9 pair-days) | Native L1 (`book`) | Same-day `BidAskProxy` | Delta (`Proxy − Book`) |
| :--- | :---: | :---: | :---: |
| **Regime A (FIFO) 1s MM markout** | `-1.312 bps` `[-1.425, -1.216]` | `-1.325 bps` `[-1.439, -1.224]` | `-0.013 bps` |
| **Regime B (`70ms`, `hl`)** *(reduction vs. A)* | `-0.667 bps` *(`49.2%`)* | `-0.796 bps` *(`39.9%`)* | `-0.129 bps` |
| **Regime D (`50ms`, `35ms` dec, `hl`)** *(reduction vs. A)* | `-0.782 bps` *(`40.4%`)* | `-0.876 bps` *(`33.9%`)* | `-0.094 bps` |
| **Zero-MEV taker All-In delta (`D − B`)** | `+0.005 bps` (`B < D`) | `+0.002 bps` (`B < D`) | `-0.003 bps` |

- **2026 proxy diagnostics (`--mid-mode bidask`):**
  - `BTC/ETH` (14 pair-days): `84,131,187` observations, `0.8%` crossed (`695,039`), mean valid spread `0.708 bps`.
  - `SOLUSDT` (7 pair-days): `15,365,789` observations, `0.0%` crossed (`1,305`), mean valid spread `1.903 bps`.

### 3.4 Equilibrium Spread, All-In Identity, Break-Even Leakage & Placebo Control

The engine solves for the zero-profit half-spread `s*net` (`--rebate-sub` default `50` = `0.50 bps` rebate paid to the MM; `--opex-sub` default `30` = `0.30 bps` hedging OpEx):

$$
s^*_{\text{net}} = \max\Big(0,\enspace -\text{Markout}_{1\text{s}} + \text{MEV}_{\text{MM}} - \text{Rebate} + \text{OpEx}\Big)
$$

Under `--wait-cohort all` (default), all four components of **All-In Net Cost** share one volume denominator (`tk_n`), guaranteeing exact linear additivity:

$$
\text{All-In Net} =
\underbrace{\text{[1] Spread Paid}}_{\text{effective } s^*_{\text{net}}}
+
\underbrace{\text{[2] Taker Stale}}_{\text{quote staleness}}
+
\underbrace{\text{[3] Wait (All)}}_{\text{batch/decrypt drift}}
+
\underbrace{\text{[4] MEV}_{\text{Taker}}}_{\text{scenario sandwich impact}}
$$

The **break-even residual leakage** `eff_leak*` (where `All-In(D) == All-In(B)`) for a given capture, impact, and Regime D exposure fraction `exp` is solved analytically in `breakeven_leak_ppm()`:

$$
\text{eff\textunderscore leak}^* =
\frac{\text{MEV}_B(\text{capture}, \text{impact}) - \Delta_0}
{\text{UnitMEV}_D(\text{impact}, \text{exp})},
\qquad
\Delta_0 = \text{All-In}_0(D) - \text{All-In}_0(B)
$$

- **Joint block bootstrap:** Synchronized 3-hour blocks resampled jointly across pairs (`56` blocks for 7-day runs, `24` for the 3-day L1 run; `1,000` draws, 95% CI).
- **Circular-shift placebo (`600s` / `3600s`):** Shifting taker timestamps with the price path fixed destroys `96.8%` of Regime A's 1s markout on `BTC/ETH` (`-1.717` to `-0.055 bps`), `97.3%` on native L1 (`-1.312` to `-0.036 bps`), and `99.7%` on organic `SOLUSDT` (`-2.011` to `-0.006 bps`, `--sniper-clip 0`). Simulated markout therefore arises from temporal alignment between order flow and price moves rather than static drift.
- **Simulated latency-arb probe (`StaleSnipe`):** Hits come from a deterministic parameterized probe (`2.0 bps` trigger, `2ms` reaction, `$25,000` clip), not from observed exchange order IDs. Setting `--sniper-clip 0` (`v035_SOL_nosniper_all`) or `mm_react = 1ms` (`0` stale hits on L1) isolates organic flow.

---

## 4. Pre-Computed Audit Reports (`results/`)

All 8 `v0.3.5` scenarios are committed under `results/` (each directory contains `report.md`, interactive `ablation_report.html`, and full `breakeven.csv`; checksums in `results/SHA256SUMS.txt`):

| Directory | Scenario & Data Source | Key Takeaway |
| :--- | :--- | :--- |
| [`results/v035_L1_book_all/`](./results/v035_L1_book_all/report.md) | **Native L1 (`BTC+ETH+SOL`, 9d)**<br>B `70ms` vs D `50ms`, `exp=all`, `drop`, `req=1.50bp` | `$62.93B` pure taker vol (`47.9%–68.2%` `≥$50k`), `40.4%` markout reduction vs. FIFO, `0.036%` dropped USD (`D50`), scenario savings `11.7%–26.7%` (`68.2%` bound; `8.7%–21.0%` raw bound), `eff_leak* = 8.4%–49.6%`. |
| [`results/v035_L1_win70_decrypt/`](./results/v035_L1_win70_decrypt/report.md) | **Native L1 decrypt isolation (matched `70ms`)**<br>B `70ms` vs D `70ms` (`0/35/70ms` dec) | Pure `35ms` decrypt cost: **`+0.007 bp`** `s*net` (`0.467` to `0.474`, `+1.5%`); `0.026%` dropped USD (identical to B). |
| [`results/v035_L1_req15/`](./results/v035_L1_req15/report.md) | **Native L1 tight requote (`0.15 bp`)** | **`51.0%`** markout reduction (`-0.591` vs. `-1.207 bp`), `0.117%` dropped USD, `54.8%` cancel-batch share, `eff_leak* = 9.0%–49.8%`. |
| [`results/v035_L1_req50/`](./results/v035_L1_req50/report.md) | **Native L1 medium requote (`0.50 bp`)** | **`48.7%`** markout reduction (`-0.629` vs. `-1.227 bp`), `0.050%` dropped USD, `45.0%` cancel-batch share, `eff_leak* = 8.6%–49.6%`. |
| [`results/v035_L1_react_sweep_hold/`](./results/v035_L1_react_sweep_hold/report.md) | **Native L1 MM reaction sweep (`1–40ms`)**, `hold` | `D` reduction **`8.2%–51.1%`**. Phase 2 frontier, placebo, and CI blocks evaluate `mm_react[0]` (`1ms`). |
| [`results/v035_BTCETH_all/`](./results/v035_BTCETH_all/report.md) | **Primary `BTC+ETH` (14d, 2026 proxy)**<br>B `70ms` vs D `50ms`, `drop` | `$164.93B` pure taker vol (`57.8%–71.2%` `≥$50k`), `38.9%` reduction vs. FIFO, `-0.002 bp [~]` at `MEV=0`, scenario savings `9.8%–22.2%` (`71.2%` bound). |
| [`results/v035_BTCETH_win70_decrypt/`](./results/v035_BTCETH_win70_decrypt/report.md) | **`BTC+ETH` decrypt isolation (matched `70ms`)** | Pure `35ms` decrypt cost **`+0.000 bp`** (`0.685` vs. `0.685`; All-In `0.924` vs. `0.924`); savings `9.6%–22.0%` (`71.2%` bound; `8.1%–19.1%` raw bound). |
| [`results/v035_SOL_nosniper_all/`](./results/v035_SOL_nosniper_all/report.md) | **Organic `SOLUSDT` (7d, `--sniper-clip 0`)**<br>B `70ms` vs D `50ms` | `$12.63B` pure organic flow (`39.8%–42.4%` `≥$50k`), **`23.8%`** reduction (`-1.533` vs. `-2.011 bp`), `0.039%` dropped USD, `99.7%` removed by `600s` placebo. |

---

## 5. Consolidated Benchmark Summary (`v0.3.5`, `exp=all`, `wait=all`)

### 5.1 Native L1 `bookTicker` (`BTC+ETH+SOL`, 9 pair-days, `MEV = 0`, `react = 10ms`, `req = 1.50bp`)

| Architecture Regime | Hist Taker Vol | MM Vol (incl. Probe) | Dropped Takers (USD%) | 1s MM Markout | `s*net` | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net | Probe Hit/Miss |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **A: Continuous FIFO** | `$62.93B` | `$64.11B` | `216 (0.004%)` | `-1.312` | `1.112` | `1.112` | `-1.068` | `0.626` | **`0.670`** | `35,107 / 48` |
| **B: Batch `70ms` (`hl`)** | `$62.91B` | `$64.26B` | `322 (0.026%)` | `-0.667` | `0.467` | `0.467` | `-0.517` | `0.701` | **`0.651`** | `3,357 / 26` |
| **C: Uniform-FBA (`100ms`)** | `$62.92B` | `$59.46B` | `269 (0.013%)` | `-0.622` | `0.422` | `0.390` | `-0.441` | `0.722` | **`0.671`** | `2,180 / 22` |
| **D: Dywe `50ms` (`0ms` dec)** | `$62.91B` | `$64.21B` | `572 (0.036%)` | `-0.778` | `0.578` | `0.578` | `-0.609` | `0.683` | **`0.653`** | `4,605 / 45` |
| **D: Dywe `50ms` (`35ms` dec)** | **`$62.91B`** | **`$64.36B`** | **`572 (0.036%)`** | **`-0.782`** | **`0.582`** | **`0.582`** | **`-0.674`** | **`0.748`** | **`0.656`** | **`4,517 / 46`** |
| **D: Dywe `50ms` (`70ms` dec)** | `$62.91B` | `$64.45B` | `572 (0.036%)` | `-0.785` | `0.585` | `0.585` | `-0.718` | `0.792` | **`0.659`** | `4,444 / 48` |
| **D: Dywe `70ms` (`35ms` dec)** | **`$62.91B`** | **`$64.39B`** | **`322 (0.026%)`** | **`-0.674`** | **`0.474`** | **`0.474`** | **`-0.577`** | **`0.761`** | **`0.658`** | **`3,339 / 25`** |

### 5.2 Primary `BTC+ETH` and Matched `70ms` Decrypt Isolation (14 pair-days, 2026 proxy, `MEV = 0`)

| Architecture Regime | Hist Taker Vol | MM Vol (incl. Probe) | Dropped Takers (USD%) | 1s MM Markout | `s*net` | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net | Probe Hit/Miss |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **A: Continuous FIFO** | `$164.90B` | `$167.72B` | `1,595 (0.022%)` | `-1.717` | `1.517` | `1.517` | `-1.266` | `0.677` | **`0.928`** | `71,633 / 97` |
| **B: Batch `70ms` (`hl`)** | `$164.87B` | `$167.90B` | `1,797 (0.041%)` | `-0.885` | `0.685` | `0.685` | `-0.636` | `0.874` | **`0.924`** | `6,308 / 60` |
| **C: Uniform-FBA (`100ms`)** | `$164.88B` | `$145.80B` | `1,650 (0.029%)` | `-0.861` | `0.661` | `0.575` | `-0.520` | `0.895` | **`0.950`** | `4,223 / 43` |
| **D: Dywe `70ms` (`0ms` dec)** | `$164.87B` | `$167.90B` | `1,797 (0.041%)` | `-0.885` | `0.685` | `0.685` | `-0.636` | `0.874` | **`0.924`** | `6,308 / 60` |
| **D: Dywe `70ms` (`35ms` dec)** | **`$164.87B`** | **`$168.18B`** | **`1,797 (0.041%)`** | **`-0.885`** | **`0.685`** | **`0.685`** | **`-0.717`** | **`0.956`** | **`0.924`** | **`6,080 / 56`** |
| **D: Dywe `70ms` (`70ms` dec)** | `$164.87B` | `$168.39B` | `1,797 (0.041%)` | `-0.886` | `0.686` | `0.686` | `-0.769` | `1.008` | **`0.924`** | `5,728 / 55` |
| **D: Dywe `50ms` (`35ms` dec)** | **`$164.85B`** | **`$168.20B`** | **`2,153 (<0.06%)`** | **`-1.049`** | **`0.849`** | **`0.849`** | **`-0.878`** | **`0.951`** | **`0.922`** | **`8,523 / 79`** |

### 5.3 Sensitivity Sweeps on Native L1 `bookTicker`

#### A) MM Reaction Latency Sweep (`--mm-react 1,2,5,10,25,40`, `pending=hold`, `requote_thr = 1.50 bp`)

| `mm_react` | `A (FIFO)` 1s Markout | `B (70ms, hl)` 1s Markout | `B` Reduction | `D (50ms, 35ms dec)` 1s Markout | `D` Reduction | `A` StaleSnipe Hits |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`1 ms`** | `-1.271` | `-0.551` | `56.6%` | **`-0.622`** | **`51.1%`** | `0` |
| **`2 ms`** | `-1.291` | `-0.569` | `55.9%` | **`-0.639`** | **`50.5%`** | `19,963` |
| **`5 ms`** | `-1.301` | `-0.613` | `52.9%` | **`-0.698`** | **`46.3%`** | `29,520` |
| **`10 ms`** | `-1.312` | `-0.667` | `49.2%` | **`-0.782`** | **`40.4%`** | `35,107` |
| **`25 ms`** | `-1.332` | `-0.818` | `38.6%` | **`-0.991`** | **`25.6%`** | `41,499` |
| **`40 ms`** | `-1.337` | `-0.983` | `26.5%` | **`-1.228`** | **`8.2%`** | `44,959` |

*Both B and D benefit; the effect is a property of new-quote-first priority batching, not of encryption.*

#### B) MM Requote Threshold Sweep (`--requote-thr-sub 15, 50, 150`, `pending=drop`, `react = 10ms`)

| `requote_thr` | `A (FIFO)` | `B (70ms)` | `D (50ms, 35ms dec)` | `D` Reduction | `D` Dropped USD% | Cancel-Batch Large % (`D`) |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`1.50 bp`** | `-1.312` | `-0.667` | **`-0.782`** | **`40.4%`** | `0.036%` | `26.7%` |
| **`0.50 bp`** | `-1.227` | `-0.506` | **`-0.629`** | **`48.7%`** | `0.050%` | `45.0%` |
| **`0.15 bp`** | `-1.207` | `-0.463` | **`-0.591`** | **`51.0%`** | `0.117%` | `54.8%` |

### 5.4 Break-Even Frontier under Uniform Exposure (`--d-exposure-mode all`, `All-In(D) − All-In(B)`, bps)

> **Heterogeneous comparison note:** The left value is `BTC/ETH` **`D70`** vs. `B70` (14d proxy). The value in brackets is native L1 **`D50`** vs. `B70` (9d). Different `D` windows and datasets mean the two columns are not directly comparable. Scenario parameters: `50%` MM MEV share, `10%` leak (`eff_leak = min(leak, capture)`), `0.50 bps` rebate paid to MM, `0.30 bps` OpEx (`35ms` decrypt). **All rows with capture > 0% are parameterized scenarios, not measurements.**

| Large-Burst MEV Capture (`≥$50k`) \ Impact | `0.000 bp` | `0.500 bp` | `1.000 bp` | `2.000 bp` | `3.000 bp` |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **0%** *(zero-MEV)* | `0.000 ~` `[+0.005 B<D]` | `0.000 ~` `[+0.005 B<D]` | `0.000 ~` `[+0.005 B<D]` | `0.000 ~` `[+0.005 B<D]` | `0.000 ~` `[+0.005 B<D]` |
| **10%** | `0.000 ~` `[+0.005 B<D]` | `0.000 ~` `[+0.005 B<D]` | `0.000 ~` `[+0.005 B<D]` | `0.000 ~` `[+0.005 B<D]` | `0.000 ~` `[+0.004 B<D]` |
| **25%** | `0.000 ~` `[+0.005 B<D]` | **`-0.053 D<B`** `[-0.045]` | **`-0.106 D<B`** `[-0.096]` | **`-0.212 D<B`** `[-0.197]` | **`-0.318 D<B`** `[-0.298]` |
| **50%** | `0.000 ~` `[+0.005 B<D]` | **`-0.141 D<B`** `[-0.129]` | **`-0.282 D<B`** `[-0.265]` | **`-0.564 D<B`** `[-0.535]` | **`-0.847 D<B`** `[-0.805]` |
| **75%** | `0.000 ~` `[+0.005 B<D]` | **`-0.229 D<B`** `[-0.214]` | **`-0.459 D<B`** `[-0.433]` | **`-0.917 D<B`** `[-0.872]` | **`-1.376 D<B`** `[-1.311]` |
| **100%** | `0.000 ~` `[+0.005 B<D]` | **`-0.317 D<B`** `[-0.298]` | **`-0.635 D<B`** `[-0.602]` | **`-1.270 D<B`** `[-1.210]` | **`-1.905 D<B`** `[-1.818]` |

### 5.5 Break-Even Residual Leakage `eff_leak*` (`All-In(D, 35ms) == All-In(B, 70ms)`)

*Dywe is cheaper for takers whenever actual residual leakage `eff_leak < eff_leak*` (`100%` uniform exposure, `50%` MM MEV share). Full multi-exposure tables are printed in each `report.md` and `ablation_report.html`. The `Cancel-Batch` column in `breakeven_leak_ppm()` is an analytical approximation scaling `mm_large_n`/`tk_large_n` by the observed cancel-batch share.*

| Ambient Capture / Impact | Native L1 `D50` (`req=1.50bp`) | Native L1 `D50` (`req=0.15bp`) | Native L1 `D70` (`req=1.50bp`) | BTC/ETH `D70` (`req=1.50bp`) | SOL organic `D50` |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **`10%` / `0.50 bp`** | `8.4%` | `9.0%` | `8.0%` | `10.1%` | `7.4%` |
| **`10%` / `1.00 bp`** | `9.2%` | `9.5%` | `9.0%` | `10.0%` | `8.7%` |
| **`25%` / `1.00 bp`** | `24.2%` | `24.5%` | `24.0%` | `25.0%` | `23.7%` |
| **`50%` / `1.00 bp`** | `49.2%` | `49.5%` | `49.0%` | `50.0%` | `48.7%` |
| **`50%` / `2.00 bp`** | `49.6%` | `49.8%` | `49.5%` | `50.0%` | `49.3%` |

---

## 6. Built-In Verification & Unit Test Suite (17 Pre-Flight Invariants + 18 Unit Tests)

Executed via `cargo test --release` (all **18 unit tests** pass in `<0.1s`) and automatically on every run (**17 synthetic invariant checks**):

1. `t_causality`: Mutating future ticks after `T` leaves all fills before `T` bit-for-bit identical across all 4 regimes.
2. `t_identity`: `Regime D (dec=0, cancel-first)` equals `Regime B (cancel-first)` at the `i128` accumulator level.
3. `t_d_post_decrypt_hl_order`: `Regime D (dec=0, hl)` equals `Regime B (hl)` bit-for-bit and removes same-batch cancel-gap drops.
4. `t_hl_batch_order`: `New (ALO) -> Cancel -> Taker` ordering removes dead gaps vs. `cancel-first`.
5. `t_accounting_reconciliation`: Exact volume conservation and exact additive `All-In Net` identity.
6. `t_wait_cohort_all_identity`: Single-denominator (`tk_n`) additivity under `--wait-cohort all` vs. `large`.
7. `t_fba_sniper_attribution`: Late FBA snipers after same-batch MM requotes are attributed to `late_fresh`.
8. `t_placebo_breaks_link`: Circular timestamp shifting destroys flow-price alignment while preserving counts and notional.
9. `t_book_ticker_diag`: Raw row counting, deduplication counters, and 1-ms histogram median interval.
10. `t_zero_capture_zero_mev`: Zero MEV extraction when `capture = 0%`, regardless of `leak_ppm`.
11. `t_d_exposure_modes_and_breakeven`: `ExposureMode::All` vs. `CancelBatch` scaling and `breakeven_leak_ppm()` math (covering the zero-spread/equal-baseline boundary case `b == d`, `Δ₀ = 0`, 100% exposure).
12. `t_dropped_precision`: `fmt_dropped_cell()` resolves sub-`0.1%` shares at `0.001%`.
13. `t_pending_drop_policy`, `t_bidask_mid_proxy`, `t_mev_share_sum`, `t_sniper_protection`, `t_shuffled_zero_spread`, `t_html_escape`: Drop vs. hold semantics, proxy diagnostics, MEV share conservation, stale-snipe protection, randomized-flow zero-bias control, and HTML sanitization.

---

## 7. Data Sources & How to Reproduce Locally

### 7.1 Market Data & Build Environment

- **Market data:** Binance Public Data Vision (`https://data.binance.vision/data/futures/um/daily/`):
  - **2026 dataset (`./data/`):** `BTCUSDT`, `ETHUSDT`, `SOLUSDT` (`2026-11-10` to `2026-11-16`: `trades/`, `aggTrades/`).
  - **Native L1 and same-day proxy dataset (`./data_l1/`):** `BTCUSDT`, `ETHUSDT`, `SOLUSDT` (`2024-03-28` to `2024-03-30`: `bookTicker/`, `trades/`, `aggTrades/`).
- **Build environment:** Apple Silicon (`macOS`), `rustc 1.100.0 (0ef1bc386 2026-11-09)`, zero external crate dependencies (`Cargo.lock` committed), release commit `939fc5c`, artifact SHA-256 checksums recorded in `results/SHA256SUMS.txt`.

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

#### Step 2: Run the Test Suite & Regenerate All 8 Reports

```bash
cargo test --release
mkdir -p results

cargo run --release -- --threads 6 --allow-reconstructed --window-ms 50 --b-window-ms 70 --b-batch-order hl --d-batch-order hl --d-exposure-mode all --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/v035_BTCETH_all

cargo run --release -- --threads 6 --allow-reconstructed --window-ms 70 --b-window-ms 70 --decrypt 0,35,70 --d-exposure-mode all --pending-policy drop --mid-mode bidask --out-dir results/v035_BTCETH_win70_decrypt

cargo run --release -- --threads 6 --allow-reconstructed --symbols SOLUSDT --sniper-clip 0 --window-ms 50 --b-window-ms 70 --b-batch-order hl --d-batch-order hl --d-exposure-mode all --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/v035_SOL_nosniper_all

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --b-window-ms 70 --d-exposure-mode all --pending-policy drop --out-dir results/v035_L1_book_all

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --window-ms 70 --b-window-ms 70 --decrypt 0,35,70 --d-exposure-mode all --pending-policy drop --out-dir results/v035_L1_win70_decrypt

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --b-window-ms 70 --requote-thr-sub 15 --d-exposure-mode all --pending-policy drop --out-dir results/v035_L1_req15

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --b-window-ms 70 --requote-thr-sub 50 --d-exposure-mode all --pending-policy drop --out-dir results/v035_L1_req50

cargo run --release -- --threads 6 --data-dir ./data_l1 --symbols BTCUSDT,ETHUSDT,SOLUSDT --start 2024-03-28 --days 3 --mid-source book --b-window-ms 70 --mm-react 1,2,5,10,25,40 --d-exposure-mode all --pending-policy hold --out-dir results/v035_L1_react_sweep_hold
```

### 7.3 Complete CLI Reference

| Argument | Description |
| --- | --- |
| `--data-dir <PATH>` | Directory with extracted Binance CSVs (default: `./data`). |
| `--out-dir <PATH>` | Output directory for `report.md`, `ablation_report.html`, `breakeven.csv` (default: `.`). |
| `--symbols <LIST>` | Comma-separated symbols (default: `BTCUSDT,ETHUSDT`). |
| `--start <YYYY-MM-DD>` / `--days <N>` | Start date and number of consecutive days (default: `2026-11-10`, `7`). |
| `--window-ms <MS>` | Regime D micro-batch window in ms (default: `50`). |
| `--b-window-ms <MS>` | Regime B batch window in ms (default: `0` = same as `--window-ms`). |
| `--fba-window-ms <MS>` | Regime C FBA window in ms (default: `100`). |
| `--b-batch-order <hl\|cancel-first>` | Intra-batch ordering for Regime B (default: `hl`). |
| `--d-batch-order <hl\|cancel-first>` | Post-decrypt ordering for Regime D (default: `hl`). |
| `--d-exposure-mode <all\|cancel-batch>` | Regime D MEV exposure (`all` = uniform; `cancel-batch` = legacy; default: `all`). |
| `--decrypt <LIST>` | Post-commit threshold decryption RTTs in ms (default: `0,35,70`). |
| `--mm-react <LIST>` | MM quote recomputation latencies in ms (default: `10`). |
| `--requote-thr-sub <INT>` | MM requote threshold in sub-bps (`100 sub = 1.00 bp`; default: `150`). |
| `--sniper-thr-sub <INT>` | Synthetic sniper trigger threshold in sub-bps (default: `200`). |
| `--sniper-clip <USD>` | Probe notional clip; `0` disables the probe (default: `25000`). |
| `--large-usd <INT>` | USD threshold for MEV-eligible large bursts (default: `50000`). |
| `--pending-policy <hold\|drop>` | Handling of takers arriving during an MM requote gap (default: `hold`). |
| `--wait-cohort <all\|large>` | Denominator for waiting drift in All-In Net (default: `all`). |
| `--mid-source <auto\|book\|proxy>` | Native `bookTicker`, trade-derived `proxy`, or `auto` fallback (default: `auto`). |
| `--mid-mode <bidask\|trade>` | Proxy midpoint formula (default: `bidask`). |
| `--allow-reconstructed` | Allows `auto` mode to fall back to the trade proxy when `bookTicker` is missing. |
| `--rebate-sub <INT>` / `--opex-sub <INT>` | Maker rebate paid to MM and hedging OpEx in sub-bps (defaults: `50`, `30`). |
| `--placebo-shifts <LIST>` | Circular timestamp shifts in ms (default: `600000,3600000`). |
| `--block-hours <H>` | Block length for the joint block bootstrap (default: `3`). |
| `--bootstrap <N>` | Number of bootstrap draws (default: `1000`). |
| `--threads <N>` | Parallel worker threads (default: `4`). |
| `--skip-preflight` / `--preflight-only` | Skip the 17 pre-flight checks, or exit right after them. |

---

## 8. Repository Structure

```text
Dywe_Rust_Microstructural_Ablation_Backtest/
├── data/                                    # Local 2026 Binance CSVs (gitignored)
├── data_l1/                                 # Local 2024 native L1 bookTicker & trades CSVs (gitignored)
├── results/                                 # Pre-computed v0.3.5 reports
│   ├── SHA256SUMS.txt                       # SHA-256 hashes of source & reports
│   ├── v035_BTCETH_all/                     # Primary BTC+ETH (B70 vs D50, exp=all)
│   ├── v035_BTCETH_win70_decrypt/           # BTC+ETH matched 70ms decrypt isolation
│   ├── v035_L1_book_all/                    # Native L1 (B70 vs D50, exp=all)
│   ├── v035_L1_win70_decrypt/               # Native L1 matched 70ms decrypt isolation
│   ├── v035_L1_req15/                       # Native L1 tight requote (0.15 bp)
│   ├── v035_L1_req50/                       # Native L1 medium requote (0.50 bp)
│   ├── v035_L1_react_sweep_hold/            # Native L1 MM reaction sweep (1–40ms, pending=hold)
│   └── v035_SOL_nosniper_all/               # Organic SOLUSDT (--sniper-clip 0, exp=all)
├── src/
│   └── main.rs                              # Simulation engine, CLI, reporting & 18 tests
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.MD
```

---

## 9. Scope, Assumptions & Limitations

- **Single representative MM, top-of-book only:** One zero-profit MM clears the replayed taker stream at an endogenous spread `s*net`; no multi-MM competition and no multi-level L2 depth replenishment.
- **Model-derived spreads and requote sensitivity:** Equilibrium half-spreads (`s*net ≈ 0.26–1.52 bps`) are zero-profit model outputs under stylized fee and OpEx assumptions. They are not comparable to real Binance Futures tick spreads (`~0.01–0.10 bps`). The MM requotes only when the mid moves by at least `--requote-thr-sub` (default `1.50 bp`), which strongly affects absolute markout levels and the cancel-batch share; `0.50 bp` and `0.15 bp` are swept in `v035_L1_req50` and `v035_L1_req15`.
- **Exogenous order flow:** Historical taker executions are replayed causally and do not shrink or reroute in response to quoted spread width.
- **1-ms burst aggregation (`merge_taker`):** Same-millisecond, same-direction executions are merged into one burst. Since `aggTrades` already groups single-price fills, unmerged rows (`47.9%` L1 / `57.8%` BTC/ETH) are a lower bound and 1-ms bursts (`68.2%` / `71.2%`) an upper bound on the `≥$50k` volume share. "Large order" in this study refers to a 1-ms burst rather than a single exchange order.
- **Shared effect of `hl` ordering:** The markout reduction at fast MM reaction (`1ms`) comes from new-quote-first priority batching and applies to Regime B as well as D. Only the encryption-related scenario terms are specific to D.
- **Sweep evaluation scope:** In multi-value `--mm-react` runs, Phase 2 frontier tables, placebo checks, and bootstrap sign checks evaluate `mm_react[0]` (`1ms`); all speeds appear in Phase 1 and cohort tables. Headline `-1.312 bps` FIFO figures come from `react = 10ms` runs.
- **Sample heterogeneity:** Native L1 exists only for 3 days in March 2024 (spanning Good Friday). 2026 results use the trade-derived proxy, which understates batch markout reduction relative to native data on the same dates. Bootstrap CIs (3-hour blocks) quantify drift sampling variance under fixed parameters and do not cover volatility autocorrelation or uncertainty in capture, impact, leak, reaction times, or windows.
- **Offline rendering:** `ablation_report.html` loads `Chart.js 4.4.1` from a CDN (charts require internet access; tables render offline).

---

## 10. Research Conclusion

**What this replay measures (`MEV = 0`):**

- **Adverse-selection reduction vs. FIFO:** Post-decrypt `New (ALO) -> Cancel -> Taker` ordering lets Dywe (`Regime D`) cut 1s MM toxic markout by **`8.2%–51.1%`** across `1–40ms` MM reaction latencies on native L1 at `requote_thr = 1.50 bp` (`40.4%` at `10ms`; `48.7%–51.0%` at `0.50–0.15 bp`), **`38.9%`** on `BTC/ETH` proxy, and **`23.8%`** on organic `SOLUSDT`. Dropped taker volume stays at `0.026%–0.041%` USD at baseline (`0.117%` at `0.15 bp`). Regime B achieves a comparable or larger reduction; this markout effect is a property of priority batching rather than encryption.
- **Threshold decryption adds near-zero MM spread cost:** At a matched `70ms` window, `35ms` decryption adds `+0.000 bps` (BTC/ETH) and `+0.007 bps` (`+1.5%`, native L1) to `s*net`; at the `50ms` product window on L1 it adds `+0.004 bps`. The `+0.115 bps` gap of `D50` vs. `B70` on L1 is `96.5%` attributable to the shorter window.
- **Static taker parity at `MEV = 0`:** All-In Net taker cost is nearly identical across Regimes A–D (`0.651–0.671 bps` L1; `0.922–0.950 bps` BTC/ETH).

**What depends on scenario assumptions and remains to be validated in production:**

- Any taker All-In advantage (`D < B`) depends on assumed ambient mempool MEV capture, per-order impact, and residual leakage (`eff_leak < eff_leak*`), requiring calibration against live on-chain CLOB MEV telemetry.
- Empirical p50/p95/p99 `BLS12-381` threshold-decryption latency across geographically distributed validators on the Dywe multi-region Devnet, endogenous MM spread competition, and multi-level L2 depth under live flow.

---

## 11. Claims Not Supported by This Study

To prevent over-citation in downstream documents, the following statements are **not** supported:

1. *"Dywe reduces taker cost"* without stating the scenario assumptions (capture, impact, leak) and the `eff_leak < eff_leak*` condition. At `MEV = 0`, there is no taker benefit.
2. *"0.0% dropped taker volume."* The supported figures are `0.026%–0.041%` USD at `1.50 bp` and `0.117%` at `0.15 bp`, comparable across FIFO, B, and D.
3. *"Encryption narrows MM spreads."* At matched windows it adds `0.000–0.007 bps`; the spread differences in the headline benchmark come from window length.
4. *"Dywe is better than open batches for market makers."* Against `B70`, the Dywe `50ms` MM spread is wider (`+0.115 bps` on L1); Dywe improves upon FIFO and, on taker cost at `MEV = 0`, matches the alternatives.
5. *"The markout reduction is due to sniper protection or encryption."* At `1ms` MM reaction it persists without stale snipes and is shared by Regime B.
6. Any figure presented as live, production, or Devnet performance. All results are historical simulation outputs on Binance data; no cryptography is executed.
7. Pooled statistics across the 2024 native and 2026 proxy samples.
