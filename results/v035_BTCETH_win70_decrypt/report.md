# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.5)
Window: 2026-11-10 + 7d | Pairs: BTCUSDT,ETHUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 70ms (hl, exp=all), FBA 100ms | requote_thr=1.500bp, sniper_thr=2.000bp | pending=drop | src=auto | mid=bidask | wait=all | sniper_clip=$25,000 | rebate=0.500bp (paid to MM), opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 14 of 14.

**TAKER FLOW DIAGNOSTICS:** Historical taker flow diagnostics (pure exchange volume $164,933,965,851, threshold >=$50,000): unmerged raw rows = 33665163 (699920 large rows, 57.8% of USD volume); 1-ms directional bursts (`merge_taker`) = 11472331 (644122 large bursts, 71.2% of USD volume).

**PROXY NOTE:** 14 pair-days used a trade-derived midpoint proxy (`--mid-source auto`, mode `bidask`). BidAskProxy diagnostics: 84131187 total observations, 695039 crossed (0.8% where ask < bid), mean valid (ask - bid) spread = 0.708 bps.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $100,159,462,293 | $99,169,012,293 | -1.174 | -1.486 | -1.614 | 1.614 | 1.414 | 1.414 | -1.166 | 0.631 | 0.879 | 25252/37 | 633 (0.020%) | 0 |
| B: Batch 70ms (hl), react 10ms | $100,247,381,174 | $99,164,531,174 | -0.618 | -0.719 | -0.874 | 0.874 | 0.674 | 0.674 | -0.620 | 0.833 | 0.888 | 2237/22 | 687 (0.025%) | 2629263 |
| C: Uniform-Price FBA 100ms, react 10ms | $88,072,027,755 | $99,171,032,601 | -0.532 | -0.651 | -0.815 | 0.815 | 0.615 | 0.541 | -0.467 | 0.836 | 0.909 | 1464/16 | 598 (0.018%) | 2407863 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms | $100,247,381,174 | $99,164,531,174 | -0.618 | -0.719 | -0.874 | 0.874 | 0.674 | 0.674 | -0.620 | 0.833 | 0.888 | 2237/22 | 687 (0.025%) | 2629263 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms | $100,350,856,174 | $99,164,531,174 | -0.689 | -0.736 | -0.873 | 0.873 | 0.673 | 0.673 | -0.691 | 0.905 | 0.886 | 2176/17 | 687 (0.025%) | 2629274 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms | $100,427,231,174 | $99,164,531,174 | -0.787 | -0.761 | -0.872 | 0.872 | 0.672 | 0.672 | -0.790 | 1.003 | 0.885 | 2079/16 | 687 (0.025%) | 2629510 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $67,555,558,813 | $65,727,758,813 | -1.434 | -1.705 | -1.871 | 1.871 | 1.671 | 1.671 | -1.418 | 0.746 | 0.999 | 46381/60 | 962 (0.025%) | 0 |
| B: Batch 70ms (hl), react 10ms | $67,649,717,351 | $65,700,917,351 | -0.654 | -0.734 | -0.902 | 0.902 | 0.702 | 0.702 | -0.660 | 0.936 | 0.978 | 4071/38 | 1110 (0.066%) | 2722991 |
| C: Uniform-Price FBA 100ms, react 10ms | $57,731,326,088 | $65,713,926,418 | -0.695 | -0.738 | -0.932 | 0.932 | 0.732 | 0.625 | -0.599 | 0.983 | 1.010 | 2759/27 | 1052 (0.046%) | 2457705 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms | $67,649,717,351 | $65,700,917,351 | -0.654 | -0.734 | -0.902 | 0.902 | 0.702 | 0.702 | -0.660 | 0.936 | 0.978 | 4071/38 | 1110 (0.066%) | 2722991 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms | $67,828,217,351 | $65,700,917,351 | -0.748 | -0.736 | -0.903 | 0.903 | 0.703 | 0.703 | -0.757 | 1.034 | 0.980 | 3904/39 | 1110 (0.066%) | 2723014 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms | $67,963,692,351 | $65,700,917,351 | -0.729 | -0.755 | -0.907 | 0.907 | 0.707 | 0.707 | -0.738 | 1.015 | 0.984 | 3649/39 | 1110 (0.066%) | 2723405 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $167,715,021,106 | $164,896,771,106 | -1.278 | -1.574 | -1.717 | 1.717 | 1.517 | 1.517 | -1.266 | 0.677 | 0.928 | 71633/97 | 1595 (0.022%) | 0 |
| B: Batch 70ms (hl), react 10ms | $167,897,098,525 | $164,865,448,525 | -0.633 | -0.725 | -0.885 | 0.885 | 0.685 | 0.685 | -0.636 | 0.874 | 0.924 | 6308/60 | 1797 (0.041%) | 5352254 |
| C: Uniform-Price FBA 100ms, react 10ms | $145,803,353,843 | $164,884,959,019 | -0.596 | -0.686 | -0.861 | 0.861 | 0.661 | 0.575 | -0.520 | 0.895 | 0.950 | 4223/43 | 1650 (0.029%) | 4865568 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms | $167,897,098,525 | $164,865,448,525 | -0.633 | -0.725 | -0.885 | 0.885 | 0.685 | 0.685 | -0.636 | 0.874 | 0.924 | 6308/60 | 1797 (0.041%) | 5352254 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms | $168,179,073,525 | $164,865,448,525 | -0.713 | -0.736 | -0.885 | 0.885 | 0.685 | 0.685 | -0.717 | 0.956 | 0.924 | 6080/56 | 1797 (0.041%) | 5352288 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms | $168,390,923,525 | $164,865,448,525 | -0.764 | -0.758 | -0.886 | 0.886 | 0.686 | 0.686 | -0.769 | 1.008 | 0.924 | 5728/55 | 1797 (0.041%) | 5352915 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | 1-ms Burst Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $164,896,771,106 | $1,790,825,000 | $1,027,425,000 | $167,715,021,106 | $0 | -1.710 | -2.754 | -1.112 | 71.2% | 0.0% |
| B: Batch 70ms (hl), react 10ms | $164,865,448,525 | $157,700,000 | $2,873,950,000 | $167,897,098,525 | $0 | -0.890 | -2.685 | -0.527 | 71.2% | 37.7% |
| C: Uniform-Price FBA 100ms, react 10ms | $145,803,353,843 | $0 | $0 | $145,803,353,843 | $0 | -0.861 | n/a | n/a | 71.2% | 0.0% |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms | $164,865,448,525 | $157,700,000 | $2,873,950,000 | $167,897,098,525 | $0 | -0.890 | -2.685 | -0.527 | 71.2% | 37.7% |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms | $164,865,448,525 | $152,000,000 | $3,161,625,000 | $168,179,073,525 | $0 | -0.889 | -2.687 | -0.583 | 71.2% | 37.7% |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms | $164,865,448,525 | $143,200,000 | $3,382,275,000 | $168,390,923,525 | $0 | -0.890 | -2.680 | -0.606 | 71.2% | 37.7% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). `1-ms Burst Vol %` reflects directional trades merged within the same millisecond (`merge_taker`); see Raw Taker Diagnostics above for the unmerged lower bound.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 600s | $167,747,623,501 | $164,929,373,501 | -0.036 | -0.037 | -0.055 | 0.055 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 71633/97 | 263 (0.002%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 600s | $167,962,261,508 | $164,930,611,508 | -0.011 | -0.013 | -0.031 | 0.031 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 6308/60 | 223 (0.002%) | 5504280 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 600s | $146,054,569,001 | $164,932,216,187 | -0.012 | -0.015 | -0.036 | 0.036 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 4223/43 | 185 (0.001%) | 4990907 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms, PLACEBO shift 600s | $167,962,261,508 | $164,930,611,508 | -0.011 | -0.013 | -0.031 | 0.031 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 6308/60 | 223 (0.002%) | 5504280 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms, PLACEBO shift 600s | $168,244,236,508 | $164,930,611,508 | -0.013 | -0.015 | -0.033 | 0.033 | <0 | 0.000 | -0.004 | 0.000 | -0.003 | 6080/56 | 223 (0.002%) | 5512256 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms, PLACEBO shift 600s | $168,456,086,508 | $164,930,611,508 | -0.015 | -0.017 | -0.035 | 0.035 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 5728/55 | 223 (0.002%) | 5517621 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 3600s | $167,750,058,106 | $164,931,808,106 | -0.039 | -0.039 | -0.046 | 0.046 | <0 | 0.000 | -0.005 | 0.000 | -0.004 | 71633/97 | 224 (0.001%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 3600s | $167,963,160,502 | $164,931,510,502 | -0.014 | -0.015 | -0.022 | 0.022 | <0 | 0.000 | -0.006 | 0.001 | -0.005 | 6308/60 | 244 (0.001%) | 5513066 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 3600s | $146,118,795,594 | $164,931,422,854 | -0.014 | -0.015 | -0.023 | 0.023 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 4223/43 | 214 (0.001%) | 4999295 |
| D: Dywe IBTE 70ms (dec 0ms, hl), react 10ms, PLACEBO shift 3600s | $167,963,160,502 | $164,931,510,502 | -0.014 | -0.015 | -0.022 | 0.022 | <0 | 0.000 | -0.006 | 0.001 | -0.005 | 6308/60 | 244 (0.001%) | 5513066 |
| D: Dywe IBTE 70ms (dec 35ms, hl), react 10ms, PLACEBO shift 3600s | $168,245,135,502 | $164,931,510,502 | -0.016 | -0.017 | -0.024 | 0.024 | <0 | 0.000 | -0.006 | 0.001 | -0.005 | 6080/56 | 244 (0.001%) | 5521606 |
| D: Dywe IBTE 70ms (dec 70ms, hl), react 10ms, PLACEBO shift 3600s | $168,456,985,502 | $164,931,510,502 | -0.016 | -0.019 | -0.025 | 0.025 | <0 | 0.000 | -0.006 | 0.001 | -0.005 | 5728/55 | 244 (0.001%) | 5527260 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.717 bp, B -0.885 bp, cancel-priority gap (B minus A) 0.831 bp.
- Placebo shift 600 s: A -0.055 bp, B -0.031 bp, gap 0.023 bp = 2.8% of the real gap.
- Placebo shift 3600 s: A -0.046 bp, B -0.022 bp, gap 0.023 bp = 2.8% of the real gap.
- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.831 bp = 48.4% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.717 bps, 95% CI [-1.871, -1.550]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier & Break-Even Leakage (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × eff_leak`, where `eff_leak = min(leak, capture)`), not measurements. Exposure mode for Regime D: `all`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 70ms (dec 0ms, hl, exp=all), react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 10% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 25% | 0.000 ~ | -0.052 D<B | -0.105 D<B | -0.211 D<B | -0.317 D<B |
| 50% | 0.000 ~ | -0.141 D<B | -0.282 D<B | -0.564 D<B | -0.846 D<B |
| 75% | 0.000 ~ | -0.229 D<B | -0.458 D<B | -0.917 D<B | -1.375 D<B |
| 100% | 0.000 ~ | -0.317 D<B | -0.635 D<B | -1.270 D<B | -1.905 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 70ms (dec 35ms, hl, exp=all), react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 10% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 25% | 0.000 ~ | -0.053 D<B | -0.106 D<B | -0.212 D<B | -0.318 D<B |
| 50% | 0.000 ~ | -0.141 D<B | -0.282 D<B | -0.564 D<B | -0.847 D<B |
| 75% | 0.000 ~ | -0.229 D<B | -0.459 D<B | -0.917 D<B | -1.376 D<B |
| 100% | 0.000 ~ | -0.317 D<B | -0.635 D<B | -1.270 D<B | -1.905 D<B |

#### Break-Even Residual Leakage `eff_leak*` (where All-In(D, dec 35ms) == All-In(B, 70ms))
Dywe is cheaper for takers (`D < B`) whenever actual residual leakage `eff_leak < eff_leak*` (note `eff_leak <= capture` by definition; values `>= capture` mean D stays cheaper for any `leak <= capture`).
| Ambient Capture / Impact | Cancel-Batch (37.7%) | 50.0% Exposed | 75.0% Exposed | 100.0% Exposed (All) |
|---|---:|---:|---:|---:|
| 10% / 0.500 bp | 26.8% | 20.2% | 13.4% | 10.1% |
| 10% / 1.000 bp | 26.6% | 20.1% | 13.4% | 10.0% |
| 25% / 1.000 bp | 66.4% | 50.1% | 33.4% | 25.0% |
| 50% / 1.000 bp | >100.0% | >100.0% | 66.7% | 50.0% |
| 50% / 2.000 bp | >100.0% | >100.0% | 66.7% | 50.0% |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 70ms (dec 70ms, hl, exp=all), react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 10% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 25% | 0.000 ~ | -0.052 D<B | -0.105 D<B | -0.211 D<B | -0.317 D<B |
| 50% | 0.000 ~ | -0.140 D<B | -0.282 D<B | -0.564 D<B | -0.846 D<B |
| 75% | 0.000 ~ | -0.229 D<B | -0.458 D<B | -0.917 D<B | -1.375 D<B |
| 100% | 0.000 ~ | -0.317 D<B | -0.634 D<B | -1.269 D<B | -1.905 D<B |

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
