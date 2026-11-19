# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.3)
Window: 2026-11-10 + 7d | Pairs: BTCUSDT,ETHUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 18 | B 200ms (hl), D 50ms (hl), FBA 100ms | pending=drop | mid=bidask | wait=all | rebate=+0.500bp, opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 14 of 14.

**NOTE:** 14 pair-days used a trade-derived midpoint proxy (mode `bidask`) because `bookTicker` files were not supplied. BidAskProxy diagnostics: 84131187 total observations, 695039 crossed (0.8% where ask < bid), mean valid (ask - bid) spread = 0.708 bps.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $100,159,462,293 | -1.174 | -1.486 | -1.614 | 1.614 | 1.414 | 1.414 | -1.166 | 0.631 | 0.879 | 25252/37 | 633 (0.0%) | 0 |
| B: Batch 200ms (hl) | react 10ms | $100,338,603,960 | -0.359 | -0.405 | -0.531 | 0.531 | 0.331 | 0.331 | -0.359 | 0.911 | 0.883 | 680/9 | 450 (0.0%) | 1838715 |
| C: Uniform-Price FBA 100ms | react 10ms | $88,072,027,755 | -0.532 | -0.651 | -0.815 | 0.815 | 0.615 | 0.541 | -0.467 | 0.836 | 0.909 | 1464/16 | 598 (0.0%) | 2407863 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $100,235,176,127 | -0.713 | -0.916 | -1.012 | 1.012 | 0.812 | 0.812 | -0.715 | 0.781 | 0.879 | 3178/33 | 819 (0.0%) | 2823633 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $100,349,201,127 | -0.818 | -0.879 | -1.018 | 1.018 | 0.818 | 0.818 | -0.821 | 0.888 | 0.884 | 2958/31 | 819 (0.0%) | 2823977 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $100,427,701,127 | -0.845 | -0.889 | -1.016 | 1.016 | 0.816 | 0.816 | -0.849 | 0.915 | 0.882 | 2815/30 | 819 (0.0%) | 2824049 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $67,555,558,813 | -1.434 | -1.705 | -1.871 | 1.871 | 1.671 | 1.671 | -1.418 | 0.746 | 0.999 | 46381/60 | 962 (0.0%) | 0 |
| B: Batch 200ms (hl) | react 10ms | $67,756,857,065 | -0.381 | -0.430 | -0.585 | 0.585 | 0.385 | 0.385 | -0.382 | 0.997 | 1.000 | 1220/12 | 434 (0.0%) | 1844279 |
| C: Uniform-Price FBA 100ms | react 10ms | $57,731,326,088 | -0.695 | -0.738 | -0.932 | 0.932 | 0.732 | 0.625 | -0.599 | 0.983 | 1.010 | 2759/27 | 1052 (0.0%) | 2457705 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $67,644,829,731 | -0.823 | -0.927 | -1.098 | 1.098 | 0.898 | 0.898 | -0.831 | 0.915 | 0.982 | 6102/53 | 1334 (0.0%) | 2963556 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $67,847,154,731 | -0.950 | -0.921 | -1.094 | 1.094 | 0.894 | 0.894 | -0.964 | 1.048 | 0.977 | 5565/48 | 1334 (0.0%) | 2964211 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $67,978,304,731 | -0.933 | -0.921 | -1.103 | 1.103 | 0.903 | 0.903 | -0.947 | 1.030 | 0.987 | 5199/47 | 1334 (0.0%) | 2964290 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $167,715,021,106 | -1.278 | -1.574 | -1.717 | 1.717 | 1.517 | 1.517 | -1.266 | 0.677 | 0.928 | 71633/97 | 1595 (0.0%) | 0 |
| B: Batch 200ms (hl) | react 10ms | $168,095,461,025 | -0.368 | -0.415 | -0.553 | 0.553 | 0.353 | 0.353 | -0.368 | 0.946 | 0.930 | 1900/21 | 884 (0.0%) | 3682994 |
| C: Uniform-Price FBA 100ms | react 10ms | $145,803,353,843 | -0.596 | -0.686 | -0.861 | 0.861 | 0.661 | 0.575 | -0.520 | 0.895 | 0.950 | 4223/43 | 1650 (0.0%) | 4865568 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $167,880,005,858 | -0.757 | -0.920 | -1.047 | 1.047 | 0.847 | 0.847 | -0.761 | 0.834 | 0.920 | 9280/86 | 2153 (0.0%) | 5787189 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $168,196,355,858 | -0.871 | -0.896 | -1.049 | 1.049 | 0.849 | 0.849 | -0.878 | 0.951 | 0.922 | 8523/79 | 2153 (0.0%) | 5788188 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $168,406,005,858 | -0.881 | -0.902 | -1.051 | 1.051 | 0.851 | 0.851 | -0.888 | 0.961 | 0.924 | 8014/77 | 2153 (0.0%) | 5788339 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | Large Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $164,896,771,106 | $1,790,825,000 | $1,027,425,000 | $167,715,021,106 | $0 | -1.710 | -2.754 | -1.112 | 71.2% | 0.0% |
| B: Batch 200ms (hl) | react 10ms | $164,912,111,025 | $47,500,000 | $3,135,850,000 | $168,095,461,025 | $0 | -0.553 | -2.711 | -0.520 | 71.2% | 50.6% |
| C: Uniform-Price FBA 100ms | react 10ms | $145,803,353,843 | $0 | $0 | $145,803,353,843 | $0 | -0.861 | n/a | n/a | 71.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | $164,883,180,858 | $232,000,000 | $2,764,825,000 | $167,880,005,858 | $0 | -1.053 | -2.702 | -0.570 | 71.2% | 32.5% |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | $164,883,180,858 | $213,075,000 | $3,100,100,000 | $168,196,355,858 | $0 | -1.055 | -2.695 | -0.599 | 71.2% | 32.5% |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | $164,883,180,858 | $200,350,000 | $3,322,475,000 | $168,406,005,858 | $0 | -1.058 | -2.693 | -0.629 | 71.2% | 32.5% |

