# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.5)
Window: 2026-11-10 + 7d | Pairs: BTCUSDT,ETHUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 50ms (hl, exp=all), FBA 100ms | requote_thr=1.500bp, sniper_thr=2.000bp | pending=drop | src=auto | mid=bidask | wait=all | sniper_clip=$25,000 | rebate=0.500bp (paid to MM), opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 14 of 14.

**TAKER FLOW DIAGNOSTICS:** Historical taker flow diagnostics (pure exchange volume $164,933,965,851, threshold >=$50,000): unmerged raw rows = 33665163 (699920 large rows, 57.8% of USD volume); 1-ms directional bursts (`merge_taker`) = 11472331 (644122 large bursts, 71.2% of USD volume).

**PROXY NOTE:** 14 pair-days used a trade-derived midpoint proxy (`--mid-source auto`, mode `bidask`). BidAskProxy diagnostics: 84131187 total observations, 695039 crossed (0.8% where ask < bid), mean valid (ask - bid) spread = 0.708 bps.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $100,159,462,293 | $99,169,012,293 | -1.174 | -1.486 | -1.614 | 1.614 | 1.414 | 1.414 | -1.166 | 0.631 | 0.879 | 25252/37 | 633 (0.020%) | 0 |
| B: Batch 70ms (hl), react 10ms | $100,247,381,174 | $99,164,531,174 | -0.618 | -0.719 | -0.874 | 0.874 | 0.674 | 0.674 | -0.620 | 0.833 | 0.888 | 2237/22 | 687 (0.025%) | 2629263 |
| C: Uniform-Price FBA 100ms, react 10ms | $88,072,027,755 | $99,171,032,601 | -0.532 | -0.651 | -0.815 | 0.815 | 0.615 | 0.541 | -0.467 | 0.836 | 0.909 | 1464/16 | 598 (0.018%) | 2407863 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $100,235,176,127 | $99,170,026,127 | -0.713 | -0.916 | -1.012 | 1.012 | 0.812 | 0.812 | -0.715 | 0.781 | 0.879 | 3178/33 | 819 (0.019%) | 2823633 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $100,349,201,127 | $99,170,026,127 | -0.818 | -0.879 | -1.018 | 1.018 | 0.818 | 0.818 | -0.821 | 0.888 | 0.884 | 2958/31 | 819 (0.019%) | 2823977 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $100,427,701,127 | $99,170,026,127 | -0.845 | -0.889 | -1.016 | 1.016 | 0.816 | 0.816 | -0.849 | 0.915 | 0.882 | 2815/30 | 819 (0.019%) | 2824049 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $67,555,558,813 | $65,727,758,813 | -1.434 | -1.705 | -1.871 | 1.871 | 1.671 | 1.671 | -1.418 | 0.746 | 0.999 | 46381/60 | 962 (0.025%) | 0 |
| B: Batch 70ms (hl), react 10ms | $67,649,717,351 | $65,700,917,351 | -0.654 | -0.734 | -0.902 | 0.902 | 0.702 | 0.702 | -0.660 | 0.936 | 0.978 | 4071/38 | 1110 (0.066%) | 2722991 |
| C: Uniform-Price FBA 100ms, react 10ms | $57,731,326,088 | $65,713,926,418 | -0.695 | -0.738 | -0.932 | 0.932 | 0.732 | 0.625 | -0.599 | 0.983 | 1.010 | 2759/27 | 1052 (0.046%) | 2457705 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $67,644,829,731 | $65,713,154,731 | -0.823 | -0.927 | -1.098 | 1.098 | 0.898 | 0.898 | -0.831 | 0.915 | 0.982 | 6102/53 | 1334 (0.047%) | 2963556 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $67,847,154,731 | $65,713,154,731 | -0.950 | -0.921 | -1.094 | 1.094 | 0.894 | 0.894 | -0.964 | 1.048 | 0.977 | 5565/48 | 1334 (0.047%) | 2964211 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $67,978,304,731 | $65,713,154,731 | -0.933 | -0.921 | -1.103 | 1.103 | 0.903 | 0.903 | -0.947 | 1.030 | 0.987 | 5199/47 | 1334 (0.047%) | 2964290 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $167,715,021,106 | $164,896,771,106 | -1.278 | -1.574 | -1.717 | 1.717 | 1.517 | 1.517 | -1.266 | 0.677 | 0.928 | 71633/97 | 1595 (0.022%) | 0 |
| B: Batch 70ms (hl), react 10ms | $167,897,098,525 | $164,865,448,525 | -0.633 | -0.725 | -0.885 | 0.885 | 0.685 | 0.685 | -0.636 | 0.874 | 0.924 | 6308/60 | 1797 (0.041%) | 5352254 |
| C: Uniform-Price FBA 100ms, react 10ms | $145,803,353,843 | $164,884,959,019 | -0.596 | -0.686 | -0.861 | 0.861 | 0.661 | 0.575 | -0.520 | 0.895 | 0.950 | 4223/43 | 1650 (0.029%) | 4865568 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $167,880,005,858 | $164,883,180,858 | -0.757 | -0.920 | -1.047 | 1.047 | 0.847 | 0.847 | -0.761 | 0.834 | 0.920 | 9280/86 | 2153 (0.030%) | 5787189 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $168,196,355,858 | $164,883,180,858 | -0.871 | -0.896 | -1.049 | 1.049 | 0.849 | 0.849 | -0.878 | 0.951 | 0.922 | 8523/79 | 2153 (0.030%) | 5788188 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $168,406,005,858 | $164,883,180,858 | -0.881 | -0.902 | -1.051 | 1.051 | 0.851 | 0.851 | -0.888 | 0.961 | 0.924 | 8014/77 | 2153 (0.030%) | 5788339 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | 1-ms Burst Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $164,896,771,106 | $1,790,825,000 | $1,027,425,000 | $167,715,021,106 | $0 | -1.710 | -2.754 | -1.112 | 71.2% | 0.0% |
| B: Batch 70ms (hl), react 10ms | $164,865,448,525 | $157,700,000 | $2,873,950,000 | $167,897,098,525 | $0 | -0.890 | -2.685 | -0.527 | 71.2% | 37.7% |
| C: Uniform-Price FBA 100ms, react 10ms | $145,803,353,843 | $0 | $0 | $145,803,353,843 | $0 | -0.861 | n/a | n/a | 71.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $164,883,180,858 | $232,000,000 | $2,764,825,000 | $167,880,005,858 | $0 | -1.053 | -2.702 | -0.570 | 71.2% | 32.5% |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $164,883,180,858 | $213,075,000 | $3,100,100,000 | $168,196,355,858 | $0 | -1.055 | -2.695 | -0.599 | 71.2% | 32.5% |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $164,883,180,858 | $200,350,000 | $3,322,475,000 | $168,406,005,858 | $0 | -1.058 | -2.693 | -0.629 | 71.2% | 32.5% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). `1-ms Burst Vol %` reflects directional trades merged within the same millisecond (`merge_taker`); see Raw Taker Diagnostics above for the unmerged lower bound.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 600s | $167,747,623,501 | $164,929,373,501 | -0.036 | -0.037 | -0.055 | 0.055 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 71633/97 | 263 (0.002%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 600s | $167,962,261,508 | $164,930,611,508 | -0.011 | -0.013 | -0.031 | 0.031 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 6308/60 | 223 (0.002%) | 5504280 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 600s | $146,054,569,001 | $164,932,216,187 | -0.012 | -0.015 | -0.036 | 0.036 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 4223/43 | 185 (0.001%) | 4990907 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 600s | $167,929,706,597 | $164,932,881,597 | -0.013 | -0.015 | -0.034 | 0.034 | <0 | 0.000 | -0.004 | 0.000 | -0.004 | 9280/86 | 201 (0.000%) | 5959949 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 600s | $168,246,056,597 | $164,932,881,597 | -0.014 | -0.016 | -0.034 | 0.034 | <0 | 0.000 | -0.004 | 0.000 | -0.004 | 8523/79 | 201 (0.000%) | 5970122 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 600s | $168,455,706,597 | $164,932,881,597 | -0.017 | -0.019 | -0.036 | 0.036 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 8014/77 | 201 (0.000%) | 5977488 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 3600s | $167,750,058,106 | $164,931,808,106 | -0.039 | -0.039 | -0.046 | 0.046 | <0 | 0.000 | -0.005 | 0.000 | -0.004 | 71633/97 | 224 (0.001%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 3600s | $167,963,160,502 | $164,931,510,502 | -0.014 | -0.015 | -0.022 | 0.022 | <0 | 0.000 | -0.006 | 0.001 | -0.005 | 6308/60 | 244 (0.001%) | 5513066 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 3600s | $146,118,795,594 | $164,931,422,854 | -0.014 | -0.015 | -0.023 | 0.023 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 4223/43 | 214 (0.001%) | 4999295 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 3600s | $167,928,738,609 | $164,931,913,609 | -0.015 | -0.016 | -0.023 | 0.023 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 9280/86 | 240 (0.001%) | 5967992 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 3600s | $168,245,088,609 | $164,931,913,609 | -0.016 | -0.018 | -0.024 | 0.024 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 8523/79 | 240 (0.001%) | 5978752 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 3600s | $168,454,738,609 | $164,931,913,609 | -0.017 | -0.020 | -0.025 | 0.025 | <0 | 0.000 | -0.006 | 0.001 | -0.004 | 8014/77 | 240 (0.001%) | 5986452 |

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

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, hl, exp=all), react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | -0.003 ~ | -0.003 ~ | -0.003 ~ | -0.003 ~ | -0.003 ~ |
| 10% | -0.003 ~ | -0.003 ~ | -0.003 ~ | -0.003 ~ | -0.003 ~ |
| 25% | -0.003 ~ | -0.056 D<B | -0.109 D<B | -0.215 D<B | -0.321 D<B |
| 50% | -0.003 ~ | -0.144 D<B | -0.285 D<B | -0.568 D<B | -0.850 D<B |
| 75% | -0.003 ~ | -0.233 D<B | -0.462 D<B | -0.920 D<B | -1.379 D<B |
| 100% | -0.003 ~ | -0.321 D<B | -0.638 D<B | -1.273 D<B | -1.908 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, hl, exp=all), react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ |
| 10% | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ |
| 25% | -0.002 ~ | -0.055 D<B | -0.108 D<B | -0.214 D<B | -0.319 D<B |
| 50% | -0.002 ~ | -0.143 D<B | -0.284 D<B | -0.566 D<B | -0.849 D<B |
| 75% | -0.002 ~ | -0.231 D<B | -0.460 D<B | -0.919 D<B | -1.378 D<B |
| 100% | -0.002 ~ | -0.319 D<B | -0.637 D<B | -1.272 D<B | -1.907 D<B |

