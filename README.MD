
# Dywe Microstructural Ablation Backtest (`dywe-ablation-sim`)

**Deterministic, Fixed-Point (`i64`/`i128`) Causal Orderbook & Execution Replay Engine** benchmarking the **Dywe Exchange** market architecture against stylized decentralized CLOB and batch-auction designs across **14 pair-days** (`7 days × 2 pairs`) of historical Binance USDⓈ-M Futures market data (`BTCUSDT` and `ETHUSDT`, November 10–16, 2026: **6,384,781 trade-derived bid/ask midpoint proxy updates** and **11,472,331 directional aggregated taker executions**).

---

## 1. Executive Thesis: Solving the Decentralized Orderbook Trilemma

Institutional market makers, quantitative trading desks, and vault managers evaluating decentralized perpetual exchanges face an architectural trade-off between **latency-arbitrage protection**, **execution responsiveness**, and **pre-trade order privacy**:

1. **Continuous Transparent CLOB (`Regime A`):** Delivers continuous execution, but exposes resting market-maker (MM) quotes to stale-quote sniping (`-1.717 bps` 1s toxic markout; `71,633` stale hits) and leaves large directional orders (`≥$50k`) visible to content-based MEV strategies. MMs compensate by widening equilibrium half-spreads to `1.517 bps` (`~3.03 bps` full spread).
2. **Open Micro-Batches with Post-Only / Cancel Priority (`Regime B` — Hyperliquid-like `70ms` / Stress `200ms` / dYdX-like `1000ms`):** Prioritizes `New (ALO/post-only)` and `Cancel` messages ahead of takers (`--b-batch-order hl`), cutting toxic MM adverse selection by **48.5%** at `70ms` (`-0.885 bps`) with `0.0%` dropped taker volume. However, the mempool remains transparent, leaving large directional sweeps (`71.2%` of total taker volume) and copy-vault rebalances exposed to MEV extraction.
3. **Uniform-Price Frequent Batch Auctions (`Regime C`, `100ms`):** Clears opposing flow internally at a single auction price (`-0.861 bps` 1s markout), but incurs `0.895 bps` volume-weighted waiting drift and `0.950 bps` All-In Net taker cost.

### How Dywe Solves This (`Regime D`)

**Dywe** decouples **pre-commit order privacy** from **post-decrypt in-memory matching** via a `50ms` micro-batch pipeline:

- **Pre-Commit Privacy (`IBTE` on `BLS12-381`):** Order direction, limit price, and size remain threshold-encrypted until `batch_close`, shielding large orders and copy-vault rebalances from mempool MEV searchers.
- **Plaintext `Cancel-Header` + Post-Decrypt Priority (`--d-batch-order hl`):** Because threshold decryption completes in matching-engine memory at `clear_ns = batch_close + decrypt_ms` (`35ms` baseline / `70ms` stress) *before* the first fill executes, Dywe applies deterministic `New (ALO) -> Cancel -> Taker` ordering without leaking ALO/IOC flags pre-commit. This cuts toxic MM markout by **38.9%** vs. FIFO (`-1.049 bps` vs. `-1.717 bps`) and reduces stale sniping hits by **88.1%** (`71,633` → `8,523`) with **`0.0%` dropped USD volume**.
- **Apples-to-Apples Economic Result (`wait=all`, `MEV = 0`):** Against a `70ms` open batch (`Regime B, hl`), Dywe's `35ms` decryption latency produces **statistically zero net execution-cost penalty** (`0.922 bps` vs. `0.924 bps`, `Δ = -0.002 bps [~]`). Under parameterized MEV scenarios, Dywe becomes strictly cheaper for takers starting at **`10%` capture and `0.5 bps` impact** (`-0.026 bps D<B`), saving **`-0.155 bps`** at `25% / 1.0 bps` and **`-0.332 bps`** at `50% / 1.0 bps`.

---

## 2. Architectural Comparison (4 Isolated Matching Regimes)

Every historical observation and directional taker execution is replayed across four isolated matching engines under identical latency and reaction parameters:

