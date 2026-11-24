# Dywe Microstructure Ablation — Scenario-Based Causal Backtest (v0.3.5)
Window: 2026-11-10 + 7d | Pairs: SOLUSDT | Bootstrap Block: 3h (56 blocks) | Runs per (Pair, Day): 18 | B 70ms (hl), D 50ms (hl, exp=all), FBA 100ms | requote_thr=1.500bp, sniper_thr=2.000bp | pending=drop | src=auto | mid=bidask | wait=all | sniper_clip=$0 | rebate=0.500bp (paid to MM), opex=0.300bp, large>=$50,000 | placebo shifts (s): [600, 3600]

Pair-Days Processed: 7 of 7.

**TAKER FLOW DIAGNOSTICS:** Historical taker flow diagnostics (pure exchange volume $12,631,910,075, threshold >=$50,000): unmerged raw rows = 2035536 (49553 large rows, 39.8% of USD volume); 1-ms directional bursts (`merge_taker`) = 1997052 (46731 large bursts, 42.4% of USD volume).

**PROXY NOTE:** 7 pair-days used a trade-derived midpoint proxy (`--mid-source auto`, mode `bidask`). BidAskProxy diagnostics: 15365789 total observations, 1305 crossed (0.0% where ask < bid), mean valid (ask - bid) spread = 1.903 bps.

