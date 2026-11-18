# Dywe Microstructural Ablation Backtest (`dywe-ablation-sim`)

**Deterministic, 100% Fixed-Point (`i64`/`i128`) Orderbook & Execution Replay Engine** benchmarking the **Dywe Exchange** market design against industry-standard decentralized orderbook architectures on **84.13 million real historical ticks** from Binance USDⓈ-M Futures (`BTCUSDT` & `ETHUSDT`, Nov 10–16, 2026).

---

## 1. Purpose & Research Question

Institutional market makers and quantitative funds evaluating decentralized perpetual exchanges ask a specific architectural question:
> *"Given that Cancel-Header Priority is already deployed in production (e.g., Hyperliquid) and Discrete Frequent Batch Auctions (DFBA) are proposed by HFT researchers (e.g., Jump Crypto), what measurable economic edge does **Dywe's combination of Threshold Intent Encryption (IBTE BLS12-381) + Cancel-Header Priority + 50ms Micro-Batching** add over existing models?"*

This repository provides a **reproducible, open-source ablation study** answering that question on identical historical tick streams without curve-fitting or floating-point non-determinism.

---

## 2. Architectures Benchmarked (4 Isolated Regimes)

Every historical tick, taker order, and L2 orderbook depth snapshot is replayed across four isolated matching regimes:

| Regime | Architecture Model | Mechanism & Ordering Rules | Vulnerability / Trade-off |
| :--- | :--- | :--- | :--- |
| **Regime A** | **Transparent FIFO** *(Baseline CLOB)* | Strict arrival-time ordering; transparent mempool; no cancel priority. | Resting MM quotes are sniped by latency arbitrageurs; large takers (`>$50k`) suffer sandwich attacks. |
| **Regime B** | **FIFO + Cancel Priority** *(Hyperliquid Model)* | `CANCEL` orders are sorted to the head of each micro-batch, protecting MMs from stale-quote sniping. | Mempool remains transparent: MEV searchers still observe large directional orders (`>$50k`) and front-run/sandwich takers. |
| **Regime C** | **Discrete Batch Auction** *(DFBA 100ms)* | All orders accumulate over `100 ms` and clear at a single uniform auction price. | Eliminates individual quote sniping (`0` snipers), but `100 ms` inventory lock forces MMs to widen spreads and increases taker waiting drift. |
| **Regime D** | **Dywe Architecture** *(IBTE BLS12-381 + Cancel-Prio)* | `CANCEL` orders execute at batch head (`0` snipers) **AND** all incoming orders are threshold-encrypted (5-of-7 BLS12-381) until batch root commit (`0` sandwiches). | Combines sub-second continuous-feel execution (`50 ms` micro-batch) with full pre-trade privacy for both MMs and large takers/copy-vaults. |

---

## 3. Proprietary Dywe Mechanisms vs. External Market Inputs

### Dywe Proprietary Components Evaluated
1. **100% Fixed-Point Matching Core (`i64` / `i128`, `1e8` scale):** Zero `f32`/`f64` instructions in price, quantity, markout, or PnL arithmetic—guaranteeing bit-identical determinism across validator nodes.
2. **Cancel-Header Priority Queue:** Sorts market-maker `MmCancel` messages ahead of aggressive takers within the `50 ms` micro-batch window (`30 ms` simulated client RTT + `5–15 ms` MM reaction jitter).
3. **IBTE (Identity-Based Threshold Encryption) Pre-Trade Privacy:** Prevents MEV searchers from inspecting order size or direction prior to batch commitment, eliminating content-based front-running and sandwich attacks on orders `> $50,000`.
4. **LULD Circuit Breaker (`±2.50%` intra-batch guard):** Monitors intra-batch price excursions across all 14 pair-days (`0` false halts triggered during normal high-volatility trading).

*Note on Cryptographic Separation:* Inside the 84.13M-tick replay loop, batch payload decryption uses constant-time integrity mixing so pure CLOB matching latency (`1 µs` p50 / `2 µs` p99) can be measured independently. Full threshold BLS12-381 (`5-of-7`) pairing decryption adds `0.75 ms` CPU overhead (`~35 ms` network RTT), benchmarked separately in *Doc #16 (Core Engine Benchmarks & PoC Report)*.

