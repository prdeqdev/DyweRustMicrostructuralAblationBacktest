# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.3)
Window: 2026-11-10 + 7d | Pairs: BTCUSDT,ETHUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 30 | B 70ms (hl), D 50ms (cancel-first), FBA 100ms | pending=hold | mid=bidask | wait=all | rebate=+0.500bp, opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 14 of 14.

**NOTE:** 14 pair-days used a trade-derived midpoint proxy (mode `bidask`) because `bookTicker` files were not supplied. BidAskProxy diagnostics: 84131187 total observations, 695039 crossed (0.8% where ask < bid), mean valid (ask - bid) spread = 0.708 bps.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $100,180,065,567 | -1.173 | -1.487 | -1.614 | 1.614 | 1.414 | 1.414 | -1.166 | 0.631 | 0.880 | 25252/37 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $100,272,465,567 | -0.618 | -0.719 | -0.874 | 0.874 | 0.674 | 0.674 | -0.620 | 0.833 | 0.887 | 2237/22 | 0 (0.0%) | 2629263 |
| C: Uniform-Price FBA 100ms | react 10ms | $88,087,778,273 | -0.532 | -0.651 | -0.815 | 0.815 | 0.615 | 0.541 | -0.467 | 0.836 | 0.910 | 1464/16 | 0 (0.0%) | 2407863 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 10ms | $99,687,065,567 | -0.716 | -0.920 | -1.017 | 1.017 | 0.817 | 0.817 | -0.714 | 0.781 | 0.883 | 3178/22741 | 0 (0.0%) | 2823633 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 10ms | $99,813,240,567 | -0.822 | -0.883 | -1.022 | 1.022 | 0.822 | 0.822 | -0.821 | 0.888 | 0.889 | 2958/22253 | 0 (0.0%) | 2823977 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 10ms | $99,895,140,567 | -0.849 | -0.893 | -1.020 | 1.020 | 0.820 | 0.820 | -0.848 | 0.915 | 0.886 | 2815/22116 | 0 (0.0%) | 2824049 |
| A: Continuous FIFO | react 25ms | $100,210,340,567 | -1.224 | -1.537 | -1.665 | 1.665 | 1.465 | 1.465 | -1.214 | 0.631 | 0.882 | 32298/23 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 25ms | $100,297,315,567 | -0.807 | -0.908 | -1.063 | 1.063 | 0.863 | 0.863 | -0.808 | 0.833 | 0.888 | 7976/19 | 0 (0.0%) | 2638764 |
| C: Uniform-Price FBA 100ms | react 25ms | $88,097,622,337 | -0.672 | -0.792 | -0.955 | 0.955 | 0.755 | 0.664 | -0.590 | 0.836 | 0.910 | 5178/22 | 0 (0.0%) | 2412924 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 25ms | $99,776,765,567 | -0.944 | -1.148 | -1.244 | 1.244 | 1.044 | 1.044 | -0.940 | 0.781 | 0.885 | 11217/20257 | 0 (0.0%) | 2839167 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 25ms | $99,882,615,567 | -1.049 | -1.110 | -1.249 | 1.249 | 1.049 | 1.049 | -1.047 | 0.888 | 0.890 | 10564/19767 | 0 (0.0%) | 2839464 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 25ms | $99,955,715,567 | -1.076 | -1.120 | -1.247 | 1.247 | 1.047 | 1.047 | -1.074 | 0.915 | 0.887 | 10070/19559 | 0 (0.0%) | 2839515 |
| A: Continuous FIFO | react 40ms | $100,232,115,567 | -1.255 | -1.568 | -1.696 | 1.696 | 1.496 | 1.496 | -1.243 | 0.631 | 0.883 | 36011/13 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 40ms | $100,316,940,567 | -0.922 | -1.022 | -1.177 | 1.177 | 0.977 | 0.977 | -0.920 | 0.833 | 0.890 | 14524/25 | 0 (0.0%) | 2648685 |
| C: Uniform-Price FBA 100ms | react 40ms | $88,109,469,297 | -0.865 | -0.985 | -1.149 | 1.149 | 0.949 | 0.834 | -0.759 | 0.836 | 0.911 | 9736/22 | 0 (0.0%) | 2418118 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 40ms | $99,947,065,567 | -1.183 | -1.386 | -1.483 | 1.483 | 1.283 | 1.283 | -1.176 | 0.781 | 0.887 | 20747/14144 | 0 (0.0%) | 2855267 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 40ms | $100,037,015,567 | -1.287 | -1.348 | -1.487 | 1.487 | 1.287 | 1.287 | -1.283 | 0.888 | 0.892 | 19689/13915 | 0 (0.0%) | 2855560 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 40ms | $100,099,165,567 | -1.314 | -1.358 | -1.484 | 1.484 | 1.284 | 1.284 | -1.310 | 0.915 | 0.889 | 18827/13844 | 0 (0.0%) | 2855595 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $67,572,150,284 | -1.434 | -1.705 | -1.870 | 1.870 | 1.670 | 1.670 | -1.418 | 0.746 | 0.998 | 46381/60 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $67,693,150,284 | -0.652 | -0.732 | -0.902 | 0.902 | 0.702 | 0.702 | -0.658 | 0.937 | 0.980 | 4071/38 | 0 (0.0%) | 2722991 |
| C: Uniform-Price FBA 100ms | react 10ms | $57,749,551,956 | -0.694 | -0.738 | -0.933 | 0.933 | 0.733 | 0.626 | -0.598 | 0.983 | 1.011 | 2759/27 | 0 (0.0%) | 2457705 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 10ms | $66,622,150,284 | -0.833 | -0.939 | -1.112 | 1.112 | 0.912 | 0.912 | -0.831 | 0.915 | 0.996 | 6102/42208 | 0 (0.0%) | 2963556 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 10ms | $66,851,725,284 | -0.962 | -0.933 | -1.108 | 1.108 | 0.908 | 0.908 | -0.963 | 1.047 | 0.992 | 5565/41113 | 0 (0.0%) | 2964211 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 10ms | $66,996,525,284 | -0.945 | -0.933 | -1.118 | 1.118 | 0.918 | 0.918 | -0.946 | 1.030 | 1.002 | 5199/40566 | 0 (0.0%) | 2964290 |
| A: Continuous FIFO | react 25ms | $67,626,150,284 | -1.500 | -1.772 | -1.937 | 1.937 | 1.737 | 1.737 | -1.479 | 0.746 | 1.004 | 58054/49 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 25ms | $67,744,375,284 | -0.855 | -0.934 | -1.102 | 1.102 | 0.902 | 0.902 | -0.859 | 0.938 | 0.981 | 13964/39 | 0 (0.0%) | 2737444 |
| C: Uniform-Price FBA 100ms | react 25ms | $57,776,313,454 | -0.896 | -0.938 | -1.132 | 1.132 | 0.932 | 0.796 | -0.771 | 0.984 | 1.010 | 9337/25 | 0 (0.0%) | 2465436 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 25ms | $66,842,175,284 | -1.141 | -1.246 | -1.419 | 1.419 | 1.219 | 1.219 | -1.134 | 0.915 | 1.000 | 20648/35420 | 0 (0.0%) | 2987438 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 25ms | $67,033,250,284 | -1.268 | -1.238 | -1.413 | 1.413 | 1.213 | 1.213 | -1.266 | 1.048 | 0.994 | 18889/34403 | 0 (0.0%) | 2987984 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 25ms | $67,153,825,284 | -1.249 | -1.237 | -1.421 | 1.421 | 1.221 | 1.221 | -1.249 | 1.030 | 1.003 | 17670/34105 | 0 (0.0%) | 2988063 |
| A: Continuous FIFO | react 40ms | $67,642,125,284 | -1.542 | -1.814 | -1.979 | 1.979 | 1.779 | 1.779 | -1.517 | 0.746 | 1.009 | 65265/15 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 40ms | $67,769,150,284 | -1.099 | -1.178 | -1.347 | 1.347 | 1.147 | 1.147 | -1.101 | 0.938 | 0.984 | 25681/57 | 0 (0.0%) | 2752385 |
| C: Uniform-Price FBA 100ms | react 40ms | $57,793,444,714 | -1.047 | -1.089 | -1.283 | 1.283 | 1.083 | 0.925 | -0.897 | 0.984 | 1.013 | 17046/29 | 0 (0.0%) | 2473345 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 40ms | $67,131,400,284 | -1.374 | -1.478 | -1.649 | 1.649 | 1.449 | 1.449 | -1.360 | 0.915 | 1.004 | 36970/24861 | 0 (0.0%) | 3012147 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 40ms | $67,286,275,284 | -1.499 | -1.469 | -1.643 | 1.643 | 1.443 | 1.443 | -1.493 | 1.047 | 0.997 | 34175/24342 | 0 (0.0%) | 3012633 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 40ms | $67,391,500,284 | -1.479 | -1.467 | -1.651 | 1.651 | 1.451 | 1.451 | -1.476 | 1.030 | 1.005 | 32149/24285 | 0 (0.0%) | 3012708 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $167,752,215,851 | -1.278 | -1.575 | -1.717 | 1.717 | 1.517 | 1.517 | -1.266 | 0.677 | 0.928 | 71633/97 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | $167,965,615,851 | -0.632 | -0.724 | -0.885 | 0.885 | 0.685 | 0.685 | -0.635 | 0.874 | 0.924 | 6308/60 | 0 (0.0%) | 5352254 |
| C: Uniform-Price FBA 100ms | react 10ms | $145,837,330,229 | -0.596 | -0.686 | -0.862 | 0.862 | 0.662 | 0.575 | -0.519 | 0.895 | 0.950 | 4223/43 | 0 (0.0%) | 4865568 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 10ms | $166,309,215,851 | -0.763 | -0.928 | -1.055 | 1.055 | 0.855 | 0.855 | -0.761 | 0.834 | 0.928 | 9280/64949 | 0 (0.0%) | 5787189 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 10ms | $166,664,965,851 | -0.878 | -0.903 | -1.057 | 1.057 | 0.857 | 0.857 | -0.878 | 0.951 | 0.930 | 8523/63366 | 0 (0.0%) | 5788188 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 10ms | $166,891,665,851 | -0.887 | -0.909 | -1.059 | 1.059 | 0.859 | 0.859 | -0.887 | 0.961 | 0.932 | 8014/62682 | 0 (0.0%) | 5788339 |
| A: Continuous FIFO | react 25ms | $167,836,490,851 | -1.335 | -1.632 | -1.775 | 1.775 | 1.575 | 1.575 | -1.320 | 0.677 | 0.932 | 90352/72 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 25ms | $168,041,690,851 | -0.826 | -0.918 | -1.079 | 1.079 | 0.879 | 0.879 | -0.829 | 0.875 | 0.925 | 21940/58 | 0 (0.0%) | 5376208 |
| C: Uniform-Price FBA 100ms | react 25ms | $145,873,935,791 | -0.761 | -0.850 | -1.025 | 1.025 | 0.825 | 0.717 | -0.662 | 0.895 | 0.950 | 14515/47 | 0 (0.0%) | 4878360 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 25ms | $166,618,940,851 | -1.023 | -1.187 | -1.314 | 1.314 | 1.114 | 1.114 | -1.017 | 0.835 | 0.931 | 31865/55677 | 0 (0.0%) | 5826605 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 25ms | $166,915,865,851 | -1.137 | -1.162 | -1.315 | 1.315 | 1.115 | 1.115 | -1.135 | 0.952 | 0.932 | 29453/54170 | 0 (0.0%) | 5827448 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 25ms | $167,109,540,851 | -1.146 | -1.167 | -1.317 | 1.317 | 1.117 | 1.117 | -1.144 | 0.961 | 0.934 | 27740/53664 | 0 (0.0%) | 5827578 |
| A: Continuous FIFO | react 40ms | $167,874,240,851 | -1.370 | -1.667 | -1.810 | 1.810 | 1.610 | 1.610 | -1.352 | 0.677 | 0.935 | 101276/28 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 40ms | $168,086,090,851 | -0.993 | -1.085 | -1.246 | 1.246 | 1.046 | 1.046 | -0.992 | 0.875 | 0.928 | 40205/82 | 0 (0.0%) | 5401070 |
| C: Uniform-Price FBA 100ms | react 40ms | $145,902,914,011 | -0.937 | -1.026 | -1.202 | 1.202 | 1.002 | 0.871 | -0.814 | 0.895 | 0.952 | 26782/51 | 0 (0.0%) | 4891463 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 40ms | $167,078,465,851 | -1.259 | -1.423 | -1.550 | 1.550 | 1.350 | 1.350 | -1.250 | 0.834 | 0.934 | 57717/39005 | 0 (0.0%) | 5867414 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 40ms | $167,323,290,851 | -1.372 | -1.397 | -1.550 | 1.550 | 1.350 | 1.350 | -1.367 | 0.951 | 0.934 | 53864/38257 | 0 (0.0%) | 5868193 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 40ms | $167,490,665,851 | -1.380 | -1.402 | -1.551 | 1.551 | 1.351 | 1.351 | -1.376 | 0.961 | 0.936 | 50976/38129 | 0 (0.0%) | 5868303 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | Large Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $164,933,965,851 | $1,790,825,000 | $1,027,425,000 | $167,752,215,851 | $0 | -1.710 | -2.754 | -1.112 | 71.2% | 0.0% |
| B: Batch 70ms (hl) | react 10ms | $164,933,965,851 | $157,700,000 | $2,873,950,000 | $167,965,615,851 | $0 | -0.889 | -2.685 | -0.527 | 71.2% | 37.7% |
| C: Uniform-Price FBA 100ms | react 10ms | $145,837,330,229 | $0 | $0 | $145,837,330,229 | $0 | -0.862 | n/a | n/a | 71.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 10ms | $164,933,965,851 | $232,000,000 | $1,143,250,000 | $166,309,215,851 | $0 | -1.053 | -2.702 | -1.055 | 71.2% | 32.5% |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 10ms | $164,933,965,851 | $213,075,000 | $1,517,925,000 | $166,664,965,851 | $0 | -1.055 | -2.695 | -1.015 | 71.2% | 32.5% |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 10ms | $164,933,965,851 | $200,350,000 | $1,757,350,000 | $166,891,665,851 | $0 | -1.058 | -2.693 | -1.011 | 71.2% | 32.5% |
| A: Continuous FIFO | react 25ms | $164,933,965,851 | $2,258,800,000 | $643,725,000 | $167,836,490,851 | $0 | -1.763 | -2.812 | -1.090 | 71.2% | 0.0% |
| B: Batch 70ms (hl) | react 25ms | $164,933,965,851 | $548,500,000 | $2,559,225,000 | $168,041,690,851 | $0 | -1.082 | -2.714 | -0.516 | 71.2% | 30.8% |
| C: Uniform-Price FBA 100ms | react 25ms | $145,873,935,791 | $0 | $0 | $145,873,935,791 | $0 | -1.025 | n/a | n/a | 71.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 25ms | $164,933,965,851 | $796,625,000 | $888,350,000 | $166,618,940,851 | $0 | -1.309 | -2.759 | -1.012 | 71.2% | 23.0% |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 25ms | $164,933,965,851 | $736,325,000 | $1,245,575,000 | $166,915,865,851 | $0 | -1.311 | -2.774 | -0.933 | 71.2% | 23.0% |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 25ms | $164,933,965,851 | $693,500,000 | $1,482,075,000 | $167,109,540,851 | $0 | -1.314 | -2.779 | -0.974 | 71.2% | 23.0% |
| A: Continuous FIFO | react 40ms | $164,933,965,851 | $2,531,900,000 | $408,375,000 | $167,874,240,851 | $0 | -1.796 | -2.853 | -1.099 | 71.2% | 0.0% |
| B: Batch 70ms (hl) | react 40ms | $164,933,965,851 | $1,005,125,000 | $2,147,000,000 | $168,086,090,851 | $0 | -1.246 | -2.795 | -0.499 | 71.2% | 23.8% |
| C: Uniform-Price FBA 100ms | react 40ms | $145,902,914,011 | $0 | $0 | $145,902,914,011 | $0 | -1.202 | n/a | n/a | 71.2% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 40ms | $164,933,965,851 | $1,442,925,000 | $701,575,000 | $167,078,465,851 | $0 | -1.541 | -2.809 | -0.916 | 71.2% | 13.8% |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 40ms | $164,933,965,851 | $1,346,600,000 | $1,042,725,000 | $167,323,290,851 | $0 | -1.544 | -2.811 | -0.931 | 71.2% | 13.8% |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 40ms | $164,933,965,851 | $1,274,400,000 | $1,282,300,000 | $167,490,665,851 | $0 | -1.546 | -2.817 | -0.944 | 71.2% | 13.8% |