| Regime | Reference Architecture | Ordering & Clearing Mechanics | Pre-Trade Privacy | Primary Trade-Off |
| :--- | :--- | :--- | :--- | :--- |
| **Regime A** | **Continuous FIFO** *(Baseline CLOB)* | Strict arrival-time (`arr_ns`) execution; no batching; no cancel priority. | None (Transparent) | Stale-quote sniping (`71,633` hits) forces `1.517 bps` equilibrium half-spread. |
| **Regime B** | **Open Priority Batch** *(`70ms` / `200ms` / `1000ms`, `hl` order)* | `New (ALO) -> Cancel -> Taker/Sniper` inside discrete batch windows. | None (Transparent) | Protects MMs with `0.0%` IOC drops, but large orders (`≥$50k`, `71.2%` of volume) remain exposed to MEV. |
| **Regime C** | **Uniform-Price FBA** *(`100ms` Auction)* | Orders accumulate over `100ms` and clear at a single uniform price; opposing flow nets internally. | None (Transparent) | `100ms` batch hold increases waiting drift (`0.895 bps`) and All-In Net cost (`0.950 bps`). |
| **Regime D** | **Dywe Protocol** *(IBTE + `50ms` Micro-Batch)* | Encrypted payloads commit at `batch_close`, decrypt at `+35ms`/`+70ms`, and clear in memory (`hl` default; `cancel-first` stress). | **IBTE on `BLS12-381`** *(10% conservative header-leak modeled)* | Statistically zero penalty at `MEV=0` (`-0.002 bps ~` vs. `B 70ms`); strictly cheaper (`D<B`) from `≥10%` capture / `0.5 bps` impact. |

> **Calibration Disclaimer:** Regimes `B (70ms)`, `B (200ms)`, and `B (1000ms)` are stylized execution models calibrated to representative batching intervals. They do not constitute protocol-level or byte-for-byte reproductions of Hyperliquid or dYdX matching engines.

---

## 3. Simulation & Mathematical Methodology

### 3.1 Separation of Empirical Data, Mechanical Parameters & Scenario Assumptions

1. **Historical Component (Measured):** Raw Binance USDⓈ-M Futures `trades` and directional `aggTrades` across 14 pair-days.
2. **Mechanical Component (Parameterized):** Feed latency (`5ms`), MM quote recomputation (`10, 25, 40ms`), sniper reaction (`2ms`), client-to-sequencer transit (`30ms`), natural sub-ms jitter (`0–999,999 ns`), batch windows (`50, 70, 100, 200, 1000ms`), and post-commit decryption RTTs (`0, 35, 70ms`).
3. **Scenario Component (Parameterized Grid):** MEV capture rate (`0%–100%`), per-order price impact (`0.0–3.0 bps`), MM loss allocation (`50%` baseline), and Cancel-Header metadata leakage (`10%` baseline, applied only to large orders inside batches containing an MM cancel — `32.5%` of large volume in `Regime D`).
4. **Derived Component (Calculated):** Competitive equilibrium half-spread (`s*net`) and All-In Net Taker Execution Cost.

### 3.2 Fixed-Point Integer Arithmetic (`i64` / `i128`) & Determinism

- Prices and quantities are scaled by `10^8` (`PRICE_SCALE = 100_000_000`, `QTY_SCALE = 100_000_000`) using widening `i128` arithmetic to prevent overflow.
- Relative moves and markouts use integer **sub-bps** (`100 sub = 1.00 bps`, truncating toward zero) and `Q = 1,000` in Phase 2 (`i128`).
- Parallel workers compute isolated per-day, per-block accumulators merged in deterministic `(symbol, day, block)` order.

### 3.3 Causal Arrival-Time Pipeline & Cryptographic Model Scope

- **Regime D Execution Timeline:** Orders arrive at `arr_ns` inside `[batch_start, batch_close)`, commit at `batch_close`, complete threshold decryption at `clear_ns = batch_close + decrypt_ms`, and clear in matching-engine memory (`--d-batch-order hl` or `cancel-first`). Public quote visibility (`sim.visible`) updates strictly at `clear_ns`.
- **Cryptographic Boundary:** Dywe's protocol design specifies Identity-Based Threshold Encryption (IBTE) over `BLS12-381` (`5-of-7` validator quorum, epoch DKG rotation). This backtest models the causal decryption delay (`decrypt_ms`) and header metadata leakage (`leak_ppm`), isolating orderbook sequencing economics from pairing CPU benchmarks. Full cryptographic specifications (DKG ceremony, quorum liveness, and malicious-validator bounds) are documented separately in *Dywe Protocol Cryptographic Specification & Core Engine Benchmarks (Document #16)*.

