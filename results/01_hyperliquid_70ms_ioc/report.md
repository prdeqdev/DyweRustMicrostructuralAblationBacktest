# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.1)
Window: 2026-11-10 + 7d | Pairs: BTCUSDT,ETHUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 18 | B window 70ms, D window 50ms, FBA window 100ms | pending=drop | mid=bidask | placebo shifts (s): [600, 3600]

Pair-Days Processed: 14 of 14.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $100,159,355,210 | -1.172 | -1.485 | -1.613 | 1.613 | 1.413 | 1.413 | -1.164 | 0.771 | 1.020 | 25188/69 | 639 | 0 |
| B: Cancel-Priority 70ms | react 10ms | $74,620,802,840 | -0.605 | -0.756 | -0.937 | 0.937 | 0.737 | 0.737 | -0.603 | 0.532 | 0.666 | 2237/23911 | 572877 | 2629227 |
| C: Uniform-Price FBA 100ms | react 10ms | $88,088,992,901 | -0.532 | -0.651 | -0.815 | 0.815 | 0.615 | 0.541 | -0.467 | 1.016 | 1.089 | 44219/0 | 0 | 2407848 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $78,771,007,554 | -0.684 | -0.872 | -1.043 | 1.043 | 0.843 | 0.843 | -0.682 | 0.571 | 0.732 | 3178/22706 | 469123 | 2823576 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $78,897,357,554 | -0.784 | -0.870 | -1.046 | 1.046 | 0.846 | 0.846 | -0.783 | 0.693 | 0.756 | 2958/22211 | 469123 | 2823920 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $78,978,832,554 | -0.832 | -0.889 | -1.045 | 1.045 | 0.845 | 0.845 | -0.832 | 0.750 | 0.763 | 2815/22094 | 469123 | 2823992 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $67,549,672,554 | -1.431 | -1.702 | -1.867 | 1.867 | 1.667 | 1.667 | -1.415 | 0.987 | 1.239 | 46274/102 | 1131 | 0 |
| B: Cancel-Priority 70ms | react 10ms | $46,241,187,868 | -0.692 | -0.850 | -1.059 | 1.059 | 0.859 | 0.859 | -0.688 | 0.635 | 0.806 | 4071/44634 | 829887 | 2722957 |
| C: Uniform-Price FBA 100ms | react 10ms | $57,753,916,198 | -0.694 | -0.736 | -0.931 | 0.931 | 0.731 | 0.624 | -0.599 | 1.287 | 1.312 | 78709/0 | 0 | 2457678 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $49,189,127,056 | -0.795 | -0.984 | -1.170 | 1.170 | 0.970 | 0.970 | -0.791 | 0.692 | 0.872 | 6104/42151 | 685203 | 2963483 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $49,417,927,056 | -0.919 | -0.991 | -1.169 | 1.169 | 0.969 | 0.969 | -0.919 | 0.860 | 0.910 | 5565/41088 | 685203 | 2964138 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $49,563,227,056 | -0.955 | -1.004 | -1.168 | 1.168 | 0.968 | 0.968 | -0.957 | 0.896 | 0.908 | 5199/40526 | 685203 | 2964217 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $167,709,027,764 | -1.276 | -1.572 | -1.715 | 1.715 | 1.515 | 1.515 | -1.264 | 0.849 | 1.100 | 71462/171 | 1770 | 0 |
| B: Cancel-Priority 70ms | react 10ms | $120,861,990,708 | -0.638 | -0.792 | -0.983 | 0.983 | 0.783 | 0.783 | -0.635 | 0.566 | 0.714 | 6308/68545 | 1402764 | 5352184 |
| C: Uniform-Price FBA 100ms | react 10ms | $145,842,909,099 | -0.596 | -0.685 | -0.861 | 0.861 | 0.661 | 0.575 | -0.520 | 1.114 | 1.168 | 122928/0 | 0 | 4865526 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $127,960,134,610 | -0.726 | -0.915 | -1.092 | 1.092 | 0.892 | 0.892 | -0.723 | 0.612 | 0.780 | 9282/64857 | 1154326 | 5787059 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $128,315,284,610 | -0.836 | -0.917 | -1.093 | 1.093 | 0.893 | 0.893 | -0.835 | 0.749 | 0.807 | 8523/63299 | 1154326 | 5788058 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $128,542,059,610 | -0.880 | -0.934 | -1.092 | 1.092 | 0.892 | 0.892 | -0.879 | 0.799 | 0.812 | 8014/62620 | 1154326 | 5788209 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $164,892,552,764 | $1,786,550,000 | $1,029,925,000 | $167,709,027,764 | $0 | -1.708 | -2.751 | -1.109 |
| B: Cancel-Priority 70ms | react 10ms | $119,542,140,708 | $157,700,000 | $1,162,150,000 | $120,861,990,708 | $0 | -0.981 | -2.685 | -1.026 |
| C: Uniform-Price FBA 100ms | react 10ms | $145,842,909,099 | $0 | $0 | $145,842,909,099 | $0 | -0.861 | n/a | n/a |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $126,582,334,610 | $232,050,000 | $1,145,750,000 | $127,960,134,610 | $0 | -1.089 | -2.702 | -1.056 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $126,582,334,610 | $213,075,000 | $1,519,875,000 | $128,315,284,610 | $0 | -1.091 | -2.695 | -1.014 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $126,582,334,610 | $200,350,000 | $1,759,375,000 | $128,542,059,610 | $0 | -1.091 | -2.693 | -1.010 |

