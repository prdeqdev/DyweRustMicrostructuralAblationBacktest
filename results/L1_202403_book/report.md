# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.4)
Window: 2024-03-28 + 3d | Pairs: BTCUSDT,ETHUSDT,SOLUSDT | Bootstrap Block: 3h (24 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 50ms (hl), FBA 100ms | pending=drop | src=book | mid=bidask | wait=all | sniper_clip=$25,000 | rebate=+0.500bp, opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 9 of 9.

**DATA DIAGNOSTICS:** Native L1 bookTicker diagnostics (9 files): 74023564 raw quote rows -> 3269988 distinct ms-level mid changes (4312026 same-ms collapsed, 66441550 unchanged-mid skipped); inter-update interval median = 3 ms, mean = 237.79 ms.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $36,790,884,363 | -0.871 | -1.041 | -1.115 | 1.115 | 0.915 | 0.915 | -0.867 | 0.533 | 0.581 | 4246/7 | 49 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $36,820,561,211 | -0.501 | -0.613 | -0.660 | 0.660 | 0.460 | 0.460 | -0.501 | 0.619 | 0.578 | 421/2 | 57 (0.0%) | 1158839 |
| C: Uniform-Price FBA 100ms | react 10ms | $33,995,816,314 | -0.453 | -0.560 | -0.612 | 0.612 | 0.412 | 0.380 | -0.418 | 0.635 | 0.597 | 266/3 | 73 (0.0%) | 1076005 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $36,805,037,617 | -0.537 | -0.661 | -0.717 | 0.717 | 0.517 | 0.517 | -0.537 | 0.599 | 0.579 | 571/7 | 193 (0.0%) | 1228382 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $36,835,312,617 | -0.598 | -0.679 | -0.722 | 0.722 | 0.522 | 0.522 | -0.598 | 0.660 | 0.583 | 561/7 | 193 (0.0%) | 1228480 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $36,854,362,617 | -0.637 | -0.696 | -0.725 | 0.725 | 0.525 | 0.525 | -0.637 | 0.699 | 0.587 | 553/7 | 193 (0.0%) | 1228514 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $17,821,358,950 | -1.129 | -1.321 | -1.395 | 1.395 | 1.195 | 1.195 | -1.118 | 0.709 | 0.786 | 7063/17 | 59 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $17,859,555,936 | -0.529 | -0.663 | -0.710 | 0.710 | 0.510 | 0.510 | -0.530 | 0.788 | 0.768 | 749/6 | 141 (0.0%) | 975655 |
| C: Uniform-Price FBA 100ms | react 10ms | $16,646,057,427 | -0.479 | -0.603 | -0.654 | 0.654 | 0.454 | 0.422 | -0.446 | 0.811 | 0.787 | 446/4 | 40 (0.0%) | 915483 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $17,848,506,814 | -0.644 | -0.793 | -0.855 | 0.855 | 0.655 | 0.655 | -0.646 | 0.765 | 0.774 | 993/11 | 147 (0.0%) | 1026988 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $17,887,581,814 | -0.722 | -0.809 | -0.855 | 0.855 | 0.655 | 0.655 | -0.725 | 0.844 | 0.773 | 978/12 | 147 (0.0%) | 1027109 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $17,911,931,814 | -0.768 | -0.820 | -0.852 | 0.852 | 0.652 | 0.652 | -0.772 | 0.890 | 0.771 | 969/11 | 147 (0.0%) | 1027161 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### SOLUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $9,501,962,539 | -1.829 | -1.961 | -1.923 | 1.923 | 1.723 | 1.723 | -1.814 | 0.846 | 0.755 | 23798/24 | 108 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $9,577,382,807 | -0.538 | -0.693 | -0.615 | 0.615 | 0.415 | 0.415 | -0.558 | 0.872 | 0.729 | 2187/18 | 124 (0.0%) | 833240 |
| C: Uniform-Price FBA 100ms | react 10ms | $8,819,757,763 | -0.550 | -0.670 | -0.604 | 0.604 | 0.404 | 0.368 | -0.525 | 0.907 | 0.751 | 1468/15 | 156 (0.0%) | 777741 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $9,560,127,135 | -0.794 | -0.908 | -0.871 | 0.871 | 0.671 | 0.671 | -0.835 | 0.871 | 0.707 | 3041/27 | 232 (0.0%) | 882025 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $9,633,527,135 | -0.847 | -0.949 | -0.879 | 0.879 | 0.679 | 0.679 | -0.890 | 0.926 | 0.715 | 2978/27 | 232 (0.0%) | 882719 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $9,681,177,135 | -0.902 | -0.925 | -0.889 | 0.889 | 0.689 | 0.689 | -0.948 | 0.984 | 0.726 | 2922/30 | 232 (0.0%) | 882982 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $64,114,205,852 | -1.085 | -1.255 | -1.312 | 1.312 | 1.112 | 1.112 | -1.068 | 0.626 | 0.670 | 35107/48 | 216 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $64,257,499,954 | -0.515 | -0.639 | -0.667 | 0.667 | 0.467 | 0.467 | -0.517 | 0.701 | 0.651 | 3357/26 | 322 (0.0%) | 2967734 |
| C: Uniform-Price FBA 100ms | react 10ms | $59,461,631,504 | -0.475 | -0.588 | -0.622 | 0.622 | 0.422 | 0.390 | -0.441 | 0.722 | 0.671 | 2180/22 | 269 (0.0%) | 2769229 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $64,213,671,566 | -0.605 | -0.735 | -0.778 | 0.778 | 0.578 | 0.578 | -0.609 | 0.683 | 0.653 | 4605/45 | 572 (0.0%) | 3137395 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $64,356,421,566 | -0.669 | -0.756 | -0.782 | 0.782 | 0.582 | 0.582 | -0.674 | 0.748 | 0.656 | 4517/46 | 572 (0.0%) | 3138308 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $64,447,471,566 | -0.713 | -0.765 | -0.785 | 0.785 | 0.585 | 0.585 | -0.718 | 0.792 | 0.659 | 4444/48 | 572 (0.0%) | 3138657 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | Large Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $62,928,280,852 | $877,675,000 | $308,250,000 | $64,114,205,852 | $0 | -1.297 | -2.564 | -0.887 | 68.2% | 0.0% |
| B: Batch 70ms (hl) | react 10ms | $62,914,699,954 | $83,925,000 | $1,258,875,000 | $64,257,499,954 | $0 | -0.670 | -2.548 | -0.420 | 68.2% | 31.2% |
| C: Uniform-Price FBA 100ms | react 10ms | $59,461,631,504 | $0 | $0 | $59,461,631,504 | $0 | -0.622 | n/a | n/a | 68.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $62,908,246,566 | $115,125,000 | $1,190,300,000 | $64,213,671,566 | $0 | -0.782 | -2.587 | -0.404 | 68.2% | 26.7% |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $62,908,246,566 | $112,925,000 | $1,335,250,000 | $64,356,421,566 | $0 | -0.785 | -2.588 | -0.483 | 68.2% | 26.7% |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $62,908,246,566 | $111,100,000 | $1,428,125,000 | $64,447,471,566 | $0 | -0.788 | -2.590 | -0.531 | 68.2% | 26.7% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). In Regime C (Uniform-Price FBA), taker and sniper orders net internally into a single imbalance before clearing against the MM. `Cancel-Batch Large %` shows the share of large orders arriving inside a batch that also contains an MM cancel.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 600s | $64,115,841,447 | -0.032 | -0.033 | -0.036 | 0.036 | <0 | 0.000 | 0.004 | 0.000 | 0.003 | 35107/48 | 54 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 600s | $64,273,998,390 | -0.004 | -0.006 | -0.009 | 0.009 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 3357/26 | 37 (0.0%) | 3054805 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 600s | $59,393,267,067 | -0.004 | -0.005 | -0.009 | 0.009 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 2180/22 | 37 (0.0%) | 2847541 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 600s | $64,236,459,799 | -0.005 | -0.006 | -0.010 | 0.010 | <0 | 0.000 | 0.002 | 0.000 | 0.003 | 4605/45 | 44 (0.0%) | 3227245 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 600s | $64,379,209,799 | -0.007 | -0.009 | -0.012 | 0.012 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 4517/46 | 44 (0.0%) | 3232151 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 600s | $64,470,259,799 | -0.009 | -0.010 | -0.015 | 0.015 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 4444/48 | 44 (0.0%) | 3235304 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 3600s | $64,115,555,787 | -0.037 | -0.038 | -0.045 | 0.045 | <0 | 0.000 | -0.001 | 0.000 | -0.002 | 35107/48 | 44 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 3600s | $64,273,946,309 | -0.009 | -0.010 | -0.017 | 0.017 | <0 | 0.000 | -0.001 | -0.001 | -0.003 | 3357/26 | 32 (0.0%) | 3057539 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 3600s | $59,409,961,292 | -0.009 | -0.010 | -0.019 | 0.019 | <0 | 0.000 | -0.001 | -0.001 | -0.002 | 2180/22 | 42 (0.0%) | 2849912 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 3600s | $64,236,634,154 | -0.010 | -0.011 | -0.018 | 0.018 | <0 | 0.000 | -0.002 | 0.000 | -0.003 | 4605/45 | 41 (0.0%) | 3229244 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 3600s | $64,379,384,154 | -0.012 | -0.014 | -0.021 | 0.021 | <0 | 0.000 | -0.001 | -0.001 | -0.003 | 4517/46 | 41 (0.0%) | 3234124 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 3600s | $64,470,434,154 | -0.013 | -0.015 | -0.023 | 0.023 | <0 | 0.000 | 0.000 | -0.002 | -0.003 | 4444/48 | 41 (0.0%) | 3237435 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.312 bp, B -0.667 bp, cancel-priority gap (B minus A) 0.645 bp.
- Placebo shift 600 s: A -0.036 bp, B -0.009 bp, gap 0.027 bp = 4.2% of the real gap.
- Placebo shift 3600 s: A -0.045 bp, B -0.017 bp, gap 0.027 bp = 4.2% of the real gap.
- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.645 bp = 49.1% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.312 bps, 95% CI [-1.425, -1.216]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × leak`), not measurements. At `capture = 0%`, MEV extraction is strictly zero regardless of `leak_ppm`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, hl) | react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ |
| 10% | 0.001 ~ | -0.023 D<B | -0.048 D<B | -0.097 D<B | -0.146 D<B |
| 25% | 0.001 ~ | -0.073 D<B | -0.149 D<B | -0.300 D<B | -0.450 D<B |
| 50% | 0.001 ~ | -0.158 D<B | -0.318 D<B | -0.637 D<B | -0.957 D<B |
| 75% | 0.001 ~ | -0.242 D<B | -0.486 D<B | -0.975 D<B | -1.463 D<B |
| 100% | 0.001 ~ | -0.327 D<B | -0.655 D<B | -1.313 D<B | -1.970 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, hl) | react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D |
| 10% | 0.005 B<D | -0.019 D<B | -0.044 D<B | -0.093 D<B | -0.143 D<B |
| 25% | 0.005 B<D | -0.070 D<B | -0.145 D<B | -0.296 D<B | -0.447 D<B |
| 50% | 0.005 B<D | -0.154 D<B | -0.314 D<B | -0.634 D<B | -0.953 D<B |
| 75% | 0.005 B<D | -0.239 D<B | -0.483 D<B | -0.971 D<B | -1.460 D<B |
| 100% | 0.005 B<D | -0.323 D<B | -0.652 D<B | -1.309 D<B | -1.966 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, hl) | react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.008 B<D | 0.008 B<D | 0.008 B<D | 0.008 B<D | 0.008 B<D |
| 10% | 0.008 B<D | -0.016 D<B | -0.041 D<B | -0.090 D<B | -0.140 D<B |
| 25% | 0.008 B<D | -0.067 D<B | -0.142 D<B | -0.293 D<B | -0.444 D<B |
| 50% | 0.008 B<D | -0.151 D<B | -0.311 D<B | -0.631 D<B | -0.950 D<B |
| 75% | 0.008 B<D | -0.236 D<B | -0.480 D<B | -0.968 D<B | -1.457 D<B |
| 100% | 0.008 B<D | -0.320 D<B | -0.649 D<B | -1.306 D<B | -1.963 D<B |

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