### 3.4 Trade-Derived Bid/Ask Midpoint Proxy (`--mid-mode bidask`)

When actual L1 `bookTicker` archives are unavailable, `--mid-mode bidask` tracks the last buyer-aggressor execution (`last_ask`) and last seller-aggressor execution (`last_bid`), setting `mid = (last_bid + last_ask) / 2` (falling back to `price` when `ask < bid`).

- **Measured Proxy Diagnostics (14 pair-days):** `84,131,187` observations, `695,039` crossed (`0.8%` where `ask < bid`), mean valid `(ask − bid)` spread = `0.708 bps`.
- *Important:* This is a trade-derived midpoint proxy, not an actual L1 `bookTicker` feed. If `bookTicker` CSVs are placed in `./data/`, the engine uses exact `(best_bid + best_ask) / 2` automatically.

### 3.5 Equilibrium Spread (`s*net`), Single-Denominator All-In Identity & Placebo Control

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

- **Joint Block Bootstrap:** `56` synchronized 3-hour blocks resampled jointly across `BTCUSDT` and `ETHUSDT` (`1,000` draws, `95% CI`).
- **Circular-Shift Placebo Control (`600s` / `3600s`):** Under a `600s` shift, **96.8%** of Regime A's baseline adverse selection disappears (`-1.717 bp` → `-0.055 bp`; **97.3%** at `3600s`), and **97.2%** of the `B 70ms (hl) − A` cancel-priority markout gap vanishes (`0.831 bp` → `0.023 bp`), confirming that the empirical signal is driven by real temporal flow-price alignment.

---

## 4. Pre-Computed Investor & Audit Reports (`results/`)

All four `v0.3.3` benchmark scenarios are committed under `results/` for immediate inspection:

| Directory | Scenario & CLI Configuration | Key Empirical Takeaway |
| :--- | :--- | :--- |
| [`results/B70_hl_vs_D50_hl_drop/`](./results/B70_hl_vs_D50_hl_drop/report.md) | **Primary Apples-to-Apples Benchmark** *(B 70ms `hl` vs D 50ms `hl`, `drop`, `wait=all`)* | `0.0%` dropped USD in both B and D. Zero-MEV delta is **`-0.002 bps (~)`**; Dywe wins (`D<B`) from **`10%` capture / `0.5 bps` impact** (`-0.026 bps`). |
| [`results/B200_hl_vs_D50_hl_drop/`](./results/B200_hl_vs_D50_hl_drop/report.md) | **200ms Window Stress** *(B 200ms `hl` vs D 50ms `hl`, `drop`, `wait=all`)* | `0.0%` dropped USD in both regimes. Zero-MEV delta is **`-0.008 bps (~)`**; Dywe wins (`D<B`) from **`10%` / `0.5 bps`** (`-0.032 bps`). |
| [`results/B1000_hl_vs_D50_hl_drop/`](./results/B1000_hl_vs_D50_hl_drop/report.md) | **1000ms Window Stress** *(B 1000ms `hl` vs D 50ms `hl`, `drop`, `wait=all`)* | Zero-MEV delta is **`+0.019 bps (~)`**; Dywe wins (`D<B`) from **`10%` / `2.0 bps`** (`-0.075 bps`) or **`25%` / `0.5 bps`** (`-0.057 bps`). |
| [`results/B70_hl_vs_D50_cancelfirst_hold_sweep/`](./results/B70_hl_vs_D50_cancelfirst_hold_sweep/report.md) | **MM Latency Sweep & `cancel-first` Stress** *(MM react `10/25/40ms`, B `hl` vs D `cancel-first`, `hold`)* | Even if D is restricted to `cancel-first` under equal-volume `hold`, zero-MEV cost is only **`+0.005 bps`** (`B<D`) and D wins from **`10%` / `0.5 bps`** (`-0.017 bps D<B`). |

Each directory contains `report.md`, interactive `ablation_report.html` (Chart.js), and full `breakeven.csv` sensitivity grids.

---

## 5. Consolidated Benchmark Summary (`v0.3.3`, `pending=drop`, `wait=all`, `mid=bidask`)

### 5.1 Pure Microstructure Mechanics (`MEV = 0`, `react = 10ms`)

