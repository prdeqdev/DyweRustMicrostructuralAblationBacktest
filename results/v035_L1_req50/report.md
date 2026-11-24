# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.5)
Window: 2024-03-28 + 3d | Pairs: BTCUSDT,ETHUSDT,SOLUSDT | Bootstrap Block: 3h (24 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 50ms (hl, exp=all), FBA 100ms | requote_thr=0.500bp, sniper_thr=2.000bp | pending=drop | src=book | mid=bidask | wait=all | sniper_clip=$25,000 | rebate=0.500bp (paid to MM), opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 9 of 9.

**TAKER FLOW DIAGNOSTICS:** Historical taker flow diagnostics (pure exchange volume $62,931,377,194, threshold >=$50,000): unmerged raw rows = 8615487 (244685 large rows, 47.9% of USD volume); 1-ms directional bursts (`merge_taker`) = 4244321 (237805 large bursts, 68.2% of USD volume).

**BOOKTICKER DIAGNOSTICS:** Native L1 bookTicker diagnostics (9 files): 74023564 raw quote rows -> 3269988 distinct ms-level mid changes (4312026 same-ms collapsed, 66441550 unchanged-mid skipped); inter-update interval median = 3 ms, mean = 237.79 ms.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $36,684,699,005 | $36,628,324,005 | -0.783 | -0.954 | -1.027 | 1.027 | 0.827 | 0.827 | -0.781 | 0.533 | 0.579 | 1202/4 | 114 (0.009%) | 0 |
| B: Batch 70ms (hl), react 10ms | $36,685,578,663 | $36,616,028,663 | -0.334 | -0.447 | -0.493 | 0.493 | 0.293 | 0.293 | -0.334 | 0.619 | 0.577 | 107/3 | 214 (0.043%) | 1164372 |
| C: Uniform-Price FBA 100ms, react 10ms | $33,846,499,837 | $36,599,871,529 | -0.262 | -0.369 | -0.422 | 0.422 | 0.222 | 0.205 | -0.242 | 0.629 | 0.592 | 68/2 | 222 (0.087%) | 1078958 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $36,683,644,971 | $36,617,844,971 | -0.380 | -0.504 | -0.559 | 0.559 | 0.359 | 0.359 | -0.380 | 0.600 | 0.579 | 148/5 | 294 (0.038%) | 1237169 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $36,702,119,971 | $36,617,844,971 | -0.440 | -0.522 | -0.564 | 0.564 | 0.364 | 0.364 | -0.440 | 0.660 | 0.584 | 136/5 | 294 (0.038%) | 1237193 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $36,716,919,971 | $36,617,844,971 | -0.480 | -0.538 | -0.567 | 0.567 | 0.367 | 0.367 | -0.480 | 0.699 | 0.587 | 131/6 | 294 (0.038%) | 1237216 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $17,669,060,579 | $17,566,110,579 | -1.061 | -1.254 | -1.327 | 1.327 | 1.127 | 1.127 | -1.055 | 0.709 | 0.781 | 2666/3 | 199 (0.018%) | 0 |
| B: Batch 70ms (hl), react 10ms | $17,677,193,637 | $17,561,168,637 | -0.376 | -0.511 | -0.556 | 0.556 | 0.356 | 0.356 | -0.376 | 0.791 | 0.770 | 270/3 | 290 (0.046%) | 983559 |
| C: Uniform-Price FBA 100ms, react 10ms | $16,465,925,229 | $17,565,612,837 | -0.302 | -0.427 | -0.476 | 0.476 | 0.276 | 0.257 | -0.281 | 0.811 | 0.787 | 150/3 | 149 (0.021%) | 920061 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $17,670,437,227 | $17,559,787,227 | -0.498 | -0.648 | -0.709 | 0.709 | 0.509 | 0.509 | -0.499 | 0.765 | 0.775 | 347/8 | 276 (0.054%) | 1039551 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $17,691,537,227 | $17,559,787,227 | -0.577 | -0.664 | -0.708 | 0.708 | 0.508 | 0.508 | -0.578 | 0.844 | 0.774 | 323/8 | 276 (0.054%) | 1039576 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $17,709,262,227 | $17,559,787,227 | -0.623 | -0.675 | -0.705 | 0.705 | 0.505 | 0.505 | -0.624 | 0.890 | 0.771 | 311/8 | 276 (0.054%) | 1039612 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### SOLUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $9,076,916,721 | $8,728,341,721 | -1.754 | -1.887 | -1.839 | 1.839 | 1.639 | 1.639 | -1.744 | 0.847 | 0.741 | 9730/17 | 270 (0.021%) | 0 |
| B: Batch 70ms (hl), react 10ms | $9,120,289,566 | $8,722,564,566 | -0.396 | -0.554 | -0.463 | 0.463 | 0.263 | 0.263 | -0.404 | 0.872 | 0.731 | 928/21 | 491 (0.087%) | 868274 |
| C: Uniform-Price FBA 100ms, react 10ms | $8,378,696,218 | $8,725,681,552 | -0.393 | -0.515 | -0.435 | 0.435 | 0.235 | 0.214 | -0.369 | 0.908 | 0.753 | 596/13 | 301 (0.051%) | 801038 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $9,105,262,583 | $8,722,112,583 | -0.666 | -0.780 | -0.732 | 0.732 | 0.532 | 0.532 | -0.685 | 0.871 | 0.718 | 1297/19 | 505 (0.092%) | 931903 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $9,183,987,583 | $8,722,112,583 | -0.717 | -0.820 | -0.738 | 0.738 | 0.538 | 0.538 | -0.740 | 0.926 | 0.724 | 1193/22 | 505 (0.092%) | 932131 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $9,255,137,583 | $8,722,112,583 | -0.770 | -0.791 | -0.746 | 0.746 | 0.546 | 0.546 | -0.798 | 0.984 | 0.732 | 1124/24 | 505 (0.092%) | 932337 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $63,430,676,305 | $62,922,776,305 | -0.999 | -1.171 | -1.227 | 1.227 | 1.027 | 1.027 | -0.991 | 0.626 | 0.661 | 13598/24 | 583 (0.013%) | 0 |
| B: Batch 70ms (hl), react 10ms | $63,483,061,866 | $62,899,761,866 | -0.355 | -0.480 | -0.506 | 0.506 | 0.306 | 0.306 | -0.356 | 0.702 | 0.652 | 1305/27 | 995 (0.050%) | 3016205 |
| C: Uniform-Price FBA 100ms, react 10ms | $58,691,121,284 | $62,891,165,918 | -0.292 | -0.406 | -0.439 | 0.439 | 0.239 | 0.221 | -0.271 | 0.718 | 0.669 | 814/18 | 672 (0.063%) | 2800057 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $63,459,344,781 | $62,899,744,781 | -0.454 | -0.584 | -0.626 | 0.626 | 0.426 | 0.426 | -0.455 | 0.683 | 0.654 | 1792/32 | 1075 (0.050%) | 3208623 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $63,577,644,781 | $62,899,744,781 | -0.518 | -0.605 | -0.629 | 0.629 | 0.429 | 0.429 | -0.520 | 0.748 | 0.657 | 1652/35 | 1075 (0.050%) | 3208900 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $63,681,319,781 | $62,899,744,781 | -0.562 | -0.613 | -0.631 | 0.631 | 0.431 | 0.431 | -0.564 | 0.792 | 0.659 | 1566/38 | 1075 (0.050%) | 3209165 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | 1-ms Burst Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $62,922,776,305 | $339,950,000 | $167,950,000 | $63,430,676,305 | $0 | -1.220 | -2.477 | -1.081 | 68.2% | 0.0% |
| B: Batch 70ms (hl), react 10ms | $62,899,761,866 | $32,625,000 | $550,675,000 | $63,483,061,866 | $0 | -0.508 | -2.503 | -0.159 | 68.2% | 51.8% |
| C: Uniform-Price FBA 100ms, react 10ms | $58,691,121,284 | $0 | $0 | $58,691,121,284 | $0 | -0.439 | n/a | n/a | 68.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $62,899,744,781 | $44,800,000 | $514,800,000 | $63,459,344,781 | $0 | -0.629 | -2.503 | -0.137 | 68.2% | 45.0% |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $62,899,744,781 | $41,300,000 | $636,600,000 | $63,577,644,781 | $0 | -0.632 | -2.503 | -0.235 | 68.2% | 45.0% |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $62,899,744,781 | $39,150,000 | $742,425,000 | $63,681,319,781 | $0 | -0.634 | -2.520 | -0.298 | 68.2% | 45.0% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). `1-ms Burst Vol %` reflects directional trades merged within the same millisecond (`merge_taker`); see Raw Taker Diagnostics above for the unmerged lower bound.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 600s | $63,436,873,378 | $62,928,973,378 | -0.014 | -0.014 | -0.015 | 0.015 | <0 | 0.000 | 0.001 | 0.000 | 0.000 | 13598/24 | 148 (0.003%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 600s | $63,510,951,780 | $62,927,651,780 | -0.001 | -0.002 | -0.003 | 0.003 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 1305/27 | 186 (0.005%) | 3257913 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 600s | $58,702,420,702 | $62,928,480,538 | -0.003 | -0.003 | -0.004 | 0.004 | <0 | 0.000 | 0.000 | 0.000 | -0.001 | 814/18 | 138 (0.004%) | 3017960 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 600s | $63,487,848,408 | $62,928,248,408 | -0.003 | -0.003 | -0.004 | 0.004 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 1792/32 | 147 (0.004%) | 3456358 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 600s | $63,606,148,408 | $62,928,248,408 | -0.003 | -0.004 | -0.005 | 0.005 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 1652/35 | 147 (0.004%) | 3457566 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 600s | $63,709,823,408 | $62,928,248,408 | -0.004 | -0.005 | -0.007 | 0.007 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 1566/38 | 147 (0.004%) | 3458701 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 3600s | $63,437,533,825 | $62,929,633,825 | -0.014 | -0.013 | -0.018 | 0.018 | <0 | 0.000 | 0.001 | 0.000 | 0.001 | 13598/24 | 144 (0.002%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 3600s | $63,513,277,575 | $62,929,977,575 | 0.000 | 0.000 | -0.005 | 0.005 | <0 | 0.000 | 0.002 | -0.001 | 0.000 | 1305/27 | 142 (0.002%) | 3264642 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 3600s | $58,710,526,737 | $62,929,203,757 | 0.000 | 0.000 | -0.006 | 0.006 | <0 | 0.000 | 0.002 | -0.001 | 0.000 | 814/18 | 119 (0.003%) | 3024732 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 3600s | $63,488,610,349 | $62,929,010,349 | 0.000 | 0.000 | -0.005 | 0.005 | <0 | 0.000 | 0.001 | 0.000 | 0.000 | 1792/32 | 117 (0.003%) | 3461704 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 3600s | $63,606,910,349 | $62,929,010,349 | -0.001 | -0.001 | -0.006 | 0.006 | <0 | 0.000 | 0.002 | -0.001 | 0.000 | 1652/35 | 117 (0.003%) | 3462922 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 3600s | $63,710,585,349 | $62,929,010,349 | -0.001 | -0.002 | -0.008 | 0.008 | <0 | 0.000 | 0.003 | -0.002 | 0.000 | 1566/38 | 117 (0.003%) | 3464130 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`0.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.227 bp, B -0.506 bp, cancel-priority gap (B minus A) 0.720 bp.
- Placebo shift 600 s: A -0.015 bp, B -0.003 bp, gap 0.012 bp = 1.7% of the real gap.
- Placebo shift 3600 s: A -0.018 bp, B -0.005 bp, gap 0.013 bp = 1.8% of the real gap.
- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.720 bp = 58.7% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.227 bps, 95% CI [-1.342, -1.128]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier & Break-Even Leakage (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × eff_leak`, where `eff_leak = min(leak, capture)`), not measurements. Exposure mode for Regime D: `all`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ |
| 10% | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ | 0.001 ~ |
| 25% | 0.001 ~ | -0.049 D<B | -0.100 D<B | -0.202 D<B | -0.303 D<B |
| 50% | 0.001 ~ | -0.134 D<B | -0.270 D<B | -0.541 D<B | -0.813 D<B |
| 75% | 0.001 ~ | -0.219 D<B | -0.439 D<B | -0.881 D<B | -1.322 D<B |
| 100% | 0.001 ~ | -0.304 D<B | -0.609 D<B | -1.221 D<B | -1.832 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.004 B<D | 0.004 B<D | 0.004 B<D | 0.004 B<D | 0.004 B<D |
| 10% | 0.004 B<D | 0.004 B<D | 0.004 B<D | 0.004 B<D | 0.004 B<D |
| 25% | 0.004 B<D | -0.046 D<B | -0.097 D<B | -0.199 D<B | -0.301 D<B |
| 50% | 0.004 B<D | -0.131 D<B | -0.267 D<B | -0.538 D<B | -0.810 D<B |
| 75% | 0.004 B<D | -0.216 D<B | -0.436 D<B | -0.878 D<B | -1.320 D<B |
| 100% | 0.004 B<D | -0.301 D<B | -0.606 D<B | -1.218 D<B | -1.829 D<B |

#### Break-Even Residual Leakage `eff_leak*` (where All-In(D, dec 35ms) == All-In(B, 70ms))
Dywe is cheaper for takers (`D < B`) whenever actual residual leakage `eff_leak < eff_leak*` (note `eff_leak <= capture` by definition; values `>= capture` mean D stays cheaper for any `leak <= capture`).
| Ambient Capture / Impact | Cancel-Batch (45.0%) | 50.0% Exposed | 75.0% Exposed | 100.0% Exposed (All) |
|---|---:|---:|---:|---:|
| 10% / 0.500 bp | 19.1% | 17.2% | 11.5% | 8.6% |
| 10% / 1.000 bp | 20.6% | 18.6% | 12.4% | 9.3% |
| 25% / 1.000 bp | 53.9% | 48.6% | 32.4% | 24.3% |
| 50% / 1.000 bp | >100.0% | 98.7% | 65.8% | 49.3% |
| 50% / 2.000 bp | >100.0% | 99.3% | 66.2% | 49.6% |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, hl, exp=all), react 10ms | pending=drop | src=book | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.007 B<D | 0.007 B<D | 0.007 B<D | 0.007 B<D | 0.007 B<D |
| 10% | 0.007 B<D | 0.007 B<D | 0.007 B<D | 0.006 B<D | 0.006 B<D |
| 25% | 0.007 B<D | -0.043 D<B | -0.094 D<B | -0.196 D<B | -0.298 D<B |
| 50% | 0.007 B<D | -0.128 D<B | -0.264 D<B | -0.536 D<B | -0.808 D<B |
| 75% | 0.007 B<D | -0.213 D<B | -0.434 D<B | -0.876 D<B | -1.317 D<B |
| 100% | 0.007 B<D | -0.298 D<B | -0.604 D<B | -1.215 D<B | -1.827 D<B |

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
- MM requote threshold: `--requote-thr-sub 50` (0.500 bp); synthetic sniper probe threshold: `--sniper-thr-sub 200` (2.000 bp), clip: `$25,000` (`--sniper-clip 0` disables probe).
- Single-denominator All-In Net: `--wait-cohort all` (`all` uses total executed taker volume `tk_n` across all 4 components).
- Under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net; inspect `Dropped Takers (USD%)` (0.001% precision) before comparing costs.
- 95% CI reflects block-resampling drift variance under fixed scenario assumptions; volatility autocorrelation and parameter uncertainty are not covered.
- Exogenous taker flow; single representative MM; model-derived `s*net` is not directly comparable to real exchange tick spreads.