## Placebo test: circularly shifted taker flow

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 600s | $167,745,891,480 | -0.036 | -0.037 | -0.055 | 0.055 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 71462/171 | 285 | 0 |
| B: Cancel-Priority 70ms | react 10ms | PLACEBO shift 600s | $163,797,098,347 | -0.010 | -0.012 | -0.027 | 0.027 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 6308/68545 | 173658 | 5504187 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 600s | $146,056,993,645 | -0.012 | -0.015 | -0.036 | 0.036 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 122928/0 | 0 | 4990854 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | PLACEBO shift 600s | $164,514,490,207 | -0.012 | -0.013 | -0.030 | 0.030 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 9282/64857 | 125273 | 5959770 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | PLACEBO shift 600s | $164,869,640,207 | -0.013 | -0.015 | -0.031 | 0.031 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 8523/63299 | 125273 | 5969950 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | PLACEBO shift 600s | $165,096,415,207 | -0.016 | -0.017 | -0.033 | 0.033 | <0 | 0.000 | -0.005 | 0.002 | -0.003 | 8014/62620 | 125273 | 5977320 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 3600s | $167,746,729,058 | -0.038 | -0.039 | -0.046 | 0.046 | <0 | 0.000 | -0.005 | 0.000 | -0.004 | 71462/171 | 280 | 0 |
| B: Cancel-Priority 70ms | react 10ms | PLACEBO shift 3600s | $164,209,451,260 | -0.013 | -0.014 | -0.020 | 0.020 | <0 | 0.000 | -0.006 | 0.001 | -0.004 | 6308/68545 | 153191 | 5512948 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 3600s | $146,121,415,305 | -0.014 | -0.015 | -0.023 | 0.023 | <0 | 0.000 | -0.004 | 0.002 | -0.002 | 122928/0 | 0 | 4999244 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | PLACEBO shift 3600s | $164,794,856,638 | -0.013 | -0.015 | -0.020 | 0.020 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 9282/64857 | 110405 | 5967809 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | PLACEBO shift 3600s | $165,150,006,638 | -0.015 | -0.017 | -0.022 | 0.022 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 8523/63299 | 110405 | 5978577 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | PLACEBO shift 3600s | $165,376,781,638 | -0.016 | -0.018 | -0.024 | 0.024 | <0 | 0.000 | -0.006 | 0.002 | -0.004 | 8014/62620 | 110405 | 5986284 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.715 bp, B -0.983 bp, cancel-priority gap (B minus A) 0.731 bp.
- Placebo shift 600 s: A -0.055 bp, B -0.027 bp, gap 0.027 bp = 3.7% of the real gap.
- Placebo shift 3600 s: A -0.046 bp, B -0.020 bp, gap 0.025 bp = 3.4% of the real gap.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.731 bp = 42.6% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.715 bps, 95% CI [-1.868, -1.549]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window 70ms, D window 50ms, decrypt 0ms | react 10ms | pending=drop | mid=bidask
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.065 B<D | 0.067 B<D | 0.070 B<D | 0.074 B<D | 0.079 B<D |
| 10% | 0.065 B<D | 0.035 ~ | 0.004 ~ | -0.056 D<B | -0.118 D<B |
| 25% | 0.065 B<D | -0.014 ~ | -0.094 D<B | -0.254 D<B | -0.414 D<B |
| 50% | 0.065 B<D | -0.096 D<B | -0.258 D<B | -0.583 D<B | -0.907 D<B |
| 75% | 0.065 B<D | -0.178 D<B | -0.423 D<B | -0.912 D<B | -1.401 D<B |
| 100% | 0.065 B<D | -0.260 D<B | -0.587 D<B | -1.241 D<B | -1.894 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window 70ms, D window 50ms, decrypt 35ms | react 10ms | pending=drop | mid=bidask
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.093 B<D | 0.095 B<D | 0.097 B<D | 0.102 B<D | 0.106 B<D |
| 10% | 0.093 B<D | 0.062 B<D | 0.031 ~ | -0.029 ~ | -0.090 D<B |
| 25% | 0.093 B<D | 0.013 ~ | -0.066 D<B | -0.226 D<B | -0.386 D<B |
| 50% | 0.093 B<D | -0.069 D<B | -0.231 D<B | -0.555 D<B | -0.880 D<B |
| 75% | 0.093 B<D | -0.151 D<B | -0.395 D<B | -0.884 D<B | -1.373 D<B |
| 100% | 0.093 B<D | -0.233 D<B | -0.560 D<B | -1.213 D<B | -1.867 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window 70ms, D window 50ms, decrypt 70ms | react 10ms | pending=drop | mid=bidask
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.097 B<D | 0.099 B<D | 0.101 B<D | 0.106 B<D | 0.110 B<D |
| 10% | 0.097 B<D | 0.066 B<D | 0.036 ~ | -0.025 ~ | -0.086 D<B |
| 25% | 0.097 B<D | 0.017 ~ | -0.062 D<B | -0.222 D<B | -0.382 D<B |
| 50% | 0.097 B<D | -0.064 D<B | -0.227 D<B | -0.551 D<B | -0.875 D<B |
| 75% | 0.097 B<D | -0.147 D<B | -0.391 D<B | -0.880 D<B | -1.369 D<B |
| 100% | 0.097 B<D | -0.229 D<B | -0.556 D<B | -1.209 D<B | -1.862 D<B |

