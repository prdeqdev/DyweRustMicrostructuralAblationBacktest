# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.2.2)
Window: 2026-11-10 + 7d | Pairs: BTCUSDT,ETHUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 6

Pair-Days Processed: 14 of 14.

**NOTE:** 14 pair-days were evaluated using RECONSTRUCTED L1 mid-prices (`price ± half_tick`, zero EMA smoothing). Do not mix with raw `bookTicker` days without noting the methodology difference.

### BTCUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $100,370,290,567 | -1.301 | -1.516 | -1.569 | 1.569 | 1.369 | 1.369 | -1.296 | 0.450 | 0.524 | 26343/140 | 0 |
| B: Cancel-Priority 50ms | react 10ms | $99,827,440,567 | -0.694 | -0.793 | -0.882 | 0.882 | 0.682 | 0.682 | -0.693 | 0.555 | 0.543 | 2820/24196 | 2820129 |
| C: Uniform-Price FBA 100ms | react 10ms | $88,367,407,599 | -0.475 | -0.560 | -0.670 | 0.670 | 0.470 | 0.413 | -0.418 | 0.572 | 0.568 | 50463/0 | 2406862 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $99,827,440,567 | -0.694 | -0.793 | -0.882 | 0.882 | 0.682 | 0.682 | -0.693 | 0.555 | 0.543 | 2820/24196 | 2820129 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $99,926,415,567 | -0.752 | -0.805 | -0.886 | 0.886 | 0.686 | 0.686 | -0.752 | 0.622 | 0.557 | 2610/23694 | 2820327 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $99,995,815,567 | -0.805 | -0.820 | -0.886 | 0.886 | 0.686 | 0.686 | -0.805 | 0.687 | 0.568 | 2498/23372 | 2820362 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### ETHUSDT (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $68,000,150,284 | -1.625 | -1.742 | -1.814 | 1.814 | 1.614 | 1.614 | -1.618 | 0.416 | 0.412 | 52676/204 | 0 |
| B: Cancel-Priority 50ms | react 10ms | $66,834,775,284 | -0.749 | -0.734 | -0.827 | 0.827 | 0.627 | 0.627 | -0.746 | 0.569 | 0.450 | 6019/48703 | 2958171 |
| C: Uniform-Price FBA 100ms | react 10ms | $58,302,397,066 | -0.550 | -0.484 | -0.586 | 0.586 | 0.386 | 0.330 | -0.473 | 0.626 | 0.482 | 91870/0 | 2455203 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $66,834,775,284 | -0.749 | -0.734 | -0.827 | 0.827 | 0.627 | 0.627 | -0.746 | 0.569 | 0.450 | 6019/48703 | 2958171 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $67,000,875,284 | -0.818 | -0.728 | -0.817 | 0.817 | 0.617 | 0.617 | -0.818 | 0.661 | 0.459 | 5508/47250 | 2958539 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $67,098,200,284 | -0.758 | -0.737 | -0.829 | 0.829 | 0.629 | 0.629 | -0.756 | 0.565 | 0.438 | 5163/46491 | 2958639 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Informed Wait | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $168,370,440,851 | -1.432 | -1.607 | -1.668 | 1.668 | 1.468 | 1.468 | -1.424 | 0.438 | 0.482 | 79019/344 | 0 |
| B: Cancel-Priority 50ms | react 10ms | $166,662,215,851 | -0.716 | -0.770 | -0.860 | 0.860 | 0.660 | 0.660 | -0.714 | 0.560 | 0.505 | 8839/72899 | 5778300 |
| C: Uniform-Price FBA 100ms | react 10ms | $146,669,804,665 | -0.505 | -0.530 | -0.636 | 0.636 | 0.436 | 0.380 | -0.440 | 0.591 | 0.531 | 142333/0 | 4862065 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $166,662,215,851 | -0.716 | -0.770 | -0.860 | 0.860 | 0.660 | 0.660 | -0.714 | 0.560 | 0.505 | 8839/72899 | 5778300 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $166,927,290,851 | -0.778 | -0.774 | -0.858 | 0.858 | 0.658 | 0.658 | -0.778 | 0.636 | 0.516 | 8118/70944 | 5778866 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $167,094,015,851 | -0.786 | -0.787 | -0.863 | 0.863 | 0.663 | 0.663 | -0.786 | 0.643 | 0.521 | 7661/69863 | 5779001 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Informed Wait = All-In Net` (within 0.001 bps display rounding).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO | react 10ms | $164,933,965,851 | $1,975,475,000 | $1,461,000,000 | $168,370,440,851 | $0 | -1.662 | -2.736 | -0.900 |
| B: Cancel-Priority 50ms | react 10ms | $164,933,965,851 | $220,975,000 | $1,507,275,000 | $166,662,215,851 | $0 | -0.857 | -2.637 | -0.901 |
| C: Uniform-Price FBA 100ms | react 10ms | $146,669,804,665 | $0 | $0 | $146,669,804,665 | $0 | -0.636 | n/a | n/a |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 0ms) | react 10ms | $164,933,965,851 | $220,975,000 | $1,507,275,000 | $166,662,215,851 | $0 | -0.857 | -2.637 | -0.901 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 35ms) | react 10ms | $164,933,965,851 | $202,950,000 | $1,790,375,000 | $166,927,290,851 | $0 | -0.856 | -2.734 | -0.876 |
| D: Dywe IBTE + Cancel-Prio 50ms (dec 70ms) | react 10ms | $164,933,965,851 | $191,525,000 | $1,968,525,000 | $167,094,015,851 | $0 | -0.861 | -2.729 | -0.894 |

Reconciliation note: `StaleSnipe` represents orders hitting the exact un-updated quote ID targeted at trigger time; `LateSnipe` represents sniper orders arriving after MM already requoted to a fresh reference mid.

**Empirical Data Verification (Regime A, 1s Markout):** -1.668 bps, 95% CI [-1.803, -1.524]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier (MEV Scenario Assumptions)
All cells below are parameterized scenario outputs (`capture × impact × mm_loss_share × leak`), not direct empirical measurements. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | decrypt 0ms | react 10ms | mm_loss_share 50% | cancel_header_leak 10%
Rows: MEV capture rate on large orders (>=$50k) in Regime B. Columns: per-order MEV impact (bps). Verdict: D<B = Dywe cheaper for takers (95% CI < 0); B<D = Regime B cheaper; ~ = statistically indistinguishable.
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.000 ~ | 0.013 B<D | 0.026 B<D | 0.052 B<D | 0.078 B<D |
| 10% | 0.000 ~ | -0.022 D<B | -0.044 D<B | -0.089 D<B | -0.133 D<B |
| 25% | 0.000 ~ | -0.075 D<B | -0.150 D<B | -0.301 D<B | -0.452 D<B |
| 50% | 0.000 ~ | -0.163 D<B | -0.327 D<B | -0.655 D<B | -0.983 D<B |
| 75% | 0.000 ~ | -0.252 D<B | -0.504 D<B | -1.009 D<B | -1.514 D<B |
| 100% | 0.000 ~ | -0.341 D<B | -0.682 D<B | -1.364 D<B | -2.046 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | decrypt 35ms | react 10ms | mm_loss_share 50% | cancel_header_leak 10%
Rows: MEV capture rate on large orders (>=$50k) in Regime B. Columns: per-order MEV impact (bps). Verdict: D<B = Dywe cheaper for takers (95% CI < 0); B<D = Regime B cheaper; ~ = statistically indistinguishable.
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.011 ~ | 0.024 B<D | 0.037 B<D | 0.063 B<D | 0.089 B<D |
| 10% | 0.011 ~ | -0.011 ~ | -0.033 D<B | -0.078 D<B | -0.122 D<B |
| 25% | 0.011 ~ | -0.064 D<B | -0.139 D<B | -0.290 D<B | -0.441 D<B |
| 50% | 0.011 ~ | -0.152 D<B | -0.316 D<B | -0.644 D<B | -0.972 D<B |
| 75% | 0.011 ~ | -0.241 D<B | -0.493 D<B | -0.998 D<B | -1.503 D<B |
| 100% | 0.011 ~ | -0.330 D<B | -0.671 D<B | -1.353 D<B | -2.035 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | decrypt 70ms | react 10ms | mm_loss_share 50% | cancel_header_leak 10%
Rows: MEV capture rate on large orders (>=$50k) in Regime B. Columns: per-order MEV impact (bps). Verdict: D<B = Dywe cheaper for takers (95% CI < 0); B<D = Regime B cheaper; ~ = statistically indistinguishable.
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.015 B<D | 0.028 B<D | 0.041 B<D | 0.068 B<D | 0.094 B<D |
| 10% | 0.015 B<D | -0.006 ~ | -0.029 D<B | -0.073 D<B | -0.118 D<B |
| 25% | 0.015 B<D | -0.059 D<B | -0.135 D<B | -0.286 D<B | -0.436 D<B |
| 50% | 0.015 B<D | -0.148 D<B | -0.312 D<B | -0.640 D<B | -0.968 D<B |
| 75% | 0.015 B<D | -0.237 D<B | -0.489 D<B | -0.994 D<B | -1.499 D<B |
| 100% | 0.015 B<D | -0.325 D<B | -0.666 D<B | -1.348 D<B | -2.030 D<B |

## Methodological Boundaries & Scope
- Regime D execution timeline: orders arrive inside `[batch_start, batch_close)`, commit at `batch_close`, complete threshold decryption at `clear_ns = batch_close + decrypt_ms`, and clear atomically against the post-cancel book.
- 95% CI reflects empirical drift variance across 3-hour blocks under FIXED scenario assumptions; parameter uncertainty (capture, impact, leak, reaction times) is not covered by CI.
- Single 7-day sample window (one volatility regime); BTCUSDT and ETHUSDT are strongly correlated and resampled jointly.
- Taker flow is exogenous and invariant to spread width; a single representative MM clears the full flow; sniper triggers are threshold-bound.
- Threshold encryption enters the model strictly via the MEV/leakage parameter grid and post-commit decryption latency.