### External Historical Datasets (Binance Public Data Vision)
* **`trades` (84,131,219 L1 ticks):** Individual trade executions used to reconstruct millisecond L1 Mid-Price (`$0.10` tick / `$0.05` half-tick for `BTCUSDT`; `$0.01` tick / `$0.005` half-tick for `ETHUSDT`).
* **`aggTrades` (26,295,897 taker streams):** Real directional market orders hitting the book (with automatic deterministic synthesis fallback from `trades` if a daily file is omitted, e.g., `ETHUSDT-aggTrades-2026-11-15.csv`).
* **`bookDepth` (40,320 L2 snapshots):** Historical `±1%..±5%` orderbook liquidity depth snapshots used to compute endogenous Kyle's Lambda price impact elasticity (`sub-bps per $100k swept`).

---

## 4. Consolidated 7-Day Empirical Results (`2026-11-10` to `2026-11-16`)

**Institutional MM Calibration:** `$10,000,000` allocated inventory turning over `15x` daily (`$150,000,000/day`), `-0.50 bps` maker rebate (`+0.50 bps` revenue), and `0.30 bps` VIP hedging/rebalancing cost.

| Architecture Regime | Markout 0ms | Markout 100ms | Markout 1000ms | Markout 5000ms | LVR Loss | Eq. Spread | Net MM ROE | Taker Slippage | MEV Saved vs B | Snipers / Sandwiches |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Regime A (FIFO)** | `-0.45 bp` | `-0.75 bp` | `-0.81 bp` | `-0.87 bp` | `2.36 bp` | `4.82 bp` | `-33.39%` | `1.96 bp` | `0.00 bp` | `80,502` / `722,448` |
| **Regime B (Hyperliquid)** | `+0.48 bp` | `+0.19 bp` | `+0.12 bp` | `+0.07 bp` | `1.59 bp` | `3.28 bp` | `+17.52%` | `1.96 bp` | `0.00 bp` | `0` / `722,448` |
| **Regime C (DFBA 100ms)** | `+0.08 bp` | `-0.04 bp` | `-0.13 bp` | `-0.18 bp` | `2.05 bp` | `4.20 bp` | `+3.83%` | `3.04 bp` | `-1.08 bp` | `0` / `722,448` |
| **Regime D (Dywe IBTE)** | **`+0.83 bp`** | **`+0.53 bp`** | **`+0.47 bp`** | **`+0.41 bp`** | **`1.34 bp`** | **`2.78 bp`** | **`+36.68%`** | **`1.13 bp`** | **`+0.83 bp`** | **`0` / `0`** |

### Key Takeaways for Investors & Liquidity Providers
* **+19.16 pp Net MM ROE Edge over Hyperliquid Model:** Eliminating residual toxic sweeps and adverse selection lifts annualized MM ROE from `+17.52%` (Regime B) to **`+36.68%`** (Regime D), and **`+70.07 pp`** above standard FIFO (`-33.39%`).
* **-43.2% LVR Reduction:** Adverse selection loss drops from `2.36 bps` (FIFO) and `1.59 bps` (Cancel-Priority) down to **`1.34 bps`**.
* **Tightest Equilibrium Spread (`2.78 bps`):** Lower toxicity allows MMs to quote `15.2%` tighter spreads than Regime B (`3.28 bps`) and `42.3%` tighter than transparent FIFO (`4.82 bps`).
* **+0.83 bps Direct Taker & Copy-Vault Savings:** Encrypting pre-trade order intent prevents `722,448` sandwich attacks on orders `> $50k`, reducing average taker execution slippage from `1.96 bps` to **`1.13 bps`**.

---

## 5. How to Reproduce Locally

### Prerequisites
* Rust toolchain (`rustc 1.82+` / `cargo`)
* `curl` and `unzip` (standard on macOS / Linux)

### Step 1: Download the 7-Day Binance Futures Dataset into `./data/`
Run the following command from the repository root to fetch the exact public archives (`BTCUSDT` & `ETHUSDT` for `2026-11-10` through `2026-11-16`):

```bash
cd data && for sym in BTCUSDT ETHUSDT; do for d in 10 11 12 13 14 15 16; do for t in aggTrades trades bookDepth; do curl -s -fLO "[https://data.binance.vision/data/futures/um/daily/$](https://data.binance.vision/data/futures/um/daily/$){t}/${sym}/${sym}-${t}-2026-11-${d}.zip" && echo "OK: ${sym}-${t}-2026-11-${d}"; done; done; done && unzip -oq "*.zip" && rm -f *.zip && cd ..

### Step 2: Run the Release Benchmark (`cargo run --release`)
Compile and execute the deterministic fixed-point simulation in release mode:

```bash
cargo run --release

### Step 3: Inspect the Interactive Visual Report
Open the generated HTML report in your browser to inspect the Markout Decay curves (0ms → 5000ms), Equilibrium Spread vs. Net MM ROE charts, and full regime metrics:

```bash
open ablation_report.html