## Calibration context (public documentation, retrieved 2026-11-18; NOT simulation inputs)
- **Hyperliquid: end-to-end latency:** The docs define end-to-end latency as the time from sending a request to receiving the committed response. A third-party mirror that cites the official docs reports a median of about 0.2 s and a 99th percentile of about 0.9 s for a co-located client (an older analyst note quotes a 0.3 s median). The numeric values were not visible in the official page excerpt retrieved here. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview ; https://www.dwellir.com/docs/hyperliquid/architecture_
- **Hyperliquid: block interval:** Not stated in the official page retrieved. A third-party node-docs site says blocks come about every 70 ms with finality in about two rounds (around 140 ms); an analyst post describes block commits in the range of 200-300 ms and an observed lag of about 700 ms in fills versus Binance (maker update in one block, taker fill in the next). Treat the block interval as roughly 70-300 ms and UNVERIFIED. _Source: https://hypernode-docs.polynode.dev/research/consensus ; https://arrakis.finance/blog/crypto-price-discovery_
- **dYdX Chain (v4): block time:** The official onboarding FAQ states an average block time of about 1 second, and that short-term order cancels are best effort until the block is final. Some third-party pages claim 0.5 s or even 100 ms blocks; they conflict with the official FAQ and are not used here. _Source: https://docs.dydx.xyz/concepts/onboarding-faqs_
- **Not documented (stay assumptions):** No venue documentation was found for market-maker reaction time, market-data feed latency, client-to-sequencer network latency or latency-arbitrageur reaction time. The model defaults (feed 5 ms, MM reaction 10 ms, sniper reaction 2 ms, network 30 ms) are assumptions. Sweep them (--mm-react) instead of trusting a single value. _Source: n/a_
- **Dywe window:** The 50 ms micro-batch and the 35 ms decryption RTT are Dywe design/benchmark parameters, not venue documentation. They are not validated by any source above. _Source: n/a_
- **Implication for Regime B:** If B is meant to represent Hyperliquid, a 50 ms window is shorter than every documented latency figure above. Use --b-window-ms 70 and --b-window-ms 200 (separate runs) to read the cancel-priority gap at those scales, and --b-window-ms 1000 for a dYdX-like cadence. _Source: n/a_
