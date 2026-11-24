# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.5)
Window: 2024-03-28 + 3d | Pairs: BTCUSDT,ETHUSDT,SOLUSDT | Bootstrap Block: 3h (24 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 70ms (hl, exp=all), FBA 100ms | requote_thr=1.500bp, sniper_thr=2.000bp | pending=drop | src=book | mid=bidask | wait=all | sniper_clip=$25,000 | rebate=0.500bp (paid to MM), opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 9 of 9.

**TAKER FLOW DIAGNOSTICS:** Historical taker flow diagnostics (pure exchange volume $62,931,377,194, threshold >=$50,000): unmerged raw rows = 8615487 (244685 large rows, 47.9% of USD volume); 1-ms directional bursts (`merge_taker`) = 4244321 (237805 large bursts, 68.2% of USD volume).

**BOOKTICKER DIAGNOSTICS:** Native L1 bookTicker diagnostics (9 files): 74023564 raw quote rows -> 3269988 distinct ms-level mid changes (4312026 same-ms collapsed, 66441550 unchanged-mid skipped); inter-update interval median = 3 ms, mean = 237.79 ms.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $36,790,884,363 | $36,630,409,363 | -0.871 | -1.041 | -1.115 | 1.115 | 0.915 | 0.915 | -0.867 | 0.533 | 0.581 | 4246/7 | 49 (0.003%) | 0 |
| B: Batch 70ms (hl), react 10ms | $36,820,561,211 | $36,625,786,211 | -0.501 | -0.613 | -0.660 | 0.660 | 0.460 | 0.460 | -0.501 | 0.619 | 0.578 | 421/2 | 57 (0.016%) | 1158839 |
| C: Uniform-Price FBA 100ms, react 10ms | $33,995,816,314 | $36,628,216,864 | -0.453 | -0.560 | -0.612 | 0.612 | 0.412 | 0.380 | -0.418 | 0.635 | 0.597 | 266/3 | 73 (0.009%) | 1076005 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms | $36,820,561,211 | $36,625,786,211 | -0.501 | -0.613 | -0.660 | 0.660 | 0.460 | 0.460 | -0.501 | 0.619 | 0.578 | 421/2 | 57 (0.016%) | 1158839 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms | $36,848,311,211 | $36,625,786,211 | -0.559 | -0.633 | -0.666 | 0.666 | 0.466 | 0.466 | -0.559 | 0.676 | 0.584 | 419/2 | 57 (0.016%) | 1158863 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms | $36,865,711,211 | $36,625,786,211 | -0.592 | -0.644 | -0.669 | 0.669 | 0.469 | 0.469 | -0.592 | 0.709 | 0.587 | 413/2 | 57 (0.016%) | 1158915 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $17,821,358,950 | $17,568,333,950 | -1.129 | -1.321 | -1.395 | 1.395 | 1.195 | 1.195 | -1.118 | 0.709 | 0.786 | 7063/17 | 59 (0.005%) | 0 |
| B: Batch 70ms (hl), react 10ms | $17,859,555,936 | $17,561,330,936 | -0.529 | -0.663 | -0.710 | 0.710 | 0.510 | 0.510 | -0.530 | 0.788 | 0.768 | 749/6 | 141 (0.045%) | 975655 |
| C: Uniform-Price FBA 100ms, react 10ms | $16,646,057,427 | $17,567,057,419 | -0.479 | -0.603 | -0.654 | 0.654 | 0.454 | 0.422 | -0.446 | 0.811 | 0.787 | 446/4 | 40 (0.012%) | 915483 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms | $17,859,555,936 | $17,561,330,936 | -0.529 | -0.663 | -0.710 | 0.710 | 0.510 | 0.510 | -0.530 | 0.788 | 0.768 | 749/6 | 141 (0.045%) | 975655 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms | $17,893,505,936 | $17,561,330,936 | -0.595 | -0.667 | -0.715 | 0.715 | 0.515 | 0.515 | -0.597 | 0.854 | 0.773 | 745/5 | 141 (0.045%) | 975683 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms | $17,915,730,936 | $17,561,330,936 | -0.642 | -0.679 | -0.713 | 0.713 | 0.513 | 0.513 | -0.643 | 0.901 | 0.771 | 730/6 | 141 (0.045%) | 975747 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### SOLUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $9,501,962,539 | $8,729,537,539 | -1.829 | -1.961 | -1.923 | 1.923 | 1.723 | 1.723 | -1.814 | 0.846 | 0.755 | 23798/24 | 108 (0.007%) | 0 |
| B: Batch 70ms (hl), react 10ms | $9,577,382,807 | $8,727,582,807 | -0.538 | -0.693 | -0.615 | 0.615 | 0.415 | 0.415 | -0.558 | 0.872 | 0.729 | 2187/18 | 124 (0.030%) | 833240 |
| C: Uniform-Price FBA 100ms, react 10ms | $8,819,757,763 | $8,727,426,115 | -0.550 | -0.670 | -0.604 | 0.604 | 0.404 | 0.368 | -0.525 | 0.907 | 0.751 | 1468/15 | 156 (0.031%) | 777741 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms | $9,577,382,807 | $8,727,582,807 | -0.538 | -0.693 | -0.615 | 0.615 | 0.415 | 0.415 | -0.558 | 0.872 | 0.729 | 2187/18 | 124 (0.030%) | 833240 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms | $9,644,257,807 | $8,727,582,807 | -0.597 | -0.648 | -0.627 | 0.627 | 0.427 | 0.427 | -0.618 | 0.932 | 0.741 | 2175/18 | 124 (0.030%) | 833535 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms | $9,690,757,807 | $8,727,582,807 | -0.609 | -0.722 | -0.633 | 0.633 | 0.433 | 0.433 | -0.629 | 0.942 | 0.747 | 2147/18 | 124 (0.030%) | 833884 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $64,114,205,852 | $62,928,280,852 | -1.085 | -1.255 | -1.312 | 1.312 | 1.112 | 1.112 | -1.068 | 0.626 | 0.670 | 35107/48 | 216 (0.004%) | 0 |
| B: Batch 70ms (hl), react 10ms | $64,257,499,954 | $62,914,699,954 | -0.515 | -0.639 | -0.667 | 0.667 | 0.467 | 0.467 | -0.517 | 0.701 | 0.651 | 3357/26 | 322 (0.026%) | 2967734 |
| C: Uniform-Price FBA 100ms, react 10ms | $59,461,631,504 | $62,922,700,398 | -0.475 | -0.588 | -0.622 | 0.622 | 0.422 | 0.390 | -0.441 | 0.722 | 0.671 | 2180/22 | 269 (0.013%) | 2769229 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms | $64,257,499,954 | $62,914,699,954 | -0.515 | -0.639 | -0.667 | 0.667 | 0.467 | 0.467 | -0.517 | 0.701 | 0.651 | 3357/26 | 322 (0.026%) | 2967734 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms | $64,386,074,954 | $62,914,699,954 | -0.574 | -0.645 | -0.674 | 0.674 | 0.474 | 0.474 | -0.577 | 0.761 | 0.658 | 3339/25 | 322 (0.026%) | 2968081 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms | $64,472,199,954 | $62,914,699,954 | -0.608 | -0.666 | -0.676 | 0.676 | 0.476 | 0.476 | -0.611 | 0.795 | 0.660 | 3290/26 | 322 (0.026%) | 2968546 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | 1-ms Burst Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $62,928,280,852 | $877,675,000 | $308,250,000 | $64,114,205,852 | $0 | -1.297 | -2.564 | -0.887 | 68.2% | 0.0% |
| B: Batch 70ms (hl), react 10ms | $62,914,699,954 | $83,925,000 | $1,258,875,000 | $64,257,499,954 | $0 | -0.670 | -2.548 | -0.420 | 68.2% | 31.2% |
| C: Uniform-Price FBA 100ms, react 10ms | $59,461,631,504 | $0 | $0 | $59,461,631,504 | $0 | -0.622 | n/a | n/a | 68.2% | 0.0% |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms | $62,914,699,954 | $83,925,000 | $1,258,875,000 | $64,257,499,954 | $0 | -0.670 | -2.548 | -0.420 | 68.2% | 31.2% |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms | $62,914,699,954 | $83,475,000 | $1,387,900,000 | $64,386,074,954 | $0 | -0.676 | -2.552 | -0.489 | 68.2% | 31.2% |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms | $62,914,699,954 | $82,250,000 | $1,475,250,000 | $64,472,199,954 | $0 | -0.677 | -2.541 | -0.524 | 68.2% | 31.2% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). `1-ms Burst Vol %` reflects directional trades merged within the same millisecond (`merge_taker`); see Raw Taker Diagnostics above for the unmerged lower bound.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 600s | $64,115,841,447 | $62,929,916,447 | -0.032 | -0.033 | -0.036 | 0.036 | <0 | 0.000 | 0.004 | 0.000 | 0.003 | 35107/48 | 54 (0.002%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 600s | $64,273,998,390 | $62,931,198,390 | -0.004 | -0.006 | -0.009 | 0.009 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 3357/26 | 37 (0.000%) | 3054805 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 600s | $59,393,267,067 | $62,931,111,397 | -0.004 | -0.005 | -0.009 | 0.009 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 2180/22 | 37 (0.000%) | 2847541 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms, PLACEBO shift 600s | $64,273,998,390 | $62,931,198,390 | -0.004 | -0.006 | -0.009 | 0.009 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 3357/26 | 37 (0.000%) | 3054805 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms, PLACEBO shift 600s | $64,402,573,390 | $62,931,198,390 | -0.006 | -0.008 | -0.011 | 0.011 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 3339/25 | 37 (0.000%) | 3058739 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms, PLACEBO shift 600s | $64,488,698,390 | $62,931,198,390 | -0.008 | -0.009 | -0.013 | 0.013 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 3290/26 | 37 (0.000%) | 3061386 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 3600s | $64,115,555,787 | $62,929,630,787 | -0.037 | -0.038 | -0.045 | 0.045 | <0 | 0.000 | -0.001 | 0.000 | -0.002 | 35107/48 | 44 (0.002%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 3600s | $64,273,946,309 | $62,931,146,309 | -0.009 | -0.010 | -0.017 | 0.017 | <0 | 0.000 | -0.001 | -0.001 | -0.003 | 3357/26 | 32 (0.000%) | 3057539 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 3600s | $59,409,961,292 | $62,931,168,286 | -0.009 | -0.010 | -0.019 | 0.019 | <0 | 0.000 | -0.001 | -0.001 | -0.002 | 2180/22 | 42 (0.000%) | 2849912 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms, PLACEBO shift 3600s | $64,273,946,309 | $62,931,146,309 | -0.009 | -0.010 | -0.017 | 0.017 | <0 | 0.000 | -0.001 | -0.001 | -0.003 | 3357/26 | 32 (0.000%) | 3057539 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms, PLACEBO shift 3600s | $64,402,521,309 | $62,931,146,309 | -0.012 | -0.012 | -0.020 | 0.020 | <0 | 0.000 | -0.001 | -0.001 | -0.003 | 3339/25 | 32 (0.000%) | 3061594 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms, PLACEBO shift 3600s | $64,488,646,309 | $62,931,146,309 | -0.012 | -0.014 | -0.023 | 0.023 | <0 | 0.000 | 0.000 | -0.002 | -0.003 | 3290/26 | 32 (0.000%) | 3064253 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.312 bp, B -0.667 bp, cancel-priority gap (B minus A) 0.645 bp.
- Placebo shift 600 s: A -0.036 bp, B -0.009 bp, gap 0.027 bp = 4.2% of the real gap.
- Placebo shift 3600 s: A -0.045 bp, B -0.017 bp, gap 0.027 bp = 4.2% of the real gap.
- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.645 bp = 49.1% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.312 bps, 95% CI [-1.425, -1.216]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier & Break-Even Leakage (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × eff_leak`, where `eff_leak = min(leak, capture)`), not measurements. Exposure mode for Regime D: `all`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 70ms (dec 0ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 10% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 25% | 0.000 ~ | -0.050 D<B | -0.101 D<B | -0.202 D<B | -0.303 D<B |
| 50% | 0.000 ~ | -0.135 D<B | -0.270 D<B | -0.540 D<B | -0.810 D<B |
| 75% | 0.000 ~ | -0.219 D<B | -0.438 D<B | -0.877 D<B | -1.316 D<B |
| 100% | 0.000 ~ | -0.303 D<B | -0.607 D<B | -1.215 D<B | -1.823 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 70ms (dec 35ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.006 B<D | 0.006 B<D | 0.006 B<D | 0.006 B<D | 0.006 B<D |
| 10% | 0.006 B<D | 0.006 B<D | 0.006 B<D | 0.006 B<D | 0.006 B<D |
| 25% | 0.006 B<D | -0.043 D<B | -0.094 D<B | -0.195 D<B | -0.297 D<B |
| 50% | 0.006 B<D | -0.128 D<B | -0.263 D<B | -0.533 D<B | -0.803 D<B |
| 75% | 0.006 B<D | -0.212 D<B | -0.432 D<B | -0.871 D<B | -1.310 D<B |
| 100% | 0.006 B<D | -0.297 D<B | -0.601 D<B | -1.208 D<B | -1.816 D<B |

#### Break-Even Residual Leakage `eff_leak*` (where All-In(D, dec 35ms) == All-In(B, 70ms))
Dywe is cheaper for takers (`D < B`) whenever actual residual leakage `eff_leak < eff_leak*` (note `eff_leak <= capture` by definition; values `>= capture` mean D stays cheaper for any `leak <= capture`).
| Ambient Capture / Impact | Cancel-Batch (31.2%) | 50.0% Exposed | 75.0% Exposed | 100.0% Exposed (All) |
|---|---:|---:|---:|---:|
| 10% / 0.500 bp | 25.6% | 16.0% | 10.6% | 8.0% |
| 10% / 1.000 bp | 28.8% | 18.0% | 12.0% | 9.0% |
| 25% / 1.000 bp | 76.8% | 48.0% | 32.0% | 24.0% |
| 50% / 1.000 bp | >100.0% | 98.1% | 65.4% | 49.0% |
| 50% / 2.000 bp | >100.0% | 99.1% | 66.0% | 49.5% |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 70ms (dec 70ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.008 B<D | 0.008 B<D | 0.008 B<D | 0.008 B<D | 0.008 B<D |
| 10% | 0.008 B<D | 0.008 B<D | 0.008 B<D | 0.008 B<D | 0.008 B<D |
| 25% | 0.008 B<D | -0.042 D<B | -0.092 D<B | -0.194 D<B | -0.295 D<B |
| 50% | 0.008 B<D | -0.126 D<B | -0.261 D<B | -0.531 D<B | -0.802 D<B |
| 75% | 0.008 B<D | -0.210 D<B | -0.430 D<B | -0.869 D<B | -1.308 D<B |
| 100% | 0.008 B<D | -0.295 D<B | -0.599 D<B | -1.207 D<B | -1.815 D<B |

## External Venue Documentation & Parameterized Assumptions (NOT simulation inputs)
- **Hyperliquid: intra-batch priority order:** Official docs (hyperliquid-l1/order-book, accessed Nov 2026) state that within each consensus batch, non-GTC/IOC actions (i.e. post-only / ALO) are processed first, followed by cancels, followed by GTC/IOC orders. Model this with `--b-batch-order hl`. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-l1/order-book_
- **Hyperliquid: block interval & buckets:** Official docs (for-developers/api/priority-fees, accessed Nov 2026) state that IOC orders are sorted in 70 ms unix-time buckets, typically corresponding to one block (with 1-2 consensus batches per block). End-to-end co-located latency is cited around ~0.2 s median in third-party mirrors. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/priority-fees ; https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview_
- **dYdX Chain (v4): block time:** Official onboarding FAQ (accessed Nov 2026) states an average block time of ~1 second and best-effort short-term cancels until block finality. Use `--b-window-ms 1000` to evaluate a 1-second batching cadence. _Source: https://docs.dydx.xyz/concepts/onboarding-faqs_
- **Not documented (parameterized assumptions):** No venue documentation specifies MM reaction time, requote threshold, feed latency, client-to-sequencer transit, or sniper reaction time. Sweep `--mm-react` (e.g. 1,2,5,10,25,40) and `--requote-thr-sub` (e.g. 15,50,150) rather than trusting a single point. _Source: n/a_

## Dywe Protocol Design Parameters & Threat Model
- **Dywe micro-batch window & threshold decryption RTT:** The 50 ms micro-batch window and 35 ms / 70 ms threshold decryption RTTs are Dywe protocol design parameters, not live multi-region Devnet measurements. No BLS12-381 cryptographic operations are executed inside this microstructure simulator.
- **Unified post-decrypt threat model & exposure mode:** All order messages (New ALO, Cancel, Taker) are threshold-encrypted client-side (BLS12-381) with uniform frame padding until batch_close, decrypting in matching-engine memory at clear_ns = batch_close + decrypt_ms before New (ALO) -> Cancel -> Taker sorting (`--d-batch-order hl`). Under `--d-exposure-mode all` (default), `eff_leak = min(leak_ppm, capture_ppm)` applies uniformly across all large bursts; under `cancel-batch` (legacy v0.3.4), it applies only to batches containing an MM cancel.
- **Stylized model disclaimer:** Regimes B (70 ms / 200 ms / 1000 ms) are stylized batching models, not byte-for-byte reproductions of Hyperliquid or dYdX matching engines.

## Methodological Boundaries & Scope
- Regime D timeline: orders commit at `batch_close`, decrypt at `clear_ns = batch_close + decrypt_ms`, and clear using `--d-batch-order hl` (exposure mode: `--d-exposure-mode all`).
- MM requote threshold: `--requote-thr-sub 150` (1.500 bp); synthetic sniper probe threshold: `--sniper-thr-sub 200` (2.000 bp), clip: `$25,000` (`--sniper-clip 0` disables probe).
- Single-denominator All-In Net: `--wait-cohort all` (`all` uses total executed taker volume `tk_n` across all 4 components).
- Under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net; inspect `Dropped Takers (USD%)` (0.001% precision) before comparing costs.
- 95% CI reflects block-resampling drift variance under fixed scenario assumptions; volatility autocorrelation and parameter uncertainty are not covered.
- Exogenous taker flow; single representative MM; model-derived `s*net` is not directly comparable to real exchange tick spreads.