| Architecture Regime | Executed MM Vol | Dropped Takers (USD%) | 1s MM Markout | Net Half-Spread `s*net` | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net | StaleSnipe Hit/Miss |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **A: Continuous FIFO** | `$167.72B` | `1,595 (0.0%)` | `-1.717 bp` | `1.517 bp` | `1.517 bp` | `-1.266 bp` | `0.677 bp` | **`0.928 bp`** | `71,633 / 97` |
| **B: Batch `70ms` (`hl`)** | `$167.90B` | `1,797 (0.0%)` | `-0.885 bp` | `0.685 bp` | `0.685 bp` | `-0.636 bp` | `0.874 bp` | **`0.924 bp`** | `6,308 / 60` |
| **B: Batch `200ms` (`hl`)** | `$168.10B` | `884 (0.0%)` | `-0.553 bp` | `0.353 bp` | `0.353 bp` | `-0.368 bp` | `0.946 bp` | **`0.930 bp`** | `1,900 / 21` |
| **B: Batch `1000ms` (`hl`)** | `$168.18B` | `511 (0.0%)` | `-0.236 bp` | `0.036 bp` | `0.036 bp` | `-0.182 bp` | `1.049 bp` | **`0.903 bp`** | `251 / 6` |
| **C: Uniform-FBA (`100ms`)** | `$145.80B` | `1,650 (0.0%)` | `-0.861 bp` | `0.661 bp` | `0.575 bp` | `-0.520 bp` | `0.895 bp` | **`0.950 bp`** | `4,223 / 43` |
| **D: Dywe IBTE `50ms` (`35ms` dec, `hl`)** | **`$168.20B`** | **`2,153 (0.0%)`** | **`-1.049 bp`** | **`0.849 bp`** | **`0.849 bp`** | **`-0.878 bp`** | **`0.951 bp`** | **`0.922 bp`** | **`8,523 / 79`** |

### 5.2 Break-Even Frontier: `All-In(D, 35ms, hl) − All-In(B, 70ms, hl)` (bps)

*Scenario parameters: `50%` MM MEV loss share, `10%` Cancel-Header leakage (on `32.5%` cancel-batch large volume), `+0.50 bps` maker rebate, `0.30 bps` hedge OpEx, `pending=drop`, `wait=all`, `mid=bidask`. Verdict tags reflect 95% joint 3-hour block bootstrap CIs (`D<B`: Dywe cheaper; `B<D`: Regime B cheaper; `~`: statistically indistinguishable). **All rows with capture > 0% are parameterized counterfactual scenarios, not measurements.***

| Large-Order MEV Capture (`≥$50k`) \ Impact | `0.000 bp` | `0.500 bp` | `1.000 bp` | `2.000 bp` | `3.000 bp` |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **0% Capture** *(Sterile Zero-MEV)* | `-0.002 ~` | `+0.009 B<D` | `+0.020 B<D` | `+0.043 B<D` | `+0.066 B<D` |
| **10% Capture** | `-0.002 ~` | **`-0.026 D<B`** | **`-0.049 D<B`** | **`-0.097 D<B`** | **`-0.144 D<B`** |
| **25% Capture** | `-0.002 ~` | **`-0.078 D<B`** | **`-0.155 D<B`** | **`-0.309 D<B`** | **`-0.462 D<B`** |
| **50% Capture** | `-0.002 ~` | **`-0.167 D<B`** | **`-0.332 D<B`** | **`-0.661 D<B`** | **`-0.991 D<B`** |
| **75% Capture** | `-0.002 ~` | **`-0.255 D<B`** | **`-0.508 D<B`** | **`-1.014 D<B`** | **`-1.520 D<B`** |
| **100% Capture** | `-0.002 ~` | **`-0.343 D<B`** | **`-0.684 D<B`** | **`-1.367 D<B`** | **`-2.050 D<B`** |

---

## 6. Built-In Verification & Unit Test Suite (14 Invariant Checks)

Executed via `cargo test --release` (all 14 pass in `<0.1s`) and automatically on pre-flight:

1. `t_causality` — Zero look-ahead bias: mutating future ticks after `T` leaves all fills before `T` bit-for-bit identical across all 4 regimes.
2. `t_identity` — `Regime D (dec=0, cancel-first)` equals `Regime B (cancel-first)` bit-for-bit at the `i128` accumulator level.
3. `t_d_post_decrypt_hl_order` — `Regime D (dec=0, hl)` equals `Regime B (hl)` bit-for-bit and eliminates artificial same-batch cancel-gap drops.
4. `t_hl_batch_order` — Verifies Hyperliquid-style `New (ALO) -> Cancel -> Taker` ordering removes dead gaps vs. `cancel-first`.
5. `t_accounting_reconciliation` — Enforces exact volume conservation and exact additive `All-In Net` component identity.
6. `t_wait_cohort_all_identity` — Validates single-denominator (`tk_n`) linear additivity under `--wait-cohort all` vs. `large`.
7. `t_fba_sniper_attribution` — Confirms late FBA snipers arriving after same-batch MM requotes are attributed to `late_fresh` (`0` false stale hits).
8. `t_placebo_breaks_link` — Confirms circular timestamp shifting destroys flow-price alignment while preserving order counts and notional.
9. `t_pending_drop_policy`, `t_bidask_mid_proxy`, `t_mev_share_sum`, `t_sniper_protection`, `t_shuffled_zero_spread`, `t_html_escape` — Validates IOC drop vs. hold semantics, `BidAskProxy` normal/crossed diagnostics, MEV share conservation, stale-snipe protection, randomized-flow zero-bias controls, and HTML sanitization.

---

## 7. Data Sources & How to Reproduce Locally

### 7.1 Historical Market Data Source

Raw daily USDⓈ-M Futures archives from **Binance Public Data Vision** (`https://data.binance.vision/data/futures/um/daily/`) for `BTCUSDT` and `ETHUSDT` (`2026-11-10` through `2026-11-16`): `trades/` (`14` CSVs), `aggTrades/` (`14` CSVs), and optional `bookTicker/` for true L1 quotes.

### 7.2 Step-by-Step Local Reproduction

**Prerequisites:** Rust toolchain (`rustc 1.82+` / `cargo`), `curl`, `shasum`, and `unzip`.

#### Step 1: Download & Verify Checksums into `./data/`

```bash
set -euo pipefail
mkdir -p data && cd data

BASE="https://data.binance.vision/data/futures/um/daily"

for sym in BTCUSDT ETHUSDT; do
  for day in 10 11 12 13 14 15 16; do
    for type in aggTrades trades; do
      file="${sym}-${type}-2026-11-${day}.zip"

      curl --fail --silent --show-error --location "${BASE}/${type}/${sym}/${file}" -o "${file}"
      curl --fail --silent --show-error --location "${BASE}/${type}/${sym}/${file}.CHECKSUM" -o "${file}.CHECKSUM"

      shasum -a 256 -c "${file}.CHECKSUM"
      unzip -oq "${file}" -d .
    done
  done
done

rm -f ./*.zip ./*.CHECKSUM
cd ..
```

#### Step 2: Run the 14-Test Invariant Suite & Regenerate All 4 Scenarios

```bash
cargo test --release

mkdir -p results

cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 70 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/B70_hl_vs_D50_hl_drop

cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 200 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/B200_hl_vs_D50_hl_drop

cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 1000 --b-batch-order hl --d-batch-order hl --pending-policy drop --wait-cohort all --mid-mode bidask --out-dir results/B1000_hl_vs_D50_hl_drop

cargo run --release -- --threads 6 --allow-reconstructed --b-window-ms 70 --b-batch-order hl --d-batch-order cancel-first --mm-react 10,25,40 --pending-policy hold --wait-cohort all --mid-mode bidask --out-dir results/B70_hl_vs_D50_cancelfirst_hold_sweep
```

### 7.3 CLI Reference

