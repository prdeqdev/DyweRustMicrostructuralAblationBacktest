# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.1)
Window: 2026-11-10 + 7d | Pairs: BTCUSDT,ETHUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 18 | B window 1000ms, D window 50ms, FBA window 100ms | pending=drop | mid=bidask | placebo shifts (s): [600, 3600]

Pair-Days Processed: 14 of 14.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $100,159,355,210 | -1.172 | -1.485 | -1.613 | 1.613 | 1.413 | 1.413 | -1.164 | 0.771 | 1.020 | 25188/69 | 639 | 0 |
| B: Cancel-Priority 1000ms | react 10ms | $49,933,348,372 | -0.320 | -0.357 | -0.454 | 0.454 | 0.254 | 0.254 | -0.317 | 0.403 | 0.340 | 99/27924 | 1684775 | 582991 |
| C: Uniform-Price FBA 100ms | react 10ms | $88,088,992,901 | -0.532 | -0.651 | -0.815 | 0.815 | 0.615 | 0.541 | -0.467 | 1.016 | 1.089 | 44219/0 | 0 | 2407848 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $78,771,007,554 | -0.684 | -0.872 | -1.043 | 1.043 | 0.843 | 0.843 | -0.682 | 0.571 | 0.732 | 3178/22706 | 469123 | 2823576 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $78,897,357,554 | -0.784 | -0.870 | -1.046 | 1.046 | 0.846 | 0.846 | -0.783 | 0.693 | 0.756 | 2958/22211 | 469123 | 2823920 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $78,978,832,554 | -0.832 | -0.889 | -1.045 | 1.045 | 0.845 | 0.845 | -0.832 | 0.750 | 0.763 | 2815/22094 | 469123 | 2823992 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $67,549,672,554 | -1.431 | -1.702 | -1.867 | 1.867 | 1.667 | 1.667 | -1.415 | 0.987 | 1.239 | 46274/102 | 1131 | 0 |
| B: Cancel-Priority 1000ms | react 10ms | $27,940,613,812 | -0.313 | -0.361 | -0.467 | 0.467 | 0.267 | 0.267 | -0.303 | 0.411 | 0.375 | 152/48065 | 2401297 | 581223 |
| C: Uniform-Price FBA 100ms | react 10ms | $57,753,916,198 | -0.694 | -0.736 | -0.931 | 0.931 | 0.731 | 0.624 | -0.599 | 1.287 | 1.312 | 78709/0 | 0 | 2457678 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $49,189,127,056 | -0.795 | -0.984 | -1.170 | 1.170 | 0.970 | 0.970 | -0.791 | 0.692 | 0.872 | 6104/42151 | 685203 | 2963483 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $49,417,927,056 | -0.919 | -0.991 | -1.169 | 1.169 | 0.969 | 0.969 | -0.919 | 0.860 | 0.910 | 5565/41088 | 685203 | 2964138 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $49,563,227,056 | -0.955 | -1.004 | -1.168 | 1.168 | 0.968 | 0.968 | -0.957 | 0.896 | 0.908 | 5199/40526 | 685203 | 2964217 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $167,709,027,764 | -1.276 | -1.572 | -1.715 | 1.715 | 1.515 | 1.515 | -1.264 | 0.849 | 1.100 | 71462/171 | 1770 | 0 |
| B: Cancel-Priority 1000ms | react 10ms | $77,873,962,184 | -0.318 | -0.359 | -0.459 | 0.459 | 0.259 | 0.259 | -0.312 | 0.405 | 0.352 | 251/75989 | 4086072 | 1164214 |
| C: Uniform-Price FBA 100ms | react 10ms | $145,842,909,099 | -0.596 | -0.685 | -0.861 | 0.861 | 0.661 | 0.575 | -0.520 | 1.114 | 1.168 | 122928/0 | 0 | 4865526 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $127,960,134,610 | -0.726 | -0.915 | -1.092 | 1.092 | 0.892 | 0.892 | -0.723 | 0.612 | 0.780 | 9282/64857 | 1154326 | 5787059 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $128,315,284,610 | -0.836 | -0.917 | -1.093 | 1.093 | 0.893 | 0.893 | -0.835 | 0.749 | 0.807 | 8523/63299 | 1154326 | 5788058 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $128,542,059,610 | -0.880 | -0.934 | -1.092 | 1.092 | 0.892 | 0.892 | -0.879 | 0.799 | 0.812 | 8014/62620 | 1154326 | 5788209 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $164,892,552,764 | $1,786,550,000 | $1,029,925,000 | $167,709,027,764 | $0 | -1.708 | -2.751 | -1.109 |
| B: Cancel-Priority 1000ms | react 10ms | $76,514,437,184 | $6,275,000 | $1,353,250,000 | $77,873,962,184 | $0 | -0.452 | -2.760 | -0.810 |
| C: Uniform-Price FBA 100ms | react 10ms | $145,842,909,099 | $0 | $0 | $145,842,909,099 | $0 | -0.861 | n/a | n/a |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $126,582,334,610 | $232,050,000 | $1,145,750,000 | $127,960,134,610 | $0 | -1.089 | -2.702 | -1.056 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $126,582,334,610 | $213,075,000 | $1,519,875,000 | $128,315,284,610 | $0 | -1.091 | -2.695 | -1.014 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $126,582,334,610 | $200,350,000 | $1,759,375,000 | $128,542,059,610 | $0 | -1.091 | -2.693 | -1.010 |