#### Break-Even Residual Leakage `eff_leak*` (where All-In(D, dec 35ms) == All-In(B, 70ms))
Dywe is cheaper for takers (`D < B`) whenever actual residual leakage `eff_leak < eff_leak*` (note `eff_leak <= capture` by definition; values `>= capture` mean D stays cheaper for any `leak <= capture`).
| Ambient Capture / Impact | Cancel-Batch (32.5%) | 50.0% Exposed | 75.0% Exposed | 100.0% Exposed (All) |
|---|---:|---:|---:|---:|
| 10% / 0.500 bp | 32.6% | 21.2% | 14.1% | 10.6% |
| 10% / 1.000 bp | 31.6% | 20.6% | 13.7% | 10.3% |
| 25% / 1.000 bp | 77.7% | 50.6% | 33.7% | 25.3% |
| 50% / 1.000 bp | >100.0% | >100.0% | 67.1% | 50.3% |
| 50% / 2.000 bp | >100.0% | >100.0% | 66.9% | 50.1% |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, hl, exp=all), react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 10% | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ | 0.000 ~ |
| 25% | 0.000 ~ | -0.052 D<B | -0.105 D<B | -0.211 D<B | -0.317 D<B |
| 50% | 0.000 ~ | -0.141 D<B | -0.282 D<B | -0.564 D<B | -0.846 D<B |
| 75% | 0.000 ~ | -0.229 D<B | -0.458 D<B | -0.917 D<B | -1.376 D<B |
| 100% | 0.000 ~ | -0.317 D<B | -0.634 D<B | -1.270 D<B | -1.905 D<B |

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