Reconciliation note: `StaleSnipe` represents orders hitting the exact un-updated quote ID targeted at trigger time; `LateSnipe` represents sniper orders arriving after MM already requoted. In Regime C (Uniform-Price FBA), taker and sniper orders net internally into a single imbalance before clearing against the MM, so sniper volumes are absorbed into `Taker Vol` (net imbalance). `Cancel-Batch Large %` shows the share of large orders arriving inside a batch that also contains an MM cancel (the exposure base for `leak_ppm` in Regime D).

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 600s | $167,747,623,501 | -0.036 | -0.037 | -0.055 | 0.055 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 71633/97 | 263 (0.0%) | 0 |
| B: Batch 200ms (hl) | react 10ms | PLACEBO shift 600s | $168,115,570,220 | -0.011 | -0.015 | -0.032 | 0.032 | <0 | 0.000 | -0.004 | 0.000 | -0.004 | 1900/21 | 168 (0.0%) | 3752717 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 600s | $146,054,569,001 | -0.012 | -0.015 | -0.036 | 0.036 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 4223/43 | 185 (0.0%) | 4990907 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 600s | $167,929,706,597 | -0.013 | -0.015 | -0.034 | 0.034 | <0 | 0.000 | -0.004 | 0.000 | -0.004 | 9280/86 | 201 (0.0%) | 5959949 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 600s | $168,246,056,597 | -0.014 | -0.016 | -0.034 | 0.034 | <0 | 0.000 | -0.004 | 0.000 | -0.004 | 8523/79 | 201 (0.0%) | 5970122 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 600s | $168,455,706,597 | -0.017 | -0.019 | -0.036 | 0.036 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 8014/77 | 201 (0.0%) | 5977488 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 3600s | $167,750,058,106 | -0.039 | -0.039 | -0.046 | 0.046 | <0 | 0.000 | -0.005 | 0.000 | -0.004 | 71633/97 | 224 (0.0%) | 0 |
| B: Batch 200ms (hl) | react 10ms | PLACEBO shift 3600s | $168,112,398,248 | -0.011 | -0.014 | -0.019 | 0.019 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 1900/21 | 196 (0.0%) | 3759995 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 3600s | $146,118,795,594 | -0.014 | -0.015 | -0.023 | 0.023 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 4223/43 | 214 (0.0%) | 4999295 |
| D: Dywe IBTE 50ms (dec 0ms, hl) | react 10ms | PLACEBO shift 3600s | $167,928,738,609 | -0.015 | -0.016 | -0.023 | 0.023 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 9280/86 | 240 (0.0%) | 5967992 |
| D: Dywe IBTE 50ms (dec 35ms, hl) | react 10ms | PLACEBO shift 3600s | $168,245,088,609 | -0.016 | -0.018 | -0.024 | 0.024 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 8523/79 | 240 (0.0%) | 5978752 |
| D: Dywe IBTE 50ms (dec 70ms, hl) | react 10ms | PLACEBO shift 3600s | $168,454,738,609 | -0.017 | -0.020 | -0.025 | 0.025 | <0 | 0.000 | -0.006 | 0.001 | -0.004 | 8014/77 | 240 (0.0%) | 5986452 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.717 bp, B -0.553 bp, cancel-priority gap (B minus A) 1.164 bp.
- Placebo shift 600 s: A -0.055 bp, B -0.032 bp, gap 0.022 bp = 1.9% of the real gap.
- Placebo shift 3600 s: A -0.046 bp, B -0.019 bp, gap 0.026 bp = 2.2% of the real gap.
- Reading: what survives the placebo does not depend on the flow predicting prices. What disappears depends on the temporal alignment of real taker flow (and, with a trade-derived midpoint proxy, partly on construction).

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 200ms, handicap/window 4%, gap 1.164 bp = 67.7% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.717 bps, 95% CI [-1.871, -1.550]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × leak`), not measurements. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 200ms (hl), D 50ms (dec 0ms, hl) | react 10ms | pending=drop | mid=bidask | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | -0.009 ~ | 0.001 ~ | 0.013 B<D | 0.036 B<D | 0.059 B<D |
| 10% | -0.009 ~ | -0.033 D<B | -0.057 D<B | -0.104 D<B | -0.152 D<B |
| 25% | -0.009 ~ | -0.086 D<B | -0.163 D<B | -0.316 D<B | -0.469 D<B |
| 50% | -0.009 ~ | -0.174 D<B | -0.339 D<B | -0.669 D<B | -0.998 D<B |
| 75% | -0.009 ~ | -0.262 D<B | -0.515 D<B | -1.021 D<B | -1.527 D<B |
| 100% | -0.009 ~ | -0.351 D<B | -0.692 D<B | -1.374 D<B | -2.056 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 200ms (hl), D 50ms (dec 35ms, hl) | react 10ms | pending=drop | mid=bidask | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | -0.008 ~ | 0.003 ~ | 0.014 B<D | 0.037 B<D | 0.060 B<D |
| 10% | -0.008 ~ | -0.032 D<B | -0.055 D<B | -0.103 D<B | -0.151 D<B |
| 25% | -0.008 ~ | -0.085 D<B | -0.161 D<B | -0.315 D<B | -0.468 D<B |
| 50% | -0.008 ~ | -0.173 D<B | -0.338 D<B | -0.667 D<B | -0.997 D<B |
| 75% | -0.008 ~ | -0.261 D<B | -0.514 D<B | -1.020 D<B | -1.526 D<B |
| 100% | -0.008 ~ | -0.349 D<B | -0.690 D<B | -1.373 D<B | -2.055 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 200ms (hl), D 50ms (dec 70ms, hl) | react 10ms | pending=drop | mid=bidask | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | -0.006 ~ | 0.005 ~ | 0.016 B<D | 0.039 B<D | 0.062 B<D |
| 10% | -0.006 ~ | -0.029 D<B | -0.053 D<B | -0.101 D<B | -0.148 D<B |
| 25% | -0.006 ~ | -0.082 D<B | -0.159 D<B | -0.312 D<B | -0.466 D<B |
| 50% | -0.006 ~ | -0.170 D<B | -0.335 D<B | -0.665 D<B | -0.995 D<B |
| 75% | -0.006 ~ | -0.259 D<B | -0.512 D<B | -1.018 D<B | -1.524 D<B |
| 100% | -0.006 ~ | -0.347 D<B | -0.688 D<B | -1.370 D<B | -2.053 D<B |

## Calibration context (public documentation; NOT simulation inputs)
- **Hyperliquid: intra-batch priority order:** Official docs (hyperliquid-l1/order-book) state that within each consensus batch, non-GTC/IOC actions (i.e. post-only / ALO) are processed first, followed by cancels, followed by GTC/IOC orders. Thus new MM post-only quotes land before cancels and takers without a dead gap. Model this with `--b-batch-order hl`. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-l1/order-book_
- **Hyperliquid: block interval & buckets:** Official docs (for-developers/api/priority-fees) state that IOC orders are sorted in 70 ms unix-time buckets, typically corresponding to one block (with 1-2 consensus batches per block, meaning effective batch windows may be shorter than 70 ms). End-to-end co-located latency is cited around ~0.2 s median in third-party mirrors. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/priority-fees ; https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview_
- **dYdX Chain (v4): block time:** The official onboarding FAQ states an average block time of about 1 second, and that short-term order cancels are best effort until the block is final. Use `--b-window-ms 1000` to evaluate a 1-second batching cadence. _Source: https://docs.dydx.xyz/concepts/onboarding-faqs_
- **Not documented (stay assumptions):** No venue documentation specifies MM reaction time, market-data feed latency, client-to-sequencer network transit, or latency-arbitrageur reaction time. Defaults (feed 5 ms, MM reaction 10 ms, sniper reaction 2 ms, network 30 ms) are parameterized assumptions; sweep `--mm-react` rather than trusting a single point. _Source: n/a_
- **Dywe window & post-decrypt matching:** The 50 ms micro-batch and 35 ms / 70 ms decryption RTTs are Dywe design parameters, not live multi-region Devnet measurements. Because threshold decryption completes in matching-engine memory BEFORE the first fill at `clear_ns`, Dywe can execute `hl` post-only-first ordering (`--d-batch-order hl`) without leaking ALO/IOC flags pre-commit. Use `--d-batch-order cancel-first` to stress-test strict pre-decrypt cancel-only sorting. _Source: n/a_
- **Stylized model disclaimer:** Regimes B (70 ms / 200 ms / 1000 ms) are stylized batching models, not byte-for-byte reproductions of Hyperliquid or dYdX matching engines. _Source: n/a_

## Methodological Boundaries & Scope
- Regime D timeline: orders arrive inside a batch window, commit at `batch_close`, complete threshold decryption at `clear_ns = batch_close + decrypt_ms`, and clear in matching-engine memory using `--d-batch-order hl`.
- Regime B uses `--b-batch-order hl` (`hl` = New/post-only -> Cancel -> Taker/Sniper; `cancel-first` = Cancel -> New/Taker/Sniper).
- Single-denominator All-In Net: `--wait-cohort all` (`all` uses total executed taker volume `tk_n` across all 4 components; `large` isolates waiting drift of large orders >= $50,000).
- Under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net; inspect `Dropped Takers (USD%)` and selection-bias warnings before comparing costs.
- 95% CI reflects drift variance across multi-hour blocks under FIXED scenario assumptions; parameter uncertainty (capture, impact, leak, reaction times, windows) is not covered.
- Single 7-day sample window; BTCUSDT and ETHUSDT are strongly correlated and resampled jointly.
- Taker flow is exogenous and invariant to spread width; one representative MM clears the full flow; sniper triggers are threshold-bound.