## Placebo test: circularly shifted taker flow

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 600s | $167,745,891,480 | -0.036 | -0.037 | -0.055 | 0.055 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 71462/171 | 285 | 0 |
| B: Cancel-Priority 1000ms | react 10ms | PLACEBO shift 600s | $139,033,097,944 | -0.009 | -0.012 | -0.009 | 0.009 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 251/75989 | 1864234 | 1168429 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 600s | $146,056,993,645 | -0.012 | -0.015 | -0.036 | 0.036 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 122928/0 | 0 | 4990854 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | PLACEBO shift 600s | $164,514,490,207 | -0.012 | -0.013 | -0.030 | 0.030 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 9282/64857 | 125273 | 5959770 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | PLACEBO shift 600s | $164,869,640,207 | -0.013 | -0.015 | -0.031 | 0.031 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 8523/63299 | 125273 | 5969950 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | PLACEBO shift 600s | $165,096,415,207 | -0.016 | -0.017 | -0.033 | 0.033 | <0 | 0.000 | -0.005 | 0.002 | -0.003 | 8014/62620 | 125273 | 5977320 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 3600s | $167,746,729,058 | -0.038 | -0.039 | -0.046 | 0.046 | <0 | 0.000 | -0.005 | 0.000 | -0.004 | 71462/171 | 280 | 0 |
| B: Cancel-Priority 1000ms | react 10ms | PLACEBO shift 3600s | $142,782,224,503 | -0.007 | -0.009 | -0.015 | 0.015 | <0 | 0.000 | -0.001 | -0.002 | -0.003 | 251/75989 | 1660091 | 1169112 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 3600s | $146,121,415,305 | -0.014 | -0.015 | -0.023 | 0.023 | <0 | 0.000 | -0.004 | 0.002 | -0.002 | 122928/0 | 0 | 4999244 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | PLACEBO shift 3600s | $164,794,856,638 | -0.013 | -0.015 | -0.020 | 0.020 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 9282/64857 | 110405 | 5967809 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | PLACEBO shift 3600s | $165,150,006,638 | -0.015 | -0.017 | -0.022 | 0.022 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 8523/63299 | 110405 | 5978577 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | PLACEBO shift 3600s | $165,376,781,638 | -0.016 | -0.018 | -0.024 | 0.024 | <0 | 0.000 | -0.006 | 0.002 | -0.004 | 8014/62620 | 110405 | 5986284 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.715 bp, B -0.459 bp, cancel-priority gap (B minus A) 1.256 bp.
- Placebo shift 600 s: A -0.055 bp, B -0.009 bp, gap 0.046 bp = 3.6% of the real gap.
- Placebo shift 3600 s: A -0.046 bp, B -0.015 bp, gap 0.030 bp = 2.4% of the real gap.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 1000ms, handicap/window 0%, gap 1.256 bp = 73.2% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.715 bps, 95% CI [-1.868, -1.549]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window 1000ms, D window 50ms, decrypt 0ms | react 10ms | pending=drop | mid=bidask
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.427 B<D | 0.430 B<D | 0.432 B<D | 0.436 B<D | 0.441 B<D |
| 10% | 0.427 B<D | 0.399 B<D | 0.370 B<D | 0.313 B<D | 0.255 B<D |
| 25% | 0.427 B<D | 0.352 B<D | 0.277 B<D | 0.127 B<D | -0.023 ~ |
| 50% | 0.427 B<D | 0.275 B<D | 0.122 B<D | -0.182 D<B | -0.487 D<B |
| 75% | 0.427 B<D | 0.197 B<D | -0.032 ~ | -0.492 D<B | -0.952 D<B |
| 100% | 0.427 B<D | 0.120 B<D | -0.187 D<B | -0.802 D<B | -1.416 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window 1000ms, D window 50ms, decrypt 35ms | react 10ms | pending=drop | mid=bidask
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.455 B<D | 0.457 B<D | 0.459 B<D | 0.464 B<D | 0.468 B<D |
| 10% | 0.455 B<D | 0.426 B<D | 0.397 B<D | 0.340 B<D | 0.282 B<D |
| 25% | 0.455 B<D | 0.379 B<D | 0.304 B<D | 0.154 B<D | 0.004 ~ |
| 50% | 0.455 B<D | 0.302 B<D | 0.149 B<D | -0.155 D<B | -0.460 D<B |
| 75% | 0.455 B<D | 0.225 B<D | -0.004 ~ | -0.464 D<B | -0.925 D<B |
| 100% | 0.455 B<D | 0.147 B<D | -0.159 D<B | -0.774 D<B | -1.389 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window 1000ms, D window 50ms, decrypt 70ms | react 10ms | pending=drop | mid=bidask
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.459 B<D | 0.461 B<D | 0.464 B<D | 0.468 B<D | 0.473 B<D |
| 10% | 0.459 B<D | 0.430 B<D | 0.402 B<D | 0.344 B<D | 0.287 B<D |
| 25% | 0.459 B<D | 0.384 B<D | 0.309 B<D | 0.158 B<D | 0.008 ~ |
| 50% | 0.459 B<D | 0.306 B<D | 0.154 B<D | -0.150 D<B | -0.456 D<B |
| 75% | 0.459 B<D | 0.229 B<D | 0.000 ~ | -0.460 D<B | -0.920 D<B |
| 100% | 0.459 B<D | 0.152 B<D | -0.155 D<B | -0.770 D<B | -1.385 D<B |

