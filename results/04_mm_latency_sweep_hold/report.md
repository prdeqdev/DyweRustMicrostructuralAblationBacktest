# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.1)
Window: 2026-11-10 + 7d | Pairs: BTCUSDT,ETHUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 30 | B window 70ms, D window 50ms, FBA window 100ms | pending=hold | mid=bidask | placebo shifts (s): [600, 3600]

Pair-Days Processed: 14 of 14.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $100,179,290,567 | -1.172 | -1.485 | -1.613 | 1.613 | 1.413 | 1.413 | -1.164 | 0.772 | 1.020 | 25188/69 | 0 | 0 |
| B: Cancel-Priority 70ms | react 10ms | $99,675,315,567 | -0.621 | -0.722 | -0.877 | 0.877 | 0.677 | 0.677 | -0.620 | 1.016 | 1.074 | 2237/23911 | 0 | 2629227 |
| C: Uniform-Price FBA 100ms | react 10ms | $88,088,992,901 | -0.532 | -0.651 | -0.815 | 0.815 | 0.615 | 0.541 | -0.467 | 1.016 | 1.089 | 44219/0 | 0 | 2407848 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $99,687,965,567 | -0.716 | -0.920 | -1.017 | 1.017 | 0.817 | 0.817 | -0.714 | 0.952 | 1.054 | 3178/22706 | 0 | 2823576 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $99,814,315,567 | -0.822 | -0.883 | -1.022 | 1.022 | 0.822 | 0.822 | -0.821 | 1.078 | 1.079 | 2958/22211 | 0 | 2823920 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $99,895,790,567 | -0.849 | -0.893 | -1.020 | 1.020 | 0.820 | 0.820 | -0.848 | 1.106 | 1.077 | 2815/22094 | 0 | 2823992 |
| A: Continuous FIFO | react 25ms | $100,209,840,567 | -1.219 | -1.532 | -1.660 | 1.660 | 1.460 | 1.460 | -1.209 | 0.772 | 1.023 | 32237/43 | 0 | 0 |
| B: Cancel-Priority 70ms | react 25ms | $99,712,315,567 | -0.811 | -0.913 | -1.067 | 1.067 | 0.867 | 0.867 | -0.808 | 1.016 | 1.075 | 7976/23421 | 0 | 2638735 |
| C: Uniform-Price FBA 100ms | react 25ms | $88,099,321,389 | -0.672 | -0.792 | -0.956 | 0.956 | 0.756 | 0.664 | -0.590 | 1.016 | 1.089 | 45116/0 | 0 | 2412911 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 25ms | $99,777,915,567 | -0.944 | -1.148 | -1.245 | 1.245 | 1.045 | 1.045 | -0.941 | 0.952 | 1.056 | 11217/20215 | 0 | 2839121 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 25ms | $99,883,515,567 | -1.050 | -1.111 | -1.250 | 1.250 | 1.050 | 1.050 | -1.047 | 1.078 | 1.080 | 10565/19733 | 0 | 2839418 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 25ms | $99,956,765,567 | -1.076 | -1.120 | -1.247 | 1.247 | 1.047 | 1.047 | -1.075 | 1.106 | 1.078 | 10071/19521 | 0 | 2839469 |
| A: Continuous FIFO | react 40ms | $100,231,940,567 | -1.253 | -1.566 | -1.694 | 1.694 | 1.494 | 1.494 | -1.242 | 0.772 | 1.024 | 35991/21 | 0 | 0 |
| B: Cancel-Priority 70ms | react 40ms | $99,812,840,567 | -0.926 | -1.027 | -1.181 | 1.181 | 0.981 | 0.981 | -0.920 | 1.016 | 1.077 | 14525/20191 | 0 | 2648645 |
| C: Uniform-Price FBA 100ms | react 40ms | $88,110,525,357 | -0.865 | -0.985 | -1.149 | 1.149 | 0.949 | 0.834 | -0.760 | 1.016 | 1.090 | 45886/0 | 0 | 2418104 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 40ms | $99,947,440,567 | -1.183 | -1.386 | -1.483 | 1.483 | 1.283 | 1.283 | -1.176 | 0.952 | 1.058 | 20749/14131 | 0 | 2855232 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 40ms | $100,037,290,567 | -1.288 | -1.348 | -1.487 | 1.487 | 1.287 | 1.287 | -1.283 | 1.078 | 1.082 | 19689/13907 | 0 | 2855525 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 40ms | $100,099,690,567 | -1.314 | -1.358 | -1.484 | 1.484 | 1.284 | 1.284 | -1.310 | 1.106 | 1.080 | 18827/13825 | 0 | 2855560 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $67,571,150,284 | -1.431 | -1.702 | -1.868 | 1.868 | 1.668 | 1.668 | -1.415 | 0.987 | 1.239 | 46274/102 | 0 | 0 |
| B: Cancel-Priority 70ms | react 10ms | $66,578,500,284 | -0.663 | -0.743 | -0.914 | 0.914 | 0.714 | 0.714 | -0.660 | 1.227 | 1.281 | 4071/44634 | 0 | 2722957 |
| C: Uniform-Price FBA 100ms | react 10ms | $57,753,916,198 | -0.694 | -0.736 | -0.931 | 0.931 | 0.731 | 0.624 | -0.599 | 1.287 | 1.312 | 78709/0 | 0 | 2457678 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $66,623,800,284 | -0.833 | -0.939 | -1.112 | 1.112 | 0.912 | 0.912 | -0.831 | 1.201 | 1.282 | 6104/42151 | 0 | 2963483 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $66,852,600,284 | -0.963 | -0.933 | -1.108 | 1.108 | 0.908 | 0.908 | -0.964 | 1.370 | 1.314 | 5565/41088 | 0 | 2964138 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $66,997,900,284 | -0.945 | -0.933 | -1.118 | 1.118 | 0.918 | 0.918 | -0.946 | 1.328 | 1.299 | 5199/40526 | 0 | 2964217 |
| A: Continuous FIFO | react 25ms | $67,625,850,284 | -1.499 | -1.771 | -1.936 | 1.936 | 1.736 | 1.736 | -1.478 | 0.987 | 1.245 | 57987/67 | 0 | 0 |
| B: Cancel-Priority 70ms | react 25ms | $66,687,200,284 | -0.866 | -0.947 | -1.117 | 1.117 | 0.917 | 0.917 | -0.859 | 1.227 | 1.284 | 13965/42333 | 0 | 2737385 |
| C: Uniform-Price FBA 100ms | react 25ms | $57,777,737,008 | -0.896 | -0.938 | -1.132 | 1.132 | 0.932 | 0.796 | -0.771 | 1.287 | 1.312 | 80588/0 | 0 | 2465406 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 25ms | $66,843,350,284 | -1.141 | -1.246 | -1.419 | 1.419 | 1.219 | 1.219 | -1.134 | 1.201 | 1.286 | 20653/35379 | 0 | 2987358 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 25ms | $67,034,275,284 | -1.268 | -1.238 | -1.412 | 1.412 | 1.212 | 1.212 | -1.266 | 1.370 | 1.316 | 18889/34370 | 0 | 2987903 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 25ms | $67,154,850,284 | -1.249 | -1.237 | -1.421 | 1.421 | 1.221 | 1.221 | -1.249 | 1.328 | 1.300 | 17671/34072 | 0 | 2987982 |
| A: Continuous FIFO | react 40ms | $67,641,300,284 | -1.542 | -1.814 | -1.979 | 1.979 | 1.779 | 1.779 | -1.517 | 0.987 | 1.249 | 65204/49 | 0 | 0 |
| B: Cancel-Priority 70ms | react 40ms | $66,872,225,284 | -1.113 | -1.193 | -1.362 | 1.362 | 1.162 | 1.162 | -1.101 | 1.227 | 1.287 | 25686/35939 | 0 | 2752333 |
| C: Uniform-Price FBA 100ms | react 40ms | $57,796,490,816 | -1.047 | -1.089 | -1.283 | 1.283 | 1.083 | 0.925 | -0.897 | 1.287 | 1.315 | 81892/0 | 0 | 2473321 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 40ms | $67,132,325,284 | -1.374 | -1.478 | -1.650 | 1.650 | 1.450 | 1.450 | -1.361 | 1.201 | 1.290 | 36975/24828 | 0 | 3012073 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 40ms | $67,286,950,284 | -1.499 | -1.469 | -1.643 | 1.643 | 1.443 | 1.443 | -1.493 | 1.370 | 1.319 | 34176/24320 | 0 | 3012559 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 40ms | $67,392,325,284 | -1.479 | -1.467 | -1.651 | 1.651 | 1.451 | 1.451 | -1.476 | 1.328 | 1.303 | 32151/24257 | 0 | 3012634 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $167,750,440,851 | -1.276 | -1.573 | -1.715 | 1.715 | 1.515 | 1.515 | -1.264 | 0.849 | 1.100 | 71462/171 | 0 | 0 |
| B: Cancel-Priority 70ms | react 10ms | $166,253,815,851 | -0.638 | -0.731 | -0.892 | 0.892 | 0.692 | 0.692 | -0.636 | 1.092 | 1.148 | 6308/68545 | 0 | 5352184 |
| C: Uniform-Price FBA 100ms | react 10ms | $145,842,909,099 | -0.596 | -0.685 | -0.861 | 0.861 | 0.661 | 0.575 | -0.520 | 1.114 | 1.168 | 122928/0 | 0 | 4865526 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $166,311,765,851 | -0.763 | -0.927 | -1.055 | 1.055 | 0.855 | 0.855 | -0.761 | 1.042 | 1.136 | 9282/64857 | 0 | 5787059 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $166,666,915,851 | -0.878 | -0.903 | -1.057 | 1.057 | 0.857 | 0.857 | -0.878 | 1.183 | 1.162 | 8523/63299 | 0 | 5788058 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $166,893,690,851 | -0.887 | -0.909 | -1.059 | 1.059 | 0.859 | 0.859 | -0.887 | 1.186 | 1.158 | 8014/62620 | 0 | 5788209 |
| A: Continuous FIFO | react 25ms | $167,835,690,851 | -1.332 | -1.628 | -1.771 | 1.771 | 1.571 | 1.571 | -1.316 | 0.849 | 1.104 | 90224/110 | 0 | 0 |
| B: Cancel-Priority 70ms | react 25ms | $166,399,515,851 | -0.833 | -0.926 | -1.087 | 1.087 | 0.887 | 0.887 | -0.829 | 1.092 | 1.151 | 21941/65754 | 0 | 5376120 |
| C: Uniform-Price FBA 100ms | react 25ms | $145,877,058,397 | -0.761 | -0.850 | -1.026 | 1.026 | 0.826 | 0.717 | -0.662 | 1.114 | 1.169 | 125704/0 | 0 | 4878317 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 25ms | $166,621,265,851 | -1.023 | -1.187 | -1.314 | 1.314 | 1.114 | 1.114 | -1.018 | 1.042 | 1.139 | 31870/55594 | 0 | 5826479 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 25ms | $166,917,790,851 | -1.137 | -1.162 | -1.315 | 1.315 | 1.115 | 1.115 | -1.135 | 1.183 | 1.164 | 29454/54103 | 0 | 5827321 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 25ms | $167,111,615,851 | -1.146 | -1.167 | -1.317 | 1.317 | 1.117 | 1.117 | -1.144 | 1.186 | 1.159 | 27742/53593 | 0 | 5827451 |
| A: Continuous FIFO | react 40ms | $167,873,240,851 | -1.369 | -1.666 | -1.809 | 1.809 | 1.609 | 1.609 | -1.351 | 0.849 | 1.107 | 101195/70 | 0 | 0 |
| B: Cancel-Priority 70ms | react 40ms | $166,685,065,851 | -1.001 | -1.093 | -1.254 | 1.254 | 1.054 | 1.054 | -0.992 | 1.092 | 1.154 | 40211/56130 | 0 | 5400978 |
| C: Uniform-Price FBA 100ms | react 40ms | $145,907,016,173 | -0.937 | -1.026 | -1.202 | 1.202 | 1.002 | 0.871 | -0.814 | 1.114 | 1.170 | 127778/0 | 0 | 4891425 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 40ms | $167,079,765,851 | -1.259 | -1.423 | -1.550 | 1.550 | 1.350 | 1.350 | -1.250 | 1.042 | 1.142 | 57724/38959 | 0 | 5867305 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 40ms | $167,324,240,851 | -1.373 | -1.397 | -1.550 | 1.550 | 1.350 | 1.350 | -1.367 | 1.183 | 1.166 | 53865/38227 | 0 | 5868084 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 40ms | $167,492,015,851 | -1.380 | -1.402 | -1.551 | 1.551 | 1.351 | 1.351 | -1.376 | 1.186 | 1.161 | 50978/38082 | 0 | 5868194 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $164,933,965,851 | $1,786,550,000 | $1,029,925,000 | $167,750,440,851 | $0 | -1.708 | -2.751 | -1.109 |
| B: Cancel-Priority 70ms | react 10ms | $164,933,965,851 | $157,700,000 | $1,162,150,000 | $166,253,815,851 | $0 | -0.889 | -2.685 | -1.026 |
| C: Uniform-Price FBA 100ms | react 10ms | $145,842,909,099 | $0 | $0 | $145,842,909,099 | $0 | -0.861 | n/a | n/a |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $164,933,965,851 | $232,050,000 | $1,145,750,000 | $166,311,765,851 | $0 | -1.053 | -2.702 | -1.056 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $164,933,965,851 | $213,075,000 | $1,519,875,000 | $166,666,915,851 | $0 | -1.055 | -2.695 | -1.014 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $164,933,965,851 | $200,350,000 | $1,759,375,000 | $166,893,690,851 | $0 | -1.057 | -2.693 | -1.010 |
| A: Continuous FIFO | react 25ms | $164,933,965,851 | $2,255,600,000 | $646,125,000 | $167,835,690,851 | $0 | -1.760 | -2.812 | -1.088 |
| B: Cancel-Priority 70ms | react 25ms | $164,933,965,851 | $548,525,000 | $917,025,000 | $166,399,515,851 | $0 | -1.082 | -2.714 | -1.016 |
| C: Uniform-Price FBA 100ms | react 25ms | $145,877,058,397 | $0 | $0 | $145,877,058,397 | $0 | -1.026 | n/a | n/a |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 25ms | $164,933,965,851 | $796,750,000 | $890,550,000 | $166,621,265,851 | $0 | -1.309 | -2.759 | -1.007 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 25ms | $164,933,965,851 | $736,350,000 | $1,247,475,000 | $166,917,790,851 | $0 | -1.311 | -2.774 | -0.929 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 25ms | $164,933,965,851 | $693,550,000 | $1,484,100,000 | $167,111,615,851 | $0 | -1.314 | -2.779 | -0.971 |
| A: Continuous FIFO | react 40ms | $164,933,965,851 | $2,529,875,000 | $409,400,000 | $167,873,240,851 | $0 | -1.795 | -2.853 | -1.098 |
| B: Cancel-Priority 70ms | react 40ms | $164,933,965,851 | $1,005,275,000 | $745,825,000 | $166,685,065,851 | $0 | -1.246 | -2.795 | -0.930 |
| C: Uniform-Price FBA 100ms | react 40ms | $145,907,016,173 | $0 | $0 | $145,907,016,173 | $0 | -1.202 | n/a | n/a |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 40ms | $164,933,965,851 | $1,443,100,000 | $702,700,000 | $167,079,765,851 | $0 | -1.541 | -2.809 | -0.918 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 40ms | $164,933,965,851 | $1,346,625,000 | $1,043,650,000 | $167,324,240,851 | $0 | -1.544 | -2.811 | -0.932 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 40ms | $164,933,965,851 | $1,274,450,000 | $1,283,600,000 | $167,492,015,851 | $0 | -1.546 | -2.817 | -0.945 |

