# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.5)
Window: 2024-03-28 + 3d | Pairs: BTCUSDT,ETHUSDT,SOLUSDT | Bootstrap Block: 3h (24 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 50ms (hl, exp=all), FBA 100ms | requote_thr=0.150bp, sniper_thr=2.000bp | pending=drop | src=book | mid=bidask | wait=all | sniper_clip=$25,000 | rebate=0.500bp (paid to MM), opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 9 of 9.

**TAKER FLOW DIAGNOSTICS:** Historical taker flow diagnostics (pure exchange volume $62,931,377,194, threshold >=$50,000): unmerged raw rows = 8615487 (244685 large rows, 47.9% of USD volume); 1-ms directional bursts (`merge_taker`) = 4244321 (237805 large bursts, 68.2% of USD volume).

**BOOKTICKER DIAGNOSTICS:** Native L1 bookTicker diagnostics (9 files): 74023564 raw quote rows -> 3269988 distinct ms-level mid changes (4312026 same-ms collapsed, 66441550 unchanged-mid skipped); inter-update interval median = 3 ms, mean = 237.79 ms.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $36,675,784,160 | $36,624,559,160 | -0.761 | -0.932 | -1.005 | 1.005 | 0.805 | 0.805 | -0.760 | 0.533 | 0.579 | 1058/3 | 327 (0.019%) | 0 |
| B: Batch 70ms (hl), react 10ms | $36,666,926,431 | $36,604,201,431 | -0.291 | -0.403 | -0.450 | 0.450 | 0.250 | 0.250 | -0.291 | 0.619 | 0.578 | 88/3 | 559 (0.075%) | 1172321 |
| C: Uniform-Price FBA 100ms, react 10ms | $33,845,111,397 | $36,602,321,897 | -0.214 | -0.320 | -0.372 | 0.372 | 0.172 | 0.159 | -0.197 | 0.635 | 0.596 | 61/3 | 484 (0.080%) | 1083453 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $36,651,796,006 | $36,592,521,006 | -0.343 | -0.467 | -0.522 | 0.522 | 0.322 | 0.322 | -0.343 | 0.600 | 0.579 | 120/4 | 657 (0.107%) | 1250334 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $36,669,521,006 | $36,592,521,006 | -0.403 | -0.485 | -0.526 | 0.526 | 0.326 | 0.326 | -0.403 | 0.660 | 0.583 | 112/5 | 657 (0.107%) | 1250346 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $36,684,271,006 | $36,592,521,006 | -0.443 | -0.501 | -0.530 | 0.530 | 0.330 | 0.330 | -0.443 | 0.700 | 0.586 | 109/7 | 657 (0.107%) | 1250361 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.150 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $17,657,152,989 | $17,565,102,989 | -1.043 | -1.236 | -1.309 | 1.309 | 1.109 | 1.109 | -1.038 | 0.709 | 0.780 | 2241/3 | 341 (0.024%) | 0 |
| B: Batch 70ms (hl), react 10ms | $17,654,696,475 | $17,551,296,475 | -0.331 | -0.466 | -0.512 | 0.512 | 0.312 | 0.312 | -0.332 | 0.790 | 0.771 | 208/7 | 503 (0.102%) | 992433 |
| C: Uniform-Price FBA 100ms, react 10ms | $16,446,307,761 | $17,558,108,585 | -0.255 | -0.380 | -0.428 | 0.428 | 0.228 | 0.213 | -0.237 | 0.811 | 0.787 | 116/5 | 385 (0.063%) | 925302 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $17,647,678,812 | $17,548,253,812 | -0.460 | -0.610 | -0.670 | 0.670 | 0.470 | 0.470 | -0.461 | 0.764 | 0.773 | 284/3 | 590 (0.119%) | 1053895 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $17,667,728,812 | $17,548,253,812 | -0.539 | -0.625 | -0.668 | 0.668 | 0.468 | 0.468 | -0.540 | 0.843 | 0.771 | 256/5 | 590 (0.119%) | 1053908 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $17,685,278,812 | $17,548,253,812 | -0.584 | -0.637 | -0.665 | 0.665 | 0.465 | 0.465 | -0.585 | 0.888 | 0.768 | 240/6 | 590 (0.119%) | 1053927 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.150 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### SOLUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $9,047,420,384 | $8,726,470,384 | -1.737 | -1.870 | -1.822 | 1.822 | 1.622 | 1.622 | -1.728 | 0.847 | 0.741 | 8553/10 | 457 (0.042%) | 0 |
| B: Batch 70ms (hl), react 10ms | $9,081,548,106 | $8,720,523,106 | -0.351 | -0.510 | -0.418 | 0.418 | 0.218 | 0.218 | -0.358 | 0.873 | 0.732 | 776/24 | 818 (0.111%) | 911879 |
| C: Uniform-Price FBA 100ms, react 10ms | $8,338,118,906 | $8,723,569,724 | -0.354 | -0.477 | -0.395 | 0.395 | 0.195 | 0.178 | -0.332 | 0.908 | 0.754 | 508/17 | 639 (0.076%) | 830830 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $9,064,621,428 | $8,716,946,428 | -0.628 | -0.744 | -0.695 | 0.695 | 0.495 | 0.495 | -0.646 | 0.870 | 0.720 | 1111/25 | 889 (0.152%) | 992481 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $9,139,871,428 | $8,716,946,428 | -0.679 | -0.784 | -0.700 | 0.700 | 0.500 | 0.500 | -0.701 | 0.926 | 0.725 | 1000/35 | 889 (0.152%) | 992562 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $9,210,421,428 | $8,716,946,428 | -0.732 | -0.754 | -0.707 | 0.707 | 0.507 | 0.507 | -0.759 | 0.984 | 0.732 | 924/36 | 889 (0.152%) | 992642 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.150 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $63,380,357,533 | $62,916,132,533 | -0.979 | -1.151 | -1.207 | 1.207 | 1.007 | 1.007 | -0.972 | 0.626 | 0.661 | 11852/16 | 1125 (0.024%) | 0 |
| B: Batch 70ms (hl), react 10ms | $63,403,171,012 | $62,876,021,012 | -0.311 | -0.436 | -0.463 | 0.463 | 0.263 | 0.263 | -0.312 | 0.702 | 0.653 | 1072/34 | 1880 (0.087%) | 3076633 |
| C: Uniform-Price FBA 100ms, react 10ms | $58,629,538,064 | $62,884,000,206 | -0.245 | -0.359 | -0.391 | 0.391 | 0.191 | 0.176 | -0.227 | 0.722 | 0.671 | 685/25 | 1508 (0.075%) | 2839585 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $63,364,096,246 | $62,857,721,246 | -0.416 | -0.546 | -0.588 | 0.588 | 0.388 | 0.388 | -0.418 | 0.683 | 0.653 | 1515/32 | 2136 (0.117%) | 3296710 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $63,477,121,246 | $62,857,721,246 | -0.481 | -0.567 | -0.591 | 0.591 | 0.391 | 0.391 | -0.483 | 0.748 | 0.656 | 1368/45 | 2136 (0.117%) | 3296816 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $63,579,971,246 | $62,857,721,246 | -0.524 | -0.576 | -0.593 | 0.593 | 0.393 | 0.393 | -0.526 | 0.792 | 0.658 | 1273/49 | 2136 (0.117%) | 3296930 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.150 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | 1-ms Burst Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $62,916,132,533 | $296,300,000 | $167,925,000 | $63,380,357,533 | $0 | -1.201 | -2.490 | -1.127 | 68.2% | 0.0% |
| B: Batch 70ms (hl), react 10ms | $62,876,021,012 | $26,800,000 | $500,350,000 | $63,403,171,012 | $0 | -0.464 | -2.527 | -0.114 | 68.2% | 62.0% |
| C: Uniform-Price FBA 100ms, react 10ms | $58,629,538,064 | $0 | $0 | $58,629,538,064 | $0 | -0.391 | n/a | n/a | 68.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $62,857,721,246 | $37,875,000 | $468,500,000 | $63,364,096,246 | $0 | -0.590 | -2.547 | -0.095 | 68.2% | 54.8% |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $62,857,721,246 | $34,200,000 | $585,200,000 | $63,477,121,246 | $0 | -0.594 | -2.553 | -0.182 | 68.2% | 54.8% |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $62,857,721,246 | $31,825,000 | $690,425,000 | $63,579,971,246 | $0 | -0.596 | -2.565 | -0.227 | 68.2% | 54.8% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). `1-ms Burst Vol %` reflects directional trades merged within the same millisecond (`merge_taker`); see Raw Taker Diagnostics above for the unmerged lower bound.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 600s | $63,392,212,838 | $62,927,987,838 | -0.013 | -0.013 | -0.014 | 0.014 | <0 | 0.000 | 0.001 | 0.000 | 0.000 | 11852/16 | 266 (0.005%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 600s | $63,455,842,536 | $62,928,692,536 | -0.001 | -0.001 | -0.002 | 0.002 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 1072/34 | 270 (0.004%) | 3451296 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 600s | $58,646,208,381 | $62,928,496,047 | -0.002 | -0.002 | -0.003 | 0.003 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 685/25 | 294 (0.004%) | 3165814 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 600s | $63,434,552,761 | $62,928,177,761 | -0.002 | -0.002 | -0.003 | 0.003 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 1515/32 | 303 (0.005%) | 3693770 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 600s | $63,547,577,761 | $62,928,177,761 | -0.002 | -0.003 | -0.004 | 0.004 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 1368/45 | 303 (0.005%) | 3694180 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 600s | $63,650,427,761 | $62,928,177,761 | -0.002 | -0.003 | -0.005 | 0.005 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 1273/49 | 303 (0.005%) | 3694658 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.150 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 3600s | $63,388,333,993 | $62,924,108,993 | -0.014 | -0.013 | -0.018 | 0.018 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 11852/16 | 282 (0.011%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 3600s | $63,456,628,504 | $62,929,478,504 | -0.001 | 0.000 | -0.005 | 0.005 | <0 | 0.000 | 0.000 | -0.001 | 0.000 | 1072/34 | 250 (0.003%) | 3461009 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 3600s | $58,655,629,399 | $62,929,569,921 | 0.000 | -0.001 | -0.007 | 0.007 | <0 | 0.000 | 0.000 | -0.001 | 0.000 | 685/25 | 240 (0.002%) | 3175642 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 3600s | $63,435,974,522 | $62,929,599,522 | -0.001 | -0.001 | -0.006 | 0.006 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 1515/32 | 249 (0.002%) | 3702130 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 3600s | $63,548,999,522 | $62,929,599,522 | -0.001 | -0.002 | -0.007 | 0.007 | <0 | 0.000 | 0.000 | -0.001 | 0.000 | 1368/45 | 249 (0.002%) | 3702541 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 3600s | $63,651,849,522 | $62,929,599,522 | -0.001 | -0.003 | -0.008 | 0.008 | <0 | 0.000 | 0.001 | -0.002 | 0.000 | 1273/49 | 249 (0.002%) | 3703084 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.150 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.207 bp, B -0.463 bp, cancel-priority gap (B minus A) 0.744 bp.
- Placebo shift 600 s: A -0.014 bp, B -0.002 bp, gap 0.011 bp = 1.6% of the real gap.
- Placebo shift 3600 s: A -0.018 bp, B -0.005 bp, gap 0.012 bp = 1.7% of the real gap.
- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.744 bp = 61.6% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.207 bps, 95% CI [-1.323, -1.107]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier & Break-Even Leakage (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × eff_leak`, where `eff_leak = min(leak, capture)`), not measurements. Exposure mode for Regime D: `all`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 10% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 25% | 0.000 ~ | -0.050 D<B | -0.101 D<B | -0.203 D<B | -0.305 D<B |
| 50% | 0.000 ~ | -0.135 D<B | -0.271 D<B | -0.543 D<B | -0.815 D<B |
| 75% | 0.000 ~ | -0.220 D<B | -0.441 D<B | -0.883 D<B | -1.324 D<B |
| 100% | 0.000 ~ | -0.305 D<B | -0.611 D<B | -1.222 D<B | -1.834 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.003 ~ | 0.003 ~ | 0.003 ~ | 0.003 ~ | 0.003 ~ |
| 10% | 0.003 ~ | 0.003 ~ | 0.003 ~ | 0.003 ~ | 0.003 ~ |
| 25% | 0.003 ~ | -0.047 D<B | -0.098 D<B | -0.200 D<B | -0.302 D<B |
| 50% | 0.003 ~ | -0.132 D<B | -0.268 D<B | -0.540 D<B | -0.812 D<B |
| 75% | 0.003 ~ | -0.217 D<B | -0.438 D<B | -0.880 D<B | -1.322 D<B |
| 100% | 0.003 ~ | -0.302 D<B | -0.608 D<B | -1.220 D<B | -1.831 D<B |

#### Break-Even Residual Leakage `eff_leak*` (where All-In(D, dec 35ms) == All-In(B, 70ms))
Dywe is cheaper for takers (`D < B`) whenever actual residual leakage `eff_leak < eff_leak*` (note `eff_leak <= capture` by definition; values `>= capture` mean D stays cheaper for any `leak <= capture`).
| Ambient Capture / Impact | Cancel-Batch (54.8%) | 50.0% Exposed | 75.0% Exposed | 100.0% Exposed (All) |
|---|---:|---:|---:|---:|
| 10% / 0.500 bp | 16.5% | 18.0% | 12.0% | 9.0% |
| 10% / 1.000 bp | 17.3% | 19.0% | 12.7% | 9.5% |
| 25% / 1.000 bp | 44.7% | 49.0% | 32.7% | 24.5% |
| 50% / 1.000 bp | 90.3% | 99.1% | 66.0% | 49.5% |
| 50% / 2.000 bp | 90.8% | 99.6% | 66.4% | 49.8% |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D |
| 10% | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D |
| 25% | 0.005 B<D | -0.045 D<B | -0.096 D<B | -0.198 D<B | -0.300 D<B |
| 50% | 0.005 B<D | -0.130 D<B | -0.266 D<B | -0.538 D<B | -0.810 D<B |
| 75% | 0.005 B<D | -0.215 D<B | -0.436 D<B | -0.878 D<B | -1.320 D<B |
| 100% | 0.005 B<D | -0.300 D<B | -0.606 D<B | -1.217 D<B | -1.829 D<B |

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
- MM requote threshold: `--requote-thr-sub 15` (0.150 bp); synthetic sniper probe threshold: `--sniper-thr-sub 200` (2.000 bp), clip: `$25,000` (`--sniper-clip 0` disables probe).
- Single-denominator All-In Net: `--wait-cohort all` (`all` uses total executed taker volume `tk_n` across all 4 components).
- Under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net; inspect `Dropped Takers (USD%)` (0.001% precision) before comparing costs.
- 95% CI reflects block-resampling drift variance under fixed scenario assumptions; volatility autocorrelation and parameter uncertainty are not covered.
- Exogenous taker flow; single representative MM; model-derived `s*net` is not directly comparable to real exchange tick spreads.