Reconciliation note: `StaleSnipe` represents orders hitting the exact un-updated quote ID targeted at trigger time; `LateSnipe` represents sniper orders arriving after MM already requoted. In Regime C (Uniform-Price FBA), taker and sniper orders net internally into a single imbalance before clearing against the MM, so sniper volumes are absorbed into `Taker Vol` (net imbalance). `Cancel-Batch Large %` shows the share of large orders arriving inside a batch that also contains an MM cancel (the exposure base for `leak_ppm` in Regime D).

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 600s | $167,752,215,851 | -0.036 | -0.037 | -0.055 | 0.055 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 71633/97 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 600s | $167,965,615,851 | -0.011 | -0.013 | -0.031 | 0.031 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 6308/60 | 0 (0.0%) | 5504280 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 600s | $146,055,739,319 | -0.012 | -0.015 | -0.036 | 0.036 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 4223/43 | 0 (0.0%) | 4990907 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 10ms | PLACEBO shift 600s | $166,309,215,851 | -0.012 | -0.014 | -0.032 | 0.032 | <0 | 0.000 | -0.004 | 0.000 | -0.004 | 9280/64949 | 0 (0.0%) | 5959949 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 10ms | PLACEBO shift 600s | $166,664,965,851 | -0.013 | -0.015 | -0.033 | 0.033 | <0 | 0.000 | -0.004 | 0.000 | -0.004 | 8523/63366 | 0 (0.0%) | 5970122 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 10ms | PLACEBO shift 600s | $166,891,665,851 | -0.016 | -0.018 | -0.035 | 0.035 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 8014/62682 | 0 (0.0%) | 5977488 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | PLACEBO shift 3600s | $167,752,215,851 | -0.039 | -0.039 | -0.046 | 0.046 | <0 | 0.000 | -0.005 | 0.000 | -0.004 | 71633/97 | 0 (0.0%) | 0 |
| B: Batch 70ms (hl) | react 10ms | PLACEBO shift 3600s | $167,965,615,851 | -0.014 | -0.015 | -0.022 | 0.022 | <0 | 0.000 | -0.006 | 0.001 | -0.005 | 6308/60 | 0 (0.0%) | 5513066 |
| C: Uniform-Price FBA 100ms | react 10ms | PLACEBO shift 3600s | $146,120,183,135 | -0.014 | -0.015 | -0.023 | 0.023 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 4223/43 | 0 (0.0%) | 4999295 |
| D: Dywe IBTE 50ms (dec 0ms, cancel-first) | react 10ms | PLACEBO shift 3600s | $166,309,215,851 | -0.014 | -0.014 | -0.021 | 0.021 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 9280/64949 | 0 (0.0%) | 5967992 |
| D: Dywe IBTE 50ms (dec 35ms, cancel-first) | react 10ms | PLACEBO shift 3600s | $166,664,965,851 | -0.015 | -0.017 | -0.022 | 0.022 | <0 | 0.000 | -0.005 | 0.001 | -0.004 | 8523/63366 | 0 (0.0%) | 5978752 |
| D: Dywe IBTE 50ms (dec 70ms, cancel-first) | react 10ms | PLACEBO shift 3600s | $166,891,665,851 | -0.016 | -0.018 | -0.024 | 0.024 | <0 | 0.000 | -0.006 | 0.001 | -0.004 | 8014/62682 | 0 (0.0%) | 5986452 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -1.717 bp, B -0.885 bp, cancel-priority gap (B minus A) 0.832 bp.
- Placebo shift 600 s: A -0.055 bp, B -0.031 bp, gap 0.024 bp = 2.8% of the real gap.
- Placebo shift 3600 s: A -0.046 bp, B -0.022 bp, gap 0.023 bp = 2.8% of the real gap.
- Reading: what survives the placebo does not depend on the flow predicting prices. What disappears depends on the temporal alignment of real taker flow (and, with a trade-derived midpoint proxy, partly on construction).

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.832 bp = 48.4% of A's 1s markout magnitude.
- react 25ms: handicap (MM minus sniper reaction) 23ms, B window 70ms, handicap/window 32%, gap 0.696 bp = 39.2% of A's 1s markout magnitude.
- react 40ms: handicap (MM minus sniper reaction) 38ms, B window 70ms, handicap/window 54%, gap 0.564 bp = 31.1% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -1.717 bps, 95% CI [-1.871, -1.550]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × leak`), not measurements. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, cancel-first) | react 10ms | pending=hold | mid=bidask | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.004 ~ | 0.015 B<D | 0.027 B<D | 0.050 B<D | 0.073 B<D |
| 10% | 0.004 ~ | -0.019 D<B | -0.043 D<B | -0.090 D<B | -0.138 D<B |
| 25% | 0.004 ~ | -0.072 D<B | -0.149 D<B | -0.302 D<B | -0.455 D<B |
| 50% | 0.004 ~ | -0.160 D<B | -0.325 D<B | -0.655 D<B | -0.984 D<B |
| 75% | 0.004 ~ | -0.248 D<B | -0.501 D<B | -1.007 D<B | -1.514 D<B |
| 100% | 0.004 ~ | -0.336 D<B | -0.678 D<B | -1.360 D<B | -2.043 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, cancel-first) | react 10ms | pending=hold | mid=bidask | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.005 B<D | 0.017 B<D | 0.028 B<D | 0.051 B<D | 0.075 B<D |
| 10% | 0.005 B<D | -0.017 D<B | -0.041 D<B | -0.089 D<B | -0.136 D<B |
| 25% | 0.005 B<D | -0.070 D<B | -0.147 D<B | -0.300 D<B | -0.454 D<B |
| 50% | 0.005 B<D | -0.159 D<B | -0.323 D<B | -0.653 D<B | -0.983 D<B |
| 75% | 0.005 B<D | -0.247 D<B | -0.500 D<B | -1.006 D<B | -1.512 D<B |
| 100% | 0.005 B<D | -0.335 D<B | -0.676 D<B | -1.359 D<B | -2.041 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, cancel-first) | react 10ms | pending=hold | mid=bidask | wait=all
Rows: MEV capture rate on large orders (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Verdict: `D<B` = Dywe cheaper for takers (95% CI < 0); `B<D` = Regime B cheaper; `~` = statistically indistinguishable. **Every cell except the 0%-capture row is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.008 ~ | 0.019 B<D | 0.031 B<D | 0.054 B<D | 0.077 B<D |
| 10% | 0.008 ~ | -0.015 D<B | -0.039 D<B | -0.086 D<B | -0.134 D<B |
| 25% | 0.008 ~ | -0.068 D<B | -0.145 D<B | -0.298 D<B | -0.451 D<B |
| 50% | 0.008 ~ | -0.156 D<B | -0.321 D<B | -0.651 D<B | -0.981 D<B |
| 75% | 0.008 ~ | -0.244 D<B | -0.497 D<B | -1.004 D<B | -1.510 D<B |
| 100% | 0.008 ~ | -0.333 D<B | -0.674 D<B | -1.356 D<B | -2.039 D<B |

## Calibration context (public documentation; NOT simulation inputs)
- **Hyperliquid: intra-batch priority order:** Official docs (hyperliquid-l1/order-book) state that within each consensus batch, non-GTC/IOC actions (i.e. post-only / ALO) are processed first, followed by cancels, followed by GTC/IOC orders. Thus new MM post-only quotes land before cancels and takers without a dead gap. Model this with `--b-batch-order hl`. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-l1/order-book_
- **Hyperliquid: block interval & buckets:** Official docs (for-developers/api/priority-fees) state that IOC orders are sorted in 70 ms unix-time buckets, typically corresponding to one block (with 1-2 consensus batches per block, meaning effective batch windows may be shorter than 70 ms). End-to-end co-located latency is cited around ~0.2 s median in third-party mirrors. _Source: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/priority-fees ; https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview_
- **dYdX Chain (v4): block time:** The official onboarding FAQ states an average block time of about 1 second, and that short-term order cancels are best effort until the block is final. Use `--b-window-ms 1000` to evaluate a 1-second batching cadence. _Source: https://docs.dydx.xyz/concepts/onboarding-faqs_
- **Not documented (stay assumptions):** No venue documentation specifies MM reaction time, market-data feed latency, client-to-sequencer network transit, or latency-arbitrageur reaction time. Defaults (feed 5 ms, MM reaction 10 ms, sniper reaction 2 ms, network 30 ms) are parameterized assumptions; sweep `--mm-react` rather than trusting a single point. _Source: n/a_
- **Dywe window & post-decrypt matching:** The 50 ms micro-batch and 35 ms / 70 ms decryption RTTs are Dywe design parameters, not live multi-region Devnet measurements. Because threshold decryption completes in matching-engine memory BEFORE the first fill at `clear_ns`, Dywe can execute `hl` post-only-first ordering (`--d-batch-order hl`) without leaking ALO/IOC flags pre-commit. Use `--d-batch-order cancel-first` to stress-test strict pre-decrypt cancel-only sorting. _Source: n/a_
- **Stylized model disclaimer:** Regimes B (70 ms / 200 ms / 1000 ms) are stylized batching models, not byte-for-byte reproductions of Hyperliquid or dYdX matching engines. _Source: n/a_

## Methodological Boundaries & Scope
- Regime D timeline: orders arrive inside a batch window, commit at `batch_close`, complete threshold decryption at `clear_ns = batch_close + decrypt_ms`, and clear in matching-engine memory using `--d-batch-order cancel-first`.
- Regime B uses `--b-batch-order hl` (`hl` = New/post-only -> Cancel -> Taker/Sniper; `cancel-first` = Cancel -> New/Taker/Sniper).
- Single-denominator All-In Net: `--wait-cohort all` (`all` uses total executed taker volume `tk_n` across all 4 components; `large` isolates waiting drift of large orders >= $50,000).
- Under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net; inspect `Dropped Takers (USD%)` and selection-bias warnings before comparing costs.
- 95% CI reflects drift variance across multi-hour blocks under FIXED scenario assumptions; parameter uncertainty (capture, impact, leak, reaction times, windows) is not covered.
- Single 7-day sample window; BTCUSDT and ETHUSDT are strongly correlated and resampled jointly.
- Taker flow is exogenous and invariant to spread width; one representative MM clears the full flow; sniper triggers are threshold-bound.