## Placebo test: circularly shifted taker flow

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 600s | $167,750,440,851 | -0.036 | -0.037 | -0.055 | 0.055 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 71462/171 | 0 | 0 |
| B: Cancel-Priority 70ms | react 10ms | PLACEBO shift 600s | $166,253,815,851 | -0.010 | -0.012 | -0.029 | 0.029 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 6308/68545 | 0 | 5504187 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 600s | $146,056,993,645 | -0.012 | -0.015 | -0.036 | 0.036 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 122928/0 | 0 | 4990854 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | PLACEBO shift 600s | $166,311,765,851 | -0.012 | -0.014 | -0.032 | 0.032 | <0 | 0.000 | -0.004 | 0.000 | -0.004 | 9282/64857 | 0 | 5959770 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | PLACEBO shift 600s | $166,666,915,851 | -0.013 | -0.015 | -0.033 | 0.033 | <0 | 0.000 | -0.004 | 0.000 | -0.004 | 8523/63299 | 0 | 5969950 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | PLACEBO shift 600s | $166,893,690,851 | -0.016 | -0.018 | -0.035 | 0.035 | <0 | 0.000 | -0.005 | 0.002 | -0.003 | 8014/62620 | 0 | 5977320 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 3600s | $167,750,440,851 | -0.038 | -0.039 | -0.046 | 0.046 | <0 | 0.000 | -0.005 | 0.000 | -0.004 | 71462/171 | 0 | 0 |
| B: Cancel-Priority 70ms | react 10ms | PLACEBO shift 3600s | $166,253,815,851 | -0.013 | -0.014 | -0.020 | 0.020 | <0 | 0.000 | -0.006 | 0.001 | -0.004 | 6308/68545 | 0 | 5512948 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 3600s | $146,121,415,305 | -0.014 | -0.015 | -0.023 | 0.023 | <0 | 0.000 | -0.004 | 0.002 | -0.002 | 122928/0 | 0 | 4999244 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | PLACEBO shift 3600s | $166,311,765,851 | -0.014 | -0.014 | -0.021 | 0.021 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 9282/64857 | 0 | 5967809 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | PLACEBO shift 3600s | $166,666,915,851 | -0.015 | -0.017 | -0.022 | 0.022 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 8523/63299 | 0 | 5978577 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | PLACEBO shift 3600s | $166,893,690,851 | -0.016 | -0.018 | -0.024 | 0.024 | <0 | 0.000 | -0.006 | 0.002 | -0.003 | 8014/62620 | 0 | 5986284 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.715 bp, B -0.892 bp, cancel-priority gap (B minus A) 0.823 bp.
- Placebo shift 600 s: A -0.055 bp, B -0.029 bp, gap 0.025 bp = 3.1% of the real gap.
- Placebo shift 3600 s: A -0.046 bp, B -0.020 bp, gap 0.025 bp = 3.0% of the real gap.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.823 bp = 47.9% of A's 1s markout magnitude.
- react 25ms: handicap (MM minus sniper reaction) 23ms, B window 70ms, handicap/window 32%, gap 0.684 bp = 38.6% of A's 1s markout magnitude.
- react 40ms: handicap (MM minus sniper reaction) 38ms, B window 70ms, handicap/window 54%, gap 0.554 bp = 30.6% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.715 bps, 95% CI [-1.869, -1.549]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window 70ms, D window 50ms, decrypt 0ms | react 10ms | pending=hold | mid=bidask
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | -0.012 ~ | -0.001 ~ | 0.010 ~ | 0.033 B<D | 0.056 B<D |
| 10% | -0.012 ~ | -0.036 D<B | -0.060 D<B | -0.108 D<B | -0.156 D<B |
| 25% | -0.012 ~ | -0.089 D<B | -0.166 D<B | -0.321 D<B | -0.475 D<B |
| 50% | -0.012 ~ | -0.178 D<B | -0.344 D<B | -0.675 D<B | -1.007 D<B |
| 75% | -0.012 ~ | -0.267 D<B | -0.521 D<B | -1.030 D<B | -1.539 D<B |
| 100% | -0.012 ~ | -0.355 D<B | -0.698 D<B | -1.384 D<B | -2.071 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window 70ms, D window 50ms, decrypt 35ms | react 10ms | pending=hold | mid=bidask
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.013 ~ | 0.025 B<D | 0.036 B<D | 0.059 B<D | 0.082 B<D |
| 10% | 0.013 ~ | -0.010 ~ | -0.034 D<B | -0.082 D<B | -0.129 D<B |
| 25% | 0.013 ~ | -0.063 D<B | -0.140 D<B | -0.294 D<B | -0.449 D<B |
| 50% | 0.013 ~ | -0.152 D<B | -0.317 D<B | -0.649 D<B | -0.981 D<B |
| 75% | 0.013 ~ | -0.240 D<B | -0.495 D<B | -1.004 D<B | -1.512 D<B |
| 100% | 0.013 ~ | -0.329 D<B | -0.672 D<B | -1.358 D<B | -2.044 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B window 70ms, D window 50ms, decrypt 70ms | react 10ms | pending=hold | mid=bidask
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.009 ~ | 0.020 ~ | 0.032 B<D | 0.055 B<D | 0.078 B<D |
| 10% | 0.009 ~ | -0.014 ~ | -0.038 D<B | -0.086 D<B | -0.134 D<B |
| 25% | 0.009 ~ | -0.068 D<B | -0.145 D<B | -0.299 D<B | -0.453 D<B |
| 50% | 0.009 ~ | -0.156 D<B | -0.322 D<B | -0.654 D<B | -0.985 D<B |
| 75% | 0.009 ~ | -0.245 D<B | -0.499 D<B | -1.008 D<B | -1.517 D<B |
| 100% | 0.009 ~ | -0.334 D<B | -0.677 D<B | -1.363 D<B | -2.049 D<B |