## Calibration context (public documentation, retrieved 2026-11-18; NOT simulation inputs)
- **Hyperliquid: end-to-end latency:** The docs define end-to-end latency as the time from sending a request to receiving the committed response. A third-party mirror that cites the official docs reports a median of about 0.2 s and a 99th percentile of about 0.9 s for a co-located client (an older analyst note quotes a 0.3 s median). The numeric values were not visible in the official page excerpt retrieved here. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview ; https://www.dwellir.com/docs/hyperliquid/architecture_
- **Hyperliquid: block interval:** Not stated in the official page retrieved. A third-party node-docs site says blocks come about every 70 ms with finality in about two rounds (around 140 ms); an analyst post describes block commits in the range of 200-300 ms and an observed lag of about 700 ms in fills versus Binance (maker update in one block, taker fill in the next). Treat the block interval as roughly 70-300 ms and UNVERIFIED. _Source: https://hypernode-docs.polynode.dev/research/consensus ; https://arrakis.finance/blog/crypto-price-discovery_
- **dYdX Chain (v4): block time:** The official onboarding FAQ states an average block time of about 1 second, and that short-term order cancels are best effort until the block is final. Some third-party pages claim 0.5 s or even 100 ms blocks; they conflict with the official FAQ and are not used here. _Source: https://docs.dydx.xyz/concepts/onboarding-faqs_
- **Not documented (stay assumptions):** No venue documentation was found for market-maker reaction time, market-data feed latency, client-to-sequencer network latency or latency-arbitrageur reaction time. The model defaults (feed 5 ms, MM reaction 10 ms, sniper reaction 2 ms, network 30 ms) are assumptions. Sweep them (--mm-react) instead of trusting a single value. _Source: n/a_
- **Dywe window:** The 50 ms micro-batch and the 35 ms decryption RTT are Dywe design/benchmark parameters, not venue documentation. They are not validated by any source above. _Source: n/a_
- **Implication for Regime B:** If B is meant to represent Hyperliquid, a 50 ms window is shorter than every documented latency figure above. Use --b-window-ms 70 and --b-window-ms 200 (separate runs) to read the cancel-priority gap at those scales, and --b-window-ms 1000 for a dYdX-like cadence. _Source: n/a_