| Argument | Description |
| :--- | :--- |
| `--data-dir <PATH>` | Directory containing extracted Binance CSVs (default: `./data`). |
| `--out-dir <PATH>` | Output directory for `report.md`, `ablation_report.html`, and `breakeven.csv` (default: `.`). |
| `--window-ms <MS>` | Dywe Regime D micro-batch window duration in ms (default: `50`). |
| `--b-window-ms <MS>` | Competitor Regime B batch window duration in ms (`70`, `200`, `1000`). |
| `--b-batch-order <hl\|cancel-first>` | Intra-batch ordering for Regime B (default: `hl`). |
| `--d-batch-order <hl\|cancel-first>` | Post-decrypt in-memory ordering for Regime D (default: `hl`). |
| `--decrypt <LIST>` | Post-commit threshold decryption RTTs in ms (default: `0,35,70`). |
| `--mm-react <LIST>` | MM quote recomputation latencies in ms (default: `10`). |
| `--pending-policy <hold\|drop>` | Handling of takers arriving during an MM requote gap (default: `hold`). |
| `--wait-cohort <all\|large>` | Denominator for waiting drift in All-In Net (default: `all`). |
| `--mid-mode <bidask\|trade>` | Trade-derived midpoint proxy when `bookTicker` is absent (default: `bidask`). |
| `--rebate-sub <INT>` / `--opex-sub <INT>` | MM maker rebate & hedging OpEx in sub-bps (`100 sub = 1.00 bps`; defaults: `50`, `30`). |
| `--large-usd <INT>` | Notional threshold in USD for MEV-eligible large orders (default: `50000`). |
| `--placebo-shifts <LIST>` | Circular timestamp shifts in ms for placebo verification (default: `600000,3600000`). |

---

## 8. Repository Structure

```text
Dywe_Rust_Microstructural_Ablation_Backtest/
├── data/                                        # Local Binance CSVs (gitignored)
├── results/                                     # Pre-computed v0.3.3 benchmark reports
│   ├── B70_hl_vs_D50_hl_drop/                   # Primary apples-to-apples benchmark
│   ├── B200_hl_vs_D50_hl_drop/                  # 200ms open-batch stress benchmark
│   ├── B1000_hl_vs_D50_hl_drop/                 # 1000ms open-batch stress benchmark
│   └── B70_hl_vs_D50_cancelfirst_hold_sweep/    # MM latency sweep (10/25/40ms) & cancel-first stress
├── src/
│   └── main.rs                                  # Fixed-point simulation engine, CLI, reporting & 14 tests
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## 9. Scope, Assumptions & Limitations

- **Single-Level Top-of-Book Abstraction:** Models an endogenous competitive spread `s*net` cleared by a representative MM rather than full multi-level L2 orderbook depth replenishment.
- **Exogenous Order Flow:** Historical taker flow (`aggTrades`) is replayed causally and does not endogenously respond to spread changes.
- **Trade-Derived Midpoint Proxy:** When `bookTicker` files are absent, `--mid-mode bidask` reconstructs midpoints from aggressive trade executions (`0.8%` crossed observations) rather than passive L1 quote updates.
- **Stylized Competitor Models:** Regimes B (`70ms`, `200ms`, `1000ms`) are stylized batching models, not protocol-level reproductions of named exchanges.
- **Cryptographic Timing & MEV Grid:** `35ms`/`70ms` decryption RTTs and Phase 2 MEV capture/impact grids are parameterized counterfactual inputs rather than live Devnet benchmarks or direct measurements of sandwich attacks on Binance Futures.

---

## 10. Research Conclusion

**What this experiment establishes:**

1. **Post-decrypt priority ordering eliminates fill-rate loss:** Separating pre-commit threshold encryption from post-decrypt in-memory `ALO -> Cancel -> Taker` matching allows Dywe (`Regime D`) to achieve `0.0%` dropped USD volume under `pending=drop`, matching open priority batches (`Regime B, hl`).
2. **Negligible zero-MEV execution-cost overhead:** Under single-denominator All-In accounting (`wait=all`), Dywe's `35ms` threshold decryption adds statistically zero net cost vs. a `70ms` open batch at `MEV = 0` (`-0.002 bps ~`; `+0.005 bps` even under `cancel-first` stress).
3. **Asymmetric MEV Protection Upside:** Across all modeled batch windows (`70ms`–`1000ms`), Dywe delivers statistically significant taker cost savings (`D<B`) once `10%–25%` of large orders (`≥$50k`) experience `0.5–1.0 bps` of MEV extraction.
4. **Endogenous Adverse Selection Confirmed:** Circular-shift placebo controls remove `96.8%` of Regime A's 1s markout and `97.2%` of the cancel-priority gap.

**What remains to be validated in production:**

- Empirical p50/p95/p99 threshold decryption latency across geographically distributed validator nodes on the Dywe multi-region Devnet.
- Multi-level L2 book depth replenishment and endogenous MM spread competition under live order flow.
- Robustness verification on native L1 `bookTicker` feeds when archived.