### SOLUSDT (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $12,629,641,501 | $12,629,641,501 | -0.997 | -1.494 | -2.011 | 2.011 | 1.811 | 1.811 | -0.997 | 0.519 | 1.332 | 0/0 | 137 (0.017%) | 0 |
| B: Batch 70ms (hl), react 10ms | $12,628,398,693 | $12,628,398,693 | -0.612 | -0.938 | -1.400 | 1.400 | 1.200 | 1.200 | -0.612 | 0.753 | 1.341 | 0/0 | 200 (0.027%) | 1649862 |
| C: Uniform-Price FBA 100ms, react 10ms | $11,440,088,430 | $12,628,908,474 | -0.595 | -0.900 | -1.384 | 1.384 | 1.184 | 1.072 | -0.539 | 0.824 | 1.357 | 0/0 | 193 (0.023%) | 1569058 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $12,626,871,546 | $12,626,871,546 | -0.686 | -1.044 | -1.524 | 1.524 | 1.324 | 1.324 | -0.686 | 0.699 | 1.338 | 0/0 | 311 (0.039%) | 1719860 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $12,626,871,546 | $12,626,871,546 | -0.881 | -1.097 | -1.533 | 1.533 | 1.333 | 1.333 | -0.881 | 0.894 | 1.346 | 0/0 | 311 (0.039%) | 1719860 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $12,626,871,546 | $12,626,871,546 | -0.988 | -1.147 | -1.533 | 1.533 | 1.333 | 1.333 | -0.989 | 1.002 | 1.346 | 0/0 | 311 (0.039%) | 1719860 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair (MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $12,629,641,501 | $12,629,641,501 | -0.997 | -1.494 | -2.011 | 2.011 | 1.811 | 1.811 | -0.997 | 0.519 | 1.332 | 0/0 | 137 (0.017%) | 0 |
| B: Batch 70ms (hl), react 10ms | $12,628,398,693 | $12,628,398,693 | -0.612 | -0.938 | -1.400 | 1.400 | 1.200 | 1.200 | -0.612 | 0.753 | 1.341 | 0/0 | 200 (0.027%) | 1649862 |
| C: Uniform-Price FBA 100ms, react 10ms | $11,440,088,430 | $12,628,908,474 | -0.595 | -0.900 | -1.384 | 1.384 | 1.184 | 1.072 | -0.539 | 0.824 | 1.357 | 0/0 | 193 (0.023%) | 1569058 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $12,626,871,546 | $12,626,871,546 | -0.686 | -1.044 | -1.524 | 1.524 | 1.324 | 1.324 | -0.686 | 0.699 | 1.338 | 0/0 | 311 (0.039%) | 1719860 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $12,626,871,546 | $12,626,871,546 | -0.881 | -1.097 | -1.533 | 1.533 | 1.333 | 1.333 | -0.881 | 0.894 | 1.346 | 0/0 | 311 (0.039%) | 1719860 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $12,626,871,546 | $12,626,871,546 | -0.988 | -1.147 | -1.533 | 1.533 | 1.333 | 1.333 | -0.989 | 1.002 | 1.346 | 0/0 | 311 (0.039%) | 1719860 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Consolidated Multi-Pair — MM Flow Cohort Decomposition & Volume Reconciliation
| Regime | Taker Vol | StaleSnipe Vol | LateSnipe Vol | Total MM Vol | Vol Diff | Taker 1s Markout | StaleSnipe 1s Markout | LateSnipe 1s Markout | 1-ms Burst Vol % (>=$50,000) | Cancel-Batch Large % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms | $12,629,641,501 | $0 | $0 | $12,629,641,501 | $0 | -2.011 | n/a | n/a | 42.4% | 0.0% |
| B: Batch 70ms (hl), react 10ms | $12,628,398,693 | $0 | $0 | $12,628,398,693 | $0 | -1.400 | n/a | n/a | 42.4% | 41.5% |
| C: Uniform-Price FBA 100ms, react 10ms | $11,440,088,430 | $0 | $0 | $11,440,088,430 | $0 | -1.384 | n/a | n/a | 42.4% | 0.0% |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms | $12,626,871,546 | $0 | $0 | $12,626,871,546 | $0 | -1.524 | n/a | n/a | 42.4% | 35.1% |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms | $12,626,871,546 | $0 | $0 | $12,626,871,546 | $0 | -1.533 | n/a | n/a | 42.4% | 35.1% |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms | $12,626,871,546 | $0 | $0 | $12,626,871,546 | $0 | -1.533 | n/a | n/a | 42.4% | 35.1% |

Reconciliation note: `Taker Vol` reflects historical exchange flow (`aggTrades`/`trades`); `StaleSnipe` and `LateSnipe` reflect the parameterized latency-arb probe (`--sniper-clip`, default $25,000). `1-ms Burst Vol %` reflects directional trades merged within the same millisecond (`merge_taker`); see Raw Taker Diagnostics above for the unmerged lower bound.

## Placebo test: circularly shifted taker flow
The taker flow is moved in time by the shift (wrapping inside each day) while the price path is untouched. Counts, sizes and sides are preserved; temporal alignment between flow and subsequent price moves is destroyed.

### Placebo: takers shifted 600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 600s | $12,631,505,842 | $12,631,505,842 | 0.000 | -0.001 | -0.006 | 0.006 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 0/0 | 32 (0.003%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 600s | $12,631,749,394 | $12,631,749,394 | -0.002 | -0.002 | -0.008 | 0.008 | <0 | 0.000 | -0.002 | 0.001 | 0.000 | 0/0 | 33 (0.001%) | 1739635 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 600s | $11,442,210,029 | $12,631,560,943 | -0.002 | -0.003 | -0.007 | 0.007 | <0 | 0.000 | -0.001 | 0.001 | 0.000 | 0/0 | 52 (0.002%) | 1653245 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 600s | $12,631,719,550 | $12,631,719,550 | 0.000 | -0.002 | -0.006 | 0.006 | <0 | 0.000 | 0.000 | 0.000 | 0.000 | 0/0 | 49 (0.001%) | 1809599 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 600s | $12,631,719,550 | $12,631,719,550 | -0.001 | -0.002 | -0.006 | 0.006 | <0 | 0.000 | -0.001 | 0.001 | 0.000 | 0/0 | 49 (0.001%) | 1809599 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 600s | $12,631,719,550 | $12,631,719,550 | -0.001 | 0.000 | -0.007 | 0.007 | <0 | 0.000 | -0.001 | 0.001 | 0.000 | 0/0 | 49 (0.001%) | 1809599 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Placebo: takers shifted 3600s (consolidated, MEV = 0)
| Regime | MM Total Vol (incl. Probe) | Hist Taker Vol | Markout 0ms | 100ms | 1s | s* gross | s* net | [1] Spread Paid | [2] Taker Stale | [3] Wait (All) | [=] All-In Net ([1]+[2]+[3]) | StaleSnipe Hit/Miss | Dropped Takers (USD%) | Batches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| A: Continuous FIFO, react 10ms, PLACEBO shift 3600s | $12,631,657,384 | $12,631,657,384 | -0.002 | -0.004 | -0.008 | 0.008 | <0 | 0.000 | -0.002 | 0.000 | -0.002 | 0/0 | 38 (0.002%) | 0 |
| B: Batch 70ms (hl), react 10ms, PLACEBO shift 3600s | $12,631,636,098 | $12,631,636,098 | -0.004 | -0.006 | -0.010 | 0.010 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 0/0 | 37 (0.002%) | 1741684 |
| C: Uniform-Price FBA 100ms, react 10ms, PLACEBO shift 3600s | $11,443,708,710 | $12,631,701,046 | -0.006 | -0.008 | -0.012 | 0.012 | <0 | 0.000 | -0.006 | 0.001 | -0.004 | 0/0 | 38 (0.001%) | 1655157 |
| D: Dywe IBTE 50ms (dec 0ms, hl), react 10ms, PLACEBO shift 3600s | $12,631,666,642 | $12,631,666,642 | -0.003 | -0.006 | -0.010 | 0.010 | <0 | 0.000 | -0.003 | 0.000 | -0.003 | 0/0 | 35 (0.001%) | 1810797 |
| D: Dywe IBTE 50ms (dec 35ms, hl), react 10ms, PLACEBO shift 3600s | $12,631,666,642 | $12,631,666,642 | -0.004 | -0.007 | -0.011 | 0.011 | <0 | 0.000 | -0.004 | 0.001 | -0.003 | 0/0 | 35 (0.001%) | 1810797 |
| D: Dywe IBTE 50ms (dec 70ms, hl), react 10ms, PLACEBO shift 3600s | $12,631,666,642 | $12,631,666,642 | -0.006 | -0.006 | -0.012 | 0.012 | <0 | 0.000 | -0.006 | 0.002 | -0.003 | 0/0 | 35 (0.001%) | 1810797 |

Units: bps. Exact identity at MEV=0: `[1] Spread Paid + [2] Taker Stale + [3] Wait = All-In Net` (within 0.001 bps display rounding; wait_cohort=`all`; requote_thr=`1.500 bp`, sniper_thr=`2.000 bp`). Note: under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net (check `Dropped Takers (USD%)` for selection bias).

### Real versus placebo cancel-priority gap (1s markout, positive = cancel-priority helps the MM)
- Real flow (react 10ms): A -2.011 bp, B -1.400 bp, cancel-priority gap (B minus A) 0.611 bp.
- Placebo shift 600 s: A -0.006 bp, B -0.008 bp, gap -0.001 bp = -0.3% of the real gap.
- Placebo shift 3600 s: A -0.008 bp, B -0.010 bp, gap -0.001 bp = -0.2% of the real gap.
- Reading: circular shifting tests whether markout stems from temporal alignment between flow and subsequent price moves rather than static drift; it does not validate midpoint reconstruction accuracy.

### Cancel-priority gap versus reaction handicap / window
- react 10ms: handicap (MM minus sniper reaction) 8ms, B window 70ms, handicap/window 11%, gap 0.611 bp = 30.3% of A's 1s markout magnitude.

**Empirical Data Verification (Regime A, 1s Markout):** -2.011 bps, 95% CI [-2.168, -1.858]. Expected negative sign confirmed.

## Phase 2: Break-Even Frontier & Break-Even Leakage (MEV Scenario Assumptions)
All cells with capture above 0% are parameterized scenario outputs (`capture × impact × mm_loss_share × eff_leak`, where `eff_leak = min(leak, capture)`), not measurements. Exposure mode for Regime D: `all`. `breakeven.csv` contains the full 6×5×4×4 grid per decryption delay.

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 0ms, hl, exp=all), react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ |
| 10% | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ | -0.002 ~ |
| 25% | -0.002 ~ | -0.034 D<B | -0.066 D<B | -0.130 D<B | -0.193 D<B |
| 50% | -0.002 ~ | -0.087 D<B | -0.172 D<B | -0.342 D<B | -0.512 D<B |
| 75% | -0.002 ~ | -0.140 D<B | -0.278 D<B | -0.554 D<B | -0.830 D<B |
| 100% | -0.002 ~ | -0.193 D<B | -0.384 D<B | -0.766 D<B | -1.148 D<B |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 35ms, hl, exp=all), react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D |
| 10% | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D | 0.005 B<D |
| 25% | 0.005 B<D | -0.026 D<B | -0.058 D<B | -0.121 D<B | -0.185 D<B |
| 50% | 0.005 B<D | -0.079 D<B | -0.164 D<B | -0.334 D<B | -0.503 D<B |
| 75% | 0.005 B<D | -0.132 D<B | -0.270 D<B | -0.546 D<B | -0.822 D<B |
| 100% | 0.005 B<D | -0.185 D<B | -0.376 D<B | -0.758 D<B | -1.140 D<B |

