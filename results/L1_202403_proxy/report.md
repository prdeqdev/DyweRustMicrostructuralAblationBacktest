# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.4)
Window: 2024-03-28 + 3d | Pairs: BTCUSDT,ETHUSDT,SOLUSDT | Bootstrap Block: 3h (24 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 50ms (hl), FBA 100ms | pending=drop | src=proxy | mid=bidask | wait=all | sniper_clip=$25,000 | rebate=+0.500bp, opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 9 of 9.

**NOTE:** 9 pair-days used a trade-derived midpoint proxy (`--mid-source proxy`, mode `bidask`). BidAskProxy diagnostics: 22327383 total observations, 758841 crossed (3.3% where ask < bid), mean valid (ask - bid) spread = 0.758 bps.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $36,758,260,380 | -0.676 | -0.892 | -1.132 | 1.132 | 0.932 | 0.932 | -0.671 | 0.192 | 0.452 | 3682/4 | 42 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $36,785,073,250 | -0.367 | -0.548 | -0.738 | 0.738 | 0.538 | 0.538 | -0.366 | 0.279 | 0.451 | 392/2 | 52 (0.0%) | 1159680 |
| C: Uniform-Price FBA 100ms | react 10ms | $33,929,402,914 | -0.391 | -0.569 | -0.754 | 0.754 | 0.554 | 0.512 | -0.361 | 0.317 | 0.468 | 225/1 | 71 (0.0%) | 1076707 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $36,761,622,111 | -0.397 | -0.580 | -0.785 | 0.785 | 0.585 | 0.585 | -0.397 | 0.263 | 0.452 | 524/2 | 83 (0.0%) | 1229017 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $36,788,472,111 | -0.474 | -0.624 | -0.787 | 0.787 | 0.587 | 0.587 | -0.474 | 0.340 | 0.454 | 497/2 | 83 (0.0%) | 1229097 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $36,810,647,111 | -0.534 | -0.657 | -0.789 | 0.789 | 0.589 | 0.589 | -0.534 | 0.400 | 0.455 | 483/2 | 83 (0.0%) | 1229101 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $17,783,732,504 | -0.857 | -1.111 | -1.403 | 1.403 | 1.203 | 1.203 | -0.842 | 0.305 | 0.666 | 6720/7 | 35 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $17,814,217,309 | -0.397 | -0.593 | -0.841 | 0.841 | 0.641 | 0.641 | -0.397 | 0.411 | 0.654 | 697/3 | 88 (0.0%) | 976153 |
| C: Uniform-Price FBA 100ms | react 10ms | $16,534,399,976 | -0.439 | -0.639 | -0.884 | 0.884 | 0.684 | 0.635 | -0.408 | 0.439 | 0.666 | 480/7 | 120 (0.0%) | 916130 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $17,799,610,285 | -0.473 | -0.693 | -0.948 | 0.948 | 0.748 | 0.748 | -0.473 | 0.377 | 0.652 | 983/13 | 198 (0.0%) | 1027584 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $17,834,160,285 | -0.559 | -0.735 | -0.949 | 0.949 | 0.749 | 0.749 | -0.560 | 0.464 | 0.652 | 955/13 | 198 (0.0%) | 1027714 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $17,859,285,285 | -0.644 | -0.761 | -0.949 | 0.949 | 0.749 | 0.749 | -0.646 | 0.549 | 0.652 | 931/13 | 198 (0.0%) | 1027724 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### SOLUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $9,430,006,515 | -1.433 | -1.707 | -1.931 | 1.931 | 1.731 | 1.731 | -1.362 | 0.495 | 0.865 | 24150/18 | 62 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $9,470,169,470 | -0.553 | -0.781 | -0.938 | 0.938 | 0.738 | 0.738 | -0.571 | 0.627 | 0.794 | 2532/14 | 133 (0.0%) | 831134 |
| C: Uniform-Price FBA 100ms | react 10ms | $8,583,517,565 | -0.590 | -0.788 | -0.952 | 0.952 | 0.752 | 0.680 | -0.555 | 0.694 | 0.818 | 1763/13 | 131 (0.0%) | 776141 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $9,457,641,332 | -0.686 | -0.883 | -1.077 | 1.077 | 0.877 | 0.877 | -0.705 | 0.617 | 0.788 | 3724/22 | 213 (0.0%) | 879085 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $9,504,291,332 | -0.759 | -0.911 | -1.082 | 1.082 | 0.882 | 0.882 | -0.784 | 0.695 | 0.793 | 3612/20 | 213 (0.0%) | 879306 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $9,543,516,332 | -0.830 | -0.952 | -1.090 | 1.090 | 0.890 | 0.890 | -0.861 | 0.773 | 0.801 | 3525/20 | 213 (0.0%) | 879339 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $63,971,999,399 | -0.838 | -1.073 | -1.325 | 1.325 | 1.125 | 1.125 | -0.815 | 0.265 | 0.575 | 34552/29 | 139 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $64,069,460,029 | -0.403 | -0.595 | -0.796 | 0.796 | 0.596 | 0.596 | -0.403 | 0.364 | 0.557 | 3621/19 | 273 (0.0%) | 2966967 |
| C: Uniform-Price FBA 100ms | react 10ms | $59,047,320,455 | -0.433 | -0.620 | -0.819 | 0.819 | 0.619 | 0.571 | -0.401 | 0.403 | 0.574 | 2468/21 | 322 (0.0%) | 2768978 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $64,018,873,728 | -0.461 | -0.656 | -0.874 | 0.874 | 0.674 | 0.674 | -0.461 | 0.344 | 0.557 | 5231/37 | 494 (0.0%) | 3135686 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $64,126,923,728 | -0.540 | -0.697 | -0.876 | 0.876 | 0.676 | 0.676 | -0.541 | 0.424 | 0.559 | 5064/35 | 494 (0.0%) | 3136117 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $64,213,448,728 | -0.609 | -0.730 | -0.878 | 0.878 | 0.678 | 0.678 | -0.610 | 0.493 | 0.561 | 4939/35 | 494 (0.0%) | 3136164 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | Large Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $62,929,624,399 | $863,800,000 | $178,575,000 | $63,971,999,399 | $0 | -1.308 | -2.647 | -0.896 | 68.2% | 0.0% |
| B: Batch 70ms (hl) | react 10ms | $62,924,610,029 | $90,525,000 | $1,054,325,000 | $64,069,460,029 | $0 | -0.801 | -2.656 | -0.330 | 68.2% | 25.7% |
| C: Uniform-Price FBA 100ms | react 10ms | $59,047,320,455 | $0 | $0 | $59,047,320,455 | $0 | -0.819 | n/a | n/a | 68.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $62,900,473,728 | $130,775,000 | $987,625,000 | $64,018,873,728 | $0 | -0.879 | -2.577 | -0.337 | 68.2% | 22.1% |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $62,900,473,728 | $126,600,000 | $1,099,850,000 | $64,126,923,728 | $0 | -0.880 | -2.589 | -0.423 | 68.2% | 22.1% |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $62,900,473,728 | $123,475,000 | $1,189,500,000 | $64,213,448,728 | $0 | -0.882 | -2.597 | -0.480 | 68.2% | 22.1% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). In Regime C (Uniform-Price FBA), taker and sniper orders net internally into a single imbalance before clearing against the MM. `Cancel-Batch Large %` shows the share of large orders arriving inside a batch that also contains an MM cancel.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 600s | $63,973,572,995 | -0.032 | -0.033 | -0.035 | 0.035 | <0 | 0.000 | 0.003 | 0.000 | 0.002 | 34552/29 | 37 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 600s | $64,075,888,998 | -0.003 | -0.004 | -0.007 | 0.007 | <0 | 0.000 | 0.002 | 0.000 | 0.001 | 3621/19 | 38 (0.0%) | 3049814 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 600s | $59,213,869,986 | -0.004 | -0.005 | -0.008 | 0.008 | <0 | 0.000 | 0.002 | 0.000 | 0.001 | 2468/21 | 45 (0.0%) | 2843756 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 600s | $64,049,115,527 | -0.005 | -0.006 | -0.009 | 0.009 | <0 | 0.000 | 0.002 | -0.001 | 0.001 | 5231/37 | 56 (0.0%) | 3221712 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 600s | $64,157,165,527 | -0.007 | -0.008 | -0.011 | 0.011 | <0 | 0.000 | 0.002 | 0.000 | 0.001 | 5064/35 | 56 (0.0%) | 3225310 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 600s | $64,243,690,527 | -0.008 | -0.009 | -0.012 | 0.012 | <0 | 0.000 | 0.002 | 0.000 | 0.001 | 4939/35 | 56 (0.0%) | 3228219 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 3600s | $63,973,052,813 | -0.032 | -0.031 | -0.036 | 0.036 | <0 | 0.000 | 0.003 | 0.000 | 0.003 | 34552/29 | 41 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 3600s | $64,075,590,046 | -0.002 | -0.002 | -0.006 | 0.006 | <0 | 0.000 | 0.004 | 0.000 | 0.003 | 3621/19 | 34 (0.0%) | 3051729 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 3600s | $59,222,496,688 | -0.001 | -0.002 | -0.007 | 0.007 | <0 | 0.000 | 0.004 | -0.001 | 0.003 | 2468/21 | 40 (0.0%) | 2846027 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 3600s | $64,049,476,435 | -0.003 | -0.003 | -0.008 | 0.008 | <0 | 0.000 | 0.004 | 0.000 | 0.003 | 5231/37 | 41 (0.0%) | 3223561 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 3600s | $64,157,526,435 | -0.004 | -0.005 | -0.010 | 0.010 | <0 | 0.000 | 0.004 | -0.001 | 0.003 | 5064/35 | 41 (0.0%) | 3227295 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 3600s | $64,244,051,435 | -0.005 | -0.006 | -0.012 | 0.012 | <0 | 0.000 | 0.005 | -0.001 | 0.003 | 4939/35 | 41 (0.0%) | 3230277 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.325 bp, B -0.796 bp, cancel-priority gap (B minus A) 0.529 bp.
- Placebo shift 600 s: A -0.035 bp, B -0.007 bp, gap 0.028 bp = 5.3% of the real gap.
- Placebo shift 3600 s: A -0.036 bp, B -0.006 bp, gap 0.029 bp = 5.5% of the real gap.
- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.529 bp = 39.9% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.325 bps, 95% CI [-1.439, -1.224]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × leak`), not measurements. At `capture = 0%`, MEV extraction is strictly zero regardless of `leak_ppm`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, hl) | react 10ms | pending=drop | src=proxy | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 10% | 0.000 ~ | -0.026 D<B | -0.052 D<B | -0.105 D<B | -0.158 D<B |
| 25% | 0.000 ~ | -0.077 D<B | -0.154 D<B | -0.308 D<B | -0.462 D<B |
| 50% | 0.000 ~ | -0.161 D<B | -0.323 D<B | -0.646 D<B | -0.969 D<B |
| 75% | 0.000 ~ | -0.246 D<B | -0.492 D<B | -0.984 D<B | -1.476 D<B |
| 100% | 0.000 ~ | -0.330 D<B | -0.661 D<B | -1.322 D<B | -1.984 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, hl) | react 10ms | pending=drop | src=proxy | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.002 B<D | 0.002 B<D | 0.002 B<D | 0.002 B<D | 0.002 B<D |
| 10% | 0.002 B<D | -0.024 D<B | -0.050 D<B | -0.103 D<B | -0.155 D<B |
| 25% | 0.002 B<D | -0.075 D<B | -0.152 D<B | -0.306 D<B | -0.460 D<B |
| 50% | 0.002 B<D | -0.159 D<B | -0.321 D<B | -0.644 D<B | -0.967 D<B |
| 75% | 0.002 B<D | -0.244 D<B | -0.490 D<B | -0.982 D<B | -1.474 D<B |
| 100% | 0.002 B<D | -0.328 D<B | -0.659 D<B | -1.320 D<B | -1.982 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, hl) | react 10ms | pending=drop | src=proxy | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.004 B<D | 0.004 B<D | 0.004 B<D | 0.004 B<D | 0.004 B<D |
| 10% | 0.004 B<D | -0.021 D<B | -0.048 D<B | -0.100 D<B | -0.153 D<B |
| 25% | 0.004 B<D | -0.072 D<B | -0.149 D<B | -0.303 D<B | -0.458 D<B |
| 50% | 0.004 B<D | -0.157 D<B | -0.318 D<B | -0.642 D<B | -0.965 D<B |
| 75% | 0.004 B<D | -0.241 D<B | -0.487 D<B | -0.980 D<B | -1.472 D<B |
| 100% | 0.004 B<D | -0.326 D<B | -0.657 D<B | -1.318 D<B | -1.979 D<B |

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