## Calibration context (public documentation, retrieved 2026-11-18; NOT simulation inputs)
- **Hyperliquid: end-to-end latency:** The docs define end-to-end latency as the time from sending a request to receiving the committed response. A third-party mirror that cites the official docs reports a median of about 0.2 s and a 99th percentile of about 0.9 s for a co-located client (an older analyst note quotes a 0.3 s median). The numeric values were not visible in the official page excerpt retrieved here. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview ; https://www.dwellir.com/docs/hyperliquid/architecture_
- **Hyperliquid: block interval:** Not stated in the official page retrieved. A third-party node-docs site says blocks come about every 70 ms with finality in about two rounds (around 140 ms); an analyst post describes block commits in the range of 200-300 ms and an observed lag of about 700 ms in fills versus Binance (maker update in one block, taker fill in the next). Treat the block interval as roughly 70-300 ms and UNVERIFIED. _Source: https://hypernode-docs.polynode.dev/research/consensus ; https://arrakis.finance/blog/crypto-price-discovery_
- **dYdX Chain (v4): block time:** The official onboarding FAQ states an average block time of about 1 second, and that short-term order cancels are best effort until the block is final. Some third-party pages claim 0.5 s or even 100 ms blocks; they conflict with the official FAQ and are not used here. _Source: https://docs.dydx.xyz/concepts/onboarding-faqs_
- **Not documented (stay assumptions):** No venue documentation was found for market-maker reaction time, market-data feed latency, client-to-sequencer network latency or latency-arbitrageur reaction time. The model defaults (feed 5 ms, MM reaction 10 ms, sniper reaction 2 ms, network 30 ms) are assumptions. Sweep them (--mm-react) instead of trusting a single value. _Source: n/a_
- **Dywe window:** The 50 ms micro-batch and the 35 ms decryption RTT are Dywe design/benchmark parameters, not venue documentation. They are not validated by any source above. _Source: n/a_
- **Implication for Regime B:** If B is meant to represent Hyperliquid, a 50 ms window is shorter than every documented latency figure above. Use --b-window-ms 70 and --b-window-ms 200 (separate runs) to read the cancel-priority gap at those scales, and --b-window-ms 1000 for a dYdX-like cadence. _Source: n/a_
