# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.4)
Window: 2026-11-10 + 7d | Pairs: SOLUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 50ms (hl), FBA 100ms | pending=drop | src=auto | mid=bidask | wait=all | sniper_clip=$25,000 | rebate=+0.500bp, opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 7 of 7.

**NOTE:** 7 pair-days used a trade-derived midpoint proxy (`--mid-source auto`, mode `bidask`). BidAskProxy diagnostics: 15365789 total observations, 1305 crossed (0.0% where ask < bid), mean valid (ask - bid) spread = 1.903 bps.

### SOLUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $15,093,516,501 | -1.241 | -1.684 | -2.159 | 2.159 | 1.959 | 1.959 | -0.997 | 0.519 | 1.480 | 90377/21 | 137 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $15,097,498,693 | -0.585 | -0.876 | -1.305 | 1.305 | 1.105 | 1.105 | -0.612 | 0.753 | 1.246 | 9654/53 | 200 (0.0%) | 1650369 |
| C: Uniform-Price FBA 100ms | react 10ms | $13,411,898,880 | -0.576 | -0.857 | -1.315 | 1.315 | 1.115 | 0.988 | -0.539 | 0.824 | 1.273 | 6847/60 | 193 (0.0%) | 1569465 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $15,098,646,546 | -0.666 | -0.986 | -1.429 | 1.429 | 1.229 | 1.229 | -0.686 | 0.699 | 1.243 | 14020/119 | 311 (0.0%) | 1720519 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $15,128,896,546 | -0.836 | -1.031 | -1.435 | 1.435 | 1.235 | 1.235 | -0.881 | 0.894 | 1.248 | 13318/115 | 311 (0.0%) | 1720921 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $15,151,321,546 | -0.930 | -1.077 | -1.434 | 1.434 | 1.234 | 1.234 | -0.989 | 1.002 | 1.248 | 12813/108 | 311 (0.0%) | 1721001 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $15,093,516,501 | -1.241 | -1.684 | -2.159 | 2.159 | 1.959 | 1.959 | -0.997 | 0.519 | 1.480 | 90377/21 | 137 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $15,097,498,693 | -0.585 | -0.876 | -1.305 | 1.305 | 1.105 | 1.105 | -0.612 | 0.753 | 1.246 | 9654/53 | 200 (0.0%) | 1650369 |
| C: Uniform-Price FBA 100ms | react 10ms | $13,411,898,880 | -0.576 | -0.857 | -1.315 | 1.315 | 1.115 | 0.988 | -0.539 | 0.824 | 1.273 | 6847/60 | 193 (0.0%) | 1569465 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $15,098,646,546 | -0.666 | -0.986 | -1.429 | 1.429 | 1.229 | 1.229 | -0.686 | 0.699 | 1.243 | 14020/119 | 311 (0.0%) | 1720519 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $15,128,896,546 | -0.836 | -1.031 | -1.435 | 1.435 | 1.235 | 1.235 | -0.881 | 0.894 | 1.248 | 13318/115 | 311 (0.0%) | 1720921 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $15,151,321,546 | -0.930 | -1.077 | -1.434 | 1.434 | 1.234 | 1.234 | -0.989 | 1.002 | 1.248 | 12813/108 | 311 (0.0%) | 1721001 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | Large Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $12,629,641,501 | $2,259,425,000 | $204,450,000 | $15,093,516,501 | $0 | -2.011 | -3.040 | -1.539 | 42.4% | 0.0% |
| B: Batch 70ms (hl) | react 10ms | $12,628,398,693 | $241,350,000 | $2,227,750,000 | $15,097,498,693 | $0 | -1.400 | -2.978 | -0.585 | 42.4% | 41.5% |
| C: Uniform-Price FBA 100ms | react 10ms | $13,411,898,880 | $0 | $0 | $13,411,898,880 | $0 | -1.315 | n/a | n/a | 42.4% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $12,626,871,546 | $350,500,000 | $2,121,275,000 | $15,098,646,546 | $0 | -1.524 | -3.015 | -0.600 | 42.4% | 35.1% |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $12,626,871,546 | $332,950,000 | $2,169,075,000 | $15,128,896,546 | $0 | -1.533 | -3.022 | -0.620 | 42.4% | 35.1% |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $12,626,871,546 | $320,325,000 | $2,204,125,000 | $15,151,321,546 | $0 | -1.533 | -3.040 | -0.636 | 42.4% | 35.1% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). In Regime C (Uniform-Price FBA), taker and sniper orders net internally into a single imbalance before clearing against the MM. `Cancel-Batch Large %` shows the share of large orders arriving inside a batch that also contains an MM cancel.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 600s | $15,095,380,842 | -0.406 | -0.434 | -0.481 | 0.481 | 0.281 | 0.281 | 0.000 | 0.000 | 0.281 | 90377/21 | 32 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 600s | $15,100,849,394 | -0.075 | -0.094 | -0.140 | 0.140 | <0 | 0.000 | -0.002 | 0.001 | 0.000 | 9654/53 | 33 (0.0%) | 1748875 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 600s | $13,733,267,825 | -0.069 | -0.090 | -0.137 | 0.137 | <0 | 0.000 | -0.001 | 0.001 | 0.000 | 6847/60 | 52 (0.0%) | 1659258 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 600s | $15,103,494,550 | -0.093 | -0.114 | -0.160 | 0.160 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 14020/119 | 49 (0.0%) | 1823629 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 600s | $15,133,744,550 | -0.101 | -0.117 | -0.160 | 0.160 | <0 | 0.000 | -0.001 | 0.001 | 0.000 | 13318/115 | 49 (0.0%) | 1826252 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 600s | $15,156,169,550 | -0.107 | -0.121 | -0.162 | 0.162 | <0 | 0.000 | -0.001 | 0.001 | 0.000 | 12813/108 | 49 (0.0%) | 1828502 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 3600s | $15,095,532,384 | -0.408 | -0.437 | -0.483 | 0.483 | 0.283 | 0.283 | -0.002 | 0.000 | 0.281 | 90377/21 | 38 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 3600s | $15,100,736,098 | -0.077 | -0.097 | -0.142 | 0.142 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 9654/53 | 37 (0.0%) | 1751126 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 3600s | $13,748,305,396 | -0.073 | -0.093 | -0.141 | 0.141 | <0 | 0.000 | -0.006 | 0.001 | -0.004 | 6847/60 | 38 (0.0%) | 1661308 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 3600s | $15,103,441,642 | -0.096 | -0.117 | -0.163 | 0.163 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 14020/119 | 35 (0.0%) | 1825167 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 3600s | $15,133,691,642 | -0.104 | -0.121 | -0.165 | 0.165 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 13318/115 | 35 (0.0%) | 1827823 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 3600s | $15,156,116,642 | -0.111 | -0.126 | -0.167 | 0.167 | <0 | 0.000 | -0.006 | 0.002 | -0.003 | 12813/108 | 35 (0.0%) | 1830170 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -2.159 bp, B -1.305 bp, cancel-priority gap (B minus A) 0.853 bp.
- Placebo shift 600 s: A -0.481 bp, B -0.140 bp, gap 0.340 bp = 39.8% of the real gap.
- Placebo shift 3600 s: A -0.483 bp, B -0.142 bp, gap 0.340 bp = 39.8% of the real gap.
- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.853 bp = 39.5% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -2.159 bps, 95% CI [-2.297, -2.015]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × leak`), not measurements. At `capture = 0%`, MEV extraction is strictly zero regardless of `leak_ppm`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, hl) | react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | -0.003 ~ | -0.003 ~ | -0.003 ~ | -0.003 ~ | -0.003 ~ |
| 10% | -0.003 ~ | -0.015 D<B | -0.028 D<B | -0.053 D<B | -0.078 D<B |
| 25% | -0.003 ~ | -0.044 D<B | -0.086 D<B | -0.170 D<B | -0.254 D<B |
| 50% | -0.003 ~ | -0.093 D<B | -0.184 D<B | -0.365 D<B | -0.546 D<B |
| 75% | -0.003 ~ | -0.142 D<B | -0.281 D<B | -0.560 D<B | -0.838 D<B |
| 100% | -0.003 ~ | -0.191 D<B | -0.379 D<B | -0.755 D<B | -1.131 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, hl) | react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.002 ~ | 0.002 ~ | 0.002 ~ | 0.002 ~ | 0.002 ~ |
| 10% | 0.002 ~ | -0.010 D<B | -0.022 D<B | -0.048 D<B | -0.073 D<B |
| 25% | 0.002 ~ | -0.039 D<B | -0.081 D<B | -0.165 D<B | -0.248 D<B |
| 50% | 0.002 ~ | -0.088 D<B | -0.178 D<B | -0.359 D<B | -0.541 D<B |
| 75% | 0.002 ~ | -0.136 D<B | -0.276 D<B | -0.554 D<B | -0.833 D<B |
| 100% | 0.002 ~ | -0.185 D<B | -0.373 D<B | -0.749 D<B | -1.125 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, hl) | react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.002 ~ | 0.002 ~ | 0.002 ~ | 0.002 ~ | 0.002 ~ |
| 10% | 0.002 ~ | -0.010 D<B | -0.023 D<B | -0.048 D<B | -0.073 D<B |
| 25% | 0.002 ~ | -0.039 D<B | -0.081 D<B | -0.165 D<B | -0.249 D<B |
| 50% | 0.002 ~ | -0.088 D<B | -0.178 D<B | -0.360 D<B | -0.541 D<B |
| 75% | 0.002 ~ | -0.137 D<B | -0.276 D<B | -0.555 D<B | -0.833 D<B |
| 100% | 0.002 ~ | -0.185 D<B | -0.373 D<B | -0.749 D<B | -1.125 D<B |

## External Venue Documentation & Parameterized Assumptions (NOT simulation inputs)
- **Hyperliquid: intra-batch priority order:** Official docs (hyperliquid-l1/order-book) state that within each consensus batch, non-GTC/IOC actions (i.e. post-only / ALO) are processed first, followed by cancels, followed by GTC/IOC orders. Thus new MM post-only quotes land before cancels and takers without a dead gap. Model this with `--b-batch-order hl`. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-l1/order-book_
- **Hyperliquid: block interval & buckets:** Official docs (for-developers/api/priority-fees) state that IOC orders are sorted in 70 ms unix-time buckets, typically corresponding to one block (with 1-2 consensus batches per block, meaning effective batch windows may be shorter than 70 ms). End-to-end co-located latency is cited around ~0.2 s median in third-party mirrors. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/priority-fees ; https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview_
- **dYdX Chain (v4): block time:** The official onboarding FAQ states an average block time of about 1 second, and that short-term order cancels are best effort until the block is final. Use `--b-window-ms 1000` to evaluate a 1-second batching cadence. _Source: https://docs.dydx.xyz/concepts/onboarding-faqs_
- **Not documented (stay parameterized assumptions):** No venue documentation specifies MM reaction time, market-data feed latency, client-to-sequencer network transit, or latency-arbitrageur reaction time. Defaults (feed 5 ms, MM reaction 10 ms, sniper reaction 2 ms, network 30 ms) are parameterized assumptions; sweep `--mm-react` rather than trusting a single point. _Source: n/a_

## Dywe Protocol Design Parameters & Threat Model
- **Dywe micro-batch window & threshold decryption RTT:** The 50 ms micro-batch window and 35 ms / 70 ms threshold decryption RTTs are Dywe protocol design parameters, not live multi-region Devnet measurements.
- **Unified post-decrypt threat model:** All order messages (New ALO, Cancel, Taker) are threshold-encrypted client-side (BLS12-381) until batch_close. Threshold decryption completes in matching-engine memory at clear_ns = batch_close + decrypt_ms before the first fill executes, enabling deterministic New (ALO) -> Cancel -> Taker ordering (--d-batch-order hl) without leaking ALO/IOC flags pre-commit. The leak_ppm grid models potential residual side-channel/timing leakage on batches with MM cancels (0% under full cryptographic payload/metadata padding; 10%-50% stress sensitivity).
- **Stylized model disclaimer:** Regimes B (70 ms / 200 ms / 1000 ms) are stylized batching models, not byte-for-byte reproductions of Hyperliquid or dYdX matching engines.

## Methodological Boundaries & Scope
- Regime D timeline: all order fields (New ALO, Cancel, Taker) are threshold-encrypted pre-commit, commit at `batch_close`, complete threshold decryption at `clear_ns = batch_close + decrypt_ms`, and clear in matching-engine memory using `--d-batch-order hl`.
- Regime B uses `--b-batch-order hl` (`hl` = New/post-only -> Cancel -> Taker/Sniper; `cancel-first` = Cancel -> New/Taker/Sniper).
- Single-denominator All-In Net: `--wait-cohort all` (`all` uses total executed taker volume `tk_n` across all 4 components; `large` isolates waiting drift of large orders >= $50,000).
- Simulated latency-arb probe: StaleSnipe hits are generated by a parameterized probe (`--sniper-clip`, default $25,000); set `--sniper-clip 0` to isolate historical taker flow.
- Under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net; inspect `Dropped Takers (USD%)` and selection-bias warnings before comparing costs.
- 95% CI reflects drift variance across multi-hour blocks under FIXED scenario assumptions; parameter uncertainty (capture, impact, leak, reaction times, windows) is not covered.
- Exogenous taker flow (invariant to spread width); one representative MM clears the full exchange flow; absolute `s*net` levels are model-derived and not directly comparable to real Binance tick spreads.
