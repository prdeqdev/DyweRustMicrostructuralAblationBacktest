# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.4)
Window: 2026-11-10 + 7d | Pairs: SOLUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 50ms (hl), FBA 100ms | pending=drop | src=auto | mid=bidask | wait=all | sniper_clip=$0 | rebate=+0.500bp, opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 7 of 7.

**NOTE:** 7 pair-days used a trade-derived midpoint proxy (`--mid-source auto`, mode `bidask`). BidAskProxy diagnostics: 15365789 total observations, 1305 crossed (0.0% where ask < bid), mean valid (ask - bid) spread = 1.903 bps.

### SOLUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $12,629,641,501 | -0.997 | -1.494 | -2.011 | 2.011 | 1.811 | 1.811 | -0.997 | 0.519 | 1.332 | 0/0 | 137 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $12,628,398,693 | -0.612 | -0.938 | -1.400 | 1.400 | 1.200 | 1.200 | -0.612 | 0.753 | 1.341 | 0/0 | 200 (0.0%) | 1649862 |
| C: Uniform-Price FBA 100ms | react 10ms | $11,440,088,430 | -0.595 | -0.900 | -1.384 | 1.384 | 1.184 | 1.072 | -0.539 | 0.824 | 1.357 | 0/0 | 193 (0.0%) | 1569058 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $12,626,871,546 | -0.686 | -1.044 | -1.524 | 1.524 | 1.324 | 1.324 | -0.686 | 0.699 | 1.338 | 0/0 | 311 (0.0%) | 1719860 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $12,626,871,546 | -0.881 | -1.097 | -1.533 | 1.533 | 1.333 | 1.333 | -0.881 | 0.894 | 1.346 | 0/0 | 311 (0.0%) | 1719860 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $12,626,871,546 | -0.988 | -1.147 | -1.533 | 1.533 | 1.333 | 1.333 | -0.989 | 1.002 | 1.346 | 0/0 | 311 (0.0%) | 1719860 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $12,629,641,501 | -0.997 | -1.494 | -2.011 | 2.011 | 1.811 | 1.811 | -0.997 | 0.519 | 1.332 | 0/0 | 137 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $12,628,398,693 | -0.612 | -0.938 | -1.400 | 1.400 | 1.200 | 1.200 | -0.612 | 0.753 | 1.341 | 0/0 | 200 (0.0%) | 1649862 |
| C: Uniform-Price FBA 100ms | react 10ms | $11,440,088,430 | -0.595 | -0.900 | -1.384 | 1.384 | 1.184 | 1.072 | -0.539 | 0.824 | 1.357 | 0/0 | 193 (0.0%) | 1569058 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $12,626,871,546 | -0.686 | -1.044 | -1.524 | 1.524 | 1.324 | 1.324 | -0.686 | 0.699 | 1.338 | 0/0 | 311 (0.0%) | 1719860 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $12,626,871,546 | -0.881 | -1.097 | -1.533 | 1.533 | 1.333 | 1.333 | -0.881 | 0.894 | 1.346 | 0/0 | 311 (0.0%) | 1719860 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $12,626,871,546 | -0.988 | -1.147 | -1.533 | 1.533 | 1.333 | 1.333 | -0.989 | 1.002 | 1.346 | 0/0 | 311 (0.0%) | 1719860 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | Large Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $12,629,641,501 | $0 | $0 | $12,629,641,501 | $0 | -2.011 | n/a | n/a | 42.4% | 0.0% |
| B: Batch 70ms (hl) | react 10ms | $12,628,398,693 | $0 | $0 | $12,628,398,693 | $0 | -1.400 | n/a | n/a | 42.4% | 41.5% |
| C: Uniform-Price FBA 100ms | react 10ms | $11,440,088,430 | $0 | $0 | $11,440,088,430 | $0 | -1.384 | n/a | n/a | 42.4% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $12,626,871,546 | $0 | $0 | $12,626,871,546 | $0 | -1.524 | n/a | n/a | 42.4% | 35.1% |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $12,626,871,546 | $0 | $0 | $12,626,871,546 | $0 | -1.533 | n/a | n/a | 42.4% | 35.1% |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $12,626,871,546 | $0 | $0 | $12,626,871,546 | $0 | -1.533 | n/a | n/a | 42.4% | 35.1% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). In Regime C (Uniform-Price FBA), taker and sniper orders net internally into a single imbalance before clearing against the MM. `Cancel-Batch Large %` shows the share of large orders arriving inside a batch that also contains an MM cancel.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 600s | $12,631,505,842 | 0.000 | -0.001 | -0.006 | 0.006 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 0/0 | 32 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 600s | $12,631,749,394 | -0.002 | -0.002 | -0.008 | 0.008 | <0 | 0.000 | -0.002 | 0.001 | 0.000 | 0/0 | 33 (0.0%) | 1739635 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 600s | $11,442,210,029 | -0.002 | -0.003 | -0.007 | 0.007 | <0 | 0.000 | -0.001 | 0.001 | 0.000 | 0/0 | 52 (0.0%) | 1653245 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 600s | $12,631,719,550 | 0.000 | -0.002 | -0.006 | 0.006 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 0/0 | 49 (0.0%) | 1809599 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 600s | $12,631,719,550 | -0.001 | -0.002 | -0.006 | 0.006 | <0 | 0.000 | -0.001 | 0.001 | 0.000 | 0/0 | 49 (0.0%) | 1809599 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 600s | $12,631,719,550 | -0.001 | 0.000 | -0.007 | 0.007 | <0 | 0.000 | -0.001 | 0.001 | 0.000 | 0/0 | 49 (0.0%) | 1809599 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 3600s | $12,631,657,384 | -0.002 | -0.004 | -0.008 | 0.008 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 0/0 | 38 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 3600s | $12,631,636,098 | -0.004 | -0.006 | -0.010 | 0.010 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 0/0 | 37 (0.0%) | 1741684 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 3600s | $11,443,708,710 | -0.006 | -0.008 | -0.012 | 0.012 | <0 | 0.000 | -0.006 | 0.001 | -0.004 | 0/0 | 38 (0.0%) | 1655157 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 3600s | $12,631,666,642 | -0.003 | -0.006 | -0.010 | 0.010 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 0/0 | 35 (0.0%) | 1810797 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 3600s | $12,631,666,642 | -0.004 | -0.007 | -0.011 | 0.011 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 0/0 | 35 (0.0%) | 1810797 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 3600s | $12,631,666,642 | -0.006 | -0.006 | -0.012 | 0.012 | <0 | 0.000 | -0.006 | 0.002 | -0.003 | 0/0 | 35 (0.0%) | 1810797 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -2.011 bp, B -1.400 bp, cancel-priority gap (B minus A) 0.611 bp.
- Placebo shift 600 s: A -0.006 bp, B -0.008 bp, gap -0.001 bp = -0.3% of the real gap.
- Placebo shift 3600 s: A -0.008 bp, B -0.010 bp, gap -0.001 bp = -0.2% of the real gap.
- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.611 bp = 30.3% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -2.011 bps, 95% CI [-2.168, -1.858]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × leak`), not measurements. At `capture = 0%`, MEV extraction is strictly zero regardless of `leak_ppm`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, hl) | react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ |
| 10% | -0.002 ~ | -0.016 D<B | -0.030 D<B | -0.057 D<B | -0.085 D<B |
| 25% | -0.002 ~ | -0.048 D<B | -0.094 D<B | -0.185 D<B | -0.276 D<B |
| 50% | -0.002 ~ | -0.101 D<B | -0.200 D<B | -0.397 D<B | -0.594 D<B |
| 75% | -0.002 ~ | -0.154 D<B | -0.306 D<B | -0.609 D<B | -0.913 D<B |
| 100% | -0.002 ~ | -0.207 D<B | -0.412 D<B | -0.821 D<B | -1.231 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, hl) | react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D |
| 10% | 0.005 B<D | -0.008 D<B | -0.022 D<B | -0.049 D<B | -0.077 D<B |
| 25% | 0.005 B<D | -0.040 D<B | -0.085 D<B | -0.176 D<B | -0.268 D<B |
| 50% | 0.005 B<D | -0.093 D<B | -0.191 D<B | -0.389 D<B | -0.586 D<B |
| 75% | 0.005 B<D | -0.146 D<B | -0.298 D<B | -0.601 D<B | -0.904 D<B |
| 100% | 0.005 B<D | -0.199 D<B | -0.404 D<B | -0.813 D<B | -1.223 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, hl) | react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.005 ~ | 0.005 ~ | 0.005 ~ | 0.005 ~ | 0.005 ~ |
| 10% | 0.005 ~ | -0.008 D<B | -0.021 D<B | -0.049 D<B | -0.076 D<B |
| 25% | 0.005 ~ | -0.039 D<B | -0.085 D<B | -0.176 D<B | -0.267 D<B |
| 50% | 0.005 ~ | -0.092 D<B | -0.191 D<B | -0.388 D<B | -0.586 D<B |
| 75% | 0.005 ~ | -0.146 D<B | -0.297 D<B | -0.601 D<B | -0.904 D<B |
| 100% | 0.005 ~ | -0.199 D<B | -0.403 D<B | -0.813 D<B | -1.222 D<B |

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
