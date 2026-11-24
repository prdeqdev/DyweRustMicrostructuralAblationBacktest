# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.5)
Window: 2024-03-28 + 3d | Pairs: BTCUSDT,ETHUSDT,SOLUSDT | Bootstrap Block: 3h (24 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 50ms (hl, exp=all), FBA 100ms | requote_thr=1.500bp, sniper_thr=2.000bp | pending=drop | src=book | mid=bidask | wait=all | sniper_clip=$25,000 | rebate=0.500bp (paid to MM), opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 9 of 9.

**TAKER FLOW DIAGNOSTICS:** Historical taker flow diagnostics (pure exchange volume $62,931,377,194, threshold >=$50,000): unmerged raw rows = 8615487 (244685 large rows, 47.9% of USD volume); 1-ms directional bursts (`merge_taker`) = 4244321 (237805 large bursts, 68.2% of USD volume).

**BOOKTICKER DIAGNOSTICS:** Native L1 bookTicker diagnostics (9 files): 74023564 raw quote rows -> 3269988 distinct ms-level mid changes (4312026 same-ms collapsed, 66441550 unchanged-mid skipped); inter-update interval median = 3 ms, mean = 237.79 ms.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $36,790,884,363 | $36,630,409,363 | -0.871 | -1.041 | -1.115 | 1.115 | 0.915 | 0.915 | -0.867 | 0.533 | 0.581 | 4246/7 | 49 (0.003%) | 0 |
| B: Batch 70ms (hl), react 10ms | $36,820,561,211 | $36,625,786,211 | -0.501 | -0.613 | -0.660 | 0.660 | 0.460 | 0.460 | -0.501 | 0.619 | 0.578 | 421/2 | 57 (0.016%) | 1158839 |
| C: Uniform-Price FBA 100ms, react 10ms | $33,995,816,314 | $36,628,216,864 | -0.453 | -0.560 | -0.612 | 0.612 | 0.412 | 0.380 | -0.418 | 0.635 | 0.597 | 266/3 | 73 (0.009%) | 1076005 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $36,805,037,617 | $36,618,762,617 | -0.537 | -0.661 | -0.717 | 0.717 | 0.517 | 0.517 | -0.537 | 0.599 | 0.579 | 571/7 | 193 (0.035%) | 1228382 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $36,835,312,617 | $36,618,762,617 | -0.598 | -0.679 | -0.722 | 0.722 | 0.522 | 0.522 | -0.598 | 0.660 | 0.583 | 561/7 | 193 (0.035%) | 1228480 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $36,854,362,617 | $36,618,762,617 | -0.637 | -0.696 | -0.725 | 0.725 | 0.525 | 0.525 | -0.637 | 0.699 | 0.587 | 553/7 | 193 (0.035%) | 1228514 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $17,821,358,950 | $17,568,333,950 | -1.129 | -1.321 | -1.395 | 1.395 | 1.195 | 1.195 | -1.118 | 0.709 | 0.786 | 7063/17 | 59 (0.005%) | 0 |
| B: Batch 70ms (hl), react 10ms | $17,859,555,936 | $17,561,330,936 | -0.529 | -0.663 | -0.710 | 0.710 | 0.510 | 0.510 | -0.530 | 0.788 | 0.768 | 749/6 | 141 (0.045%) | 975655 |
| C: Uniform-Price FBA 100ms, react 10ms | $16,646,057,427 | $17,567,057,419 | -0.479 | -0.603 | -0.654 | 0.654 | 0.454 | 0.422 | -0.446 | 0.811 | 0.787 | 446/4 | 40 (0.012%) | 915483 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $17,848,506,814 | $17,561,906,814 | -0.644 | -0.793 | -0.855 | 0.855 | 0.655 | 0.655 | -0.646 | 0.765 | 0.774 | 993/11 | 147 (0.042%) | 1026988 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $17,887,581,814 | $17,561,906,814 | -0.722 | -0.809 | -0.855 | 0.855 | 0.655 | 0.655 | -0.725 | 0.844 | 0.773 | 978/12 | 147 (0.042%) | 1027109 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $17,911,931,814 | $17,561,906,814 | -0.768 | -0.820 | -0.852 | 0.852 | 0.652 | 0.652 | -0.772 | 0.890 | 0.771 | 969/11 | 147 (0.042%) | 1027161 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### SOLUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $9,501,962,539 | $8,729,537,539 | -1.829 | -1.961 | -1.923 | 1.923 | 1.723 | 1.723 | -1.814 | 0.846 | 0.755 | 23798/24 | 108 (0.007%) | 0 |
| B: Batch 70ms (hl), react 10ms | $9,577,382,807 | $8,727,582,807 | -0.538 | -0.693 | -0.615 | 0.615 | 0.415 | 0.415 | -0.558 | 0.872 | 0.729 | 2187/18 | 124 (0.030%) | 833240 |
| C: Uniform-Price FBA 100ms, react 10ms | $8,819,757,763 | $8,727,426,115 | -0.550 | -0.670 | -0.604 | 0.604 | 0.404 | 0.368 | -0.525 | 0.907 | 0.751 | 1468/15 | 156 (0.031%) | 777741 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $9,560,127,135 | $8,727,577,135 | -0.794 | -0.908 | -0.871 | 0.871 | 0.671 | 0.671 | -0.835 | 0.871 | 0.707 | 3041/27 | 232 (0.030%) | 882025 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $9,633,527,135 | $8,727,577,135 | -0.847 | -0.949 | -0.879 | 0.879 | 0.679 | 0.679 | -0.890 | 0.926 | 0.715 | 2978/27 | 232 (0.030%) | 882719 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $9,681,177,135 | $8,727,577,135 | -0.902 | -0.925 | -0.889 | 0.889 | 0.689 | 0.689 | -0.948 | 0.984 | 0.726 | 2922/30 | 232 (0.030%) | 882982 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $64,114,205,852 | $62,928,280,852 | -1.085 | -1.255 | -1.312 | 1.312 | 1.112 | 1.112 | -1.068 | 0.626 | 0.670 | 35107/48 | 216 (0.004%) | 0 |
| B: Batch 70ms (hl), react 10ms | $64,257,499,954 | $62,914,699,954 | -0.515 | -0.639 | -0.667 | 0.667 | 0.467 | 0.467 | -0.517 | 0.701 | 0.651 | 3357/26 | 322 (0.026%) | 2967734 |
| C: Uniform-Price FBA 100ms, react 10ms | $59,461,631,504 | $62,922,700,398 | -0.475 | -0.588 | -0.622 | 0.622 | 0.422 | 0.390 | -0.441 | 0.722 | 0.671 | 2180/22 | 269 (0.013%) | 2769229 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $64,213,671,566 | $62,908,246,566 | -0.605 | -0.735 | -0.778 | 0.778 | 0.578 | 0.578 | -0.609 | 0.683 | 0.653 | 4605/45 | 572 (0.036%) | 3137395 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $64,356,421,566 | $62,908,246,566 | -0.669 | -0.756 | -0.782 | 0.782 | 0.582 | 0.582 | -0.674 | 0.748 | 0.656 | 4517/46 | 572 (0.036%) | 3138308 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $64,447,471,566 | $62,908,246,566 | -0.713 | -0.765 | -0.785 | 0.785 | 0.585 | 0.585 | -0.718 | 0.792 | 0.659 | 4444/48 | 572 (0.036%) | 3138657 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | 1-ms Burst Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $62,928,280,852 | $877,675,000 | $308,250,000 | $64,114,205,852 | $0 | -1.297 | -2.564 | -0.887 | 68.2% | 0.0% |
| B: Batch 70ms (hl), react 10ms | $62,914,699,954 | $83,925,000 | $1,258,875,000 | $64,257,499,954 | $0 | -0.670 | -2.548 | -0.420 | 68.2% | 31.2% |
| C: Uniform-Price FBA 100ms, react 10ms | $59,461,631,504 | $0 | $0 | $59,461,631,504 | $0 | -0.622 | n/a | n/a | 68.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $62,908,246,566 | $115,125,000 | $1,190,300,000 | $64,213,671,566 | $0 | -0.782 | -2.587 | -0.404 | 68.2% | 26.7% |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $62,908,246,566 | $112,925,000 | $1,335,250,000 | $64,356,421,566 | $0 | -0.785 | -2.588 | -0.483 | 68.2% | 26.7% |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $62,908,246,566 | $111,100,000 | $1,428,125,000 | $64,447,471,566 | $0 | -0.788 | -2.590 | -0.531 | 68.2% | 26.7% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). `1-ms Burst Vol %` reflects directional trades merged within the same millisecond (`merge_taker`); see Raw Taker Diagnostics above for the unmerged lower bound.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 600s | $64,115,841,447 | $62,929,916,447 | -0.032 | -0.033 | -0.036 | 0.036 | <0 | 0.000 | 0.004 | 0.000 | 0.003 | 35107/48 | 54 (0.002%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 600s | $64,273,998,390 | $62,931,198,390 | -0.004 | -0.006 | -0.009 | 0.009 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 3357/26 | 37 (0.000%) | 3054805 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 600s | $59,393,267,067 | $62,931,111,397 | -0.004 | -0.005 | -0.009 | 0.009 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 2180/22 | 37 (0.000%) | 2847541 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 600s | $64,236,459,799 | $62,931,034,799 | -0.005 | -0.006 | -0.010 | 0.010 | <0 | 0.000 | 0.002 | 0.000 | 0.003 | 4605/45 | 44 (0.000%) | 3227245 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 600s | $64,379,209,799 | $62,931,034,799 | -0.007 | -0.009 | -0.012 | 0.012 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 4517/46 | 44 (0.000%) | 3232151 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 600s | $64,470,259,799 | $62,931,034,799 | -0.009 | -0.010 | -0.015 | 0.015 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 4444/48 | 44 (0.000%) | 3235304 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 3600s | $64,115,555,787 | $62,929,630,787 | -0.037 | -0.038 | -0.045 | 0.045 | <0 | 0.000 | -0.001 | 0.000 | -0.002 | 35107/48 | 44 (0.002%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 3600s | $64,273,946,309 | $62,931,146,309 | -0.009 | -0.010 | -0.017 | 0.017 | <0 | 0.000 | -0.001 | -0.001 | -0.003 | 3357/26 | 32 (0.000%) | 3057539 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 3600s | $59,409,961,292 | $62,931,168,286 | -0.009 | -0.010 | -0.019 | 0.019 | <0 | 0.000 | -0.001 | -0.001 | -0.002 | 2180/22 | 42 (0.000%) | 2849912 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 3600s | $64,236,634,154 | $62,931,209,154 | -0.010 | -0.011 | -0.018 | 0.018 | <0 | 0.000 | -0.002 | 0.000 | -0.003 | 4605/45 | 41 (0.000%) | 3229244 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 3600s | $64,379,384,154 | $62,931,209,154 | -0.012 | -0.014 | -0.021 | 0.021 | <0 | 0.000 | -0.001 | -0.001 | -0.003 | 4517/46 | 41 (0.000%) | 3234124 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 3600s | $64,470,434,154 | $62,931,209,154 | -0.013 | -0.015 | -0.023 | 0.023 | <0 | 0.000 | 0.000 | -0.002 | -0.003 | 4444/48 | 41 (0.000%) | 3237435 |

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

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ |
| 10% | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ |
| 25% | 0.001 ~ | -0.049 D<B | -0.099 D<B | -0.201 D<B | -0.302 D<B |
| 50% | 0.001 ~ | -0.133 D<B | -0.268 D<B | -0.538 D<B | -0.808 D<B |
| 75% | 0.001 ~ | -0.218 D<B | -0.437 D<B | -0.876 D<B | -1.315 D<B |
| 100% | 0.001 ~ | -0.302 D<B | -0.606 D<B | -1.214 D<B | -1.821 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D |
| 10% | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.004 B<D |
| 25% | 0.005 B<D | -0.045 D<B | -0.096 D<B | -0.197 D<B | -0.298 D<B |
| 50% | 0.005 B<D | -0.129 D<B | -0.265 D<B | -0.535 D<B | -0.805 D<B |
| 75% | 0.005 B<D | -0.214 D<B | -0.433 D<B | -0.872 D<B | -1.311 D<B |
| 100% | 0.005 B<D | -0.298 D<B | -0.602 D<B | -1.210 D<B | -1.818 D<B |

#### Break-Even Residual Leakage `eff_leak*` (where All-In(D, dec 35ms) == All-In(B, 70ms))
Dywe is cheaper for takers (`D < B`) whenever actual residual leakage `eff_leak < eff_leak*` (note `eff_leak <= capture` by definition; values `>= capture` mean D stays cheaper for any `leak <= capture`).
| Ambient Capture / Impact | Cancel-Batch (26.7%) | 50.0% Exposed | 75.0% Exposed | 100.0% Exposed (All) |
|---|---:|---:|---:|---:|
| 10% / 0.500 bp | 31.6% | 16.9% | 11.3% | 8.4% |
| 10% / 1.000 bp | 34.5% | 18.4% | 12.3% | 9.2% |
| 25% / 1.000 bp | 90.5% | 48.5% | 32.3% | 24.2% |
| 50% / 1.000 bp | >100.0% | 98.5% | 65.7% | 49.2% |
| 50% / 2.000 bp | >100.0% | 99.3% | 66.2% | 49.6% |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.008 B<D | 0.008 B<D | 0.008 B<D | 0.008 B<D | 0.008 B<D |
| 10% | 0.008 B<D | 0.008 B<D | 0.008 B<D | 0.007 B<D | 0.007 B<D |
| 25% | 0.008 B<D | -0.042 D<B | -0.093 D<B | -0.194 D<B | -0.296 D<B |
| 50% | 0.008 B<D | -0.126 D<B | -0.262 D<B | -0.532 D<B | -0.802 D<B |
| 75% | 0.008 B<D | -0.211 D<B | -0.430 D<B | -0.869 D<B | -1.309 D<B |
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