#### Break-Even Residual Leakage `eff_leak*` (where All-In(D, dec 35ms) == All-In(B, 70ms))
Dywe is cheaper for takers (`D < B`) whenever actual residual leakage `eff_leak < eff_leak*` (note `eff_leak <= capture` by definition; values `>= capture` mean D stays cheaper for any `leak <= capture`).
| Ambient Capture / Impact | Cancel-Batch (35.1%) | 50.0% Exposed | 75.0% Exposed | 100.0% Exposed (All) |
|---|---:|---:|---:|---:|
| 10% / 0.500 bp | 21.2% | 14.9% | 9.9% | 7.4% |
| 10% / 1.000 bp | 24.8% | 17.4% | 11.6% | 8.7% |
| 25% / 1.000 bp | 67.4% | 47.4% | 31.6% | 23.7% |
| 50% / 1.000 bp | >100.0% | 97.4% | 64.9% | 48.7% |
| 50% / 2.000 bp | >100.0% | 98.7% | 65.8% | 49.3% |

#### Break-Even Frontier: All-In(D) − All-In(B) (bps) | B 70ms (hl), D 50ms (dec 70ms, hl, exp=all), react 10ms | pending=drop | src=auto | wait=all
Rows: MEV capture rate on 1-ms bursts (>=$50,000) in Regime B. Columns: per-order MEV impact (bps). Fixed slice: 50% MM share, 10% leak (`eff_leak = min(leak, capture)`). Verdict: `D<B` = Dywe cheaper (95% CI < 0); `B<D` = Regime B cheaper; `~` = indistinguishable. **Every cell except 0%-capture is a parameterized scenario output, not a measurement.**
| Capture \ Impact | 0.000 bp | 0.500 bp | 1.000 bp | 2.000 bp | 3.000 bp |
|---|---:|---:|---:|---:|---:|
| 0% | 0.005 ~ | 0.005 ~ | 0.005 ~ | 0.005 ~ | 0.005 ~ |
| 10% | 0.005 ~ | 0.005 ~ | 0.005 ~ | 0.005 ~ | 0.005 ~ |
| 25% | 0.005 ~ | -0.026 D<B | -0.057 D<B | -0.121 D<B | -0.185 D<B |
| 50% | 0.005 ~ | -0.079 D<B | -0.164 D<B | -0.333 D<B | -0.503 D<B |
| 75% | 0.005 ~ | -0.132 D<B | -0.270 D<B | -0.546 D<B | -0.821 D<B |
| 100% | 0.005 ~ | -0.185 D<B | -0.376 D<B | -0.758 D<B | -1.140 D<B |

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
- MM requote threshold: `--requote-thr-sub 150` (1.500 bp); synthetic sniper probe threshold: `--sniper-thr-sub 200` (2.000 bp), clip: `$0` (`--sniper-clip 0` disables probe).
- Single-denominator All-In Net: `--wait-cohort all` (`all` uses total executed taker volume `tk_n` across all 4 components).
- Under `pending=drop`, dropped takers are excluded from both MM markout and All-In Net; inspect `Dropped Takers (USD%)` (0.001% precision) before comparing costs.
- 95% CI reflects block-resampling drift variance under fixed scenario assumptions; volatility autocorrelation and parameter uncertainty are not covered.
- Exogenous taker flow; single representative MM; model-derived `s*net` is not directly comparable to real exchange tick spreads.
