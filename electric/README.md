# abr-lompoc-electric

**ABR/ABRCE relational analysis of the Lompoc municipal electric grid —
declared domain structure, load observables, and independently verifiable
savings estimates.**

Metatron Dynamics, Inc. — [relationalrelativity.dev](https://relationalrelativity.dev)  
Delaware C-Corp #10551645

---

## What this repository is

This repository declares the Lompoc electric distribution system as a
relational domain under the ABR/ABRCE kernel, derives quantified observables
from public data, and proposes a 90-day pilot with a pre-declared success
criterion.

Every number in this repository is recomputable by anyone who clones it
and runs two commands:

```
cargo test --release
cargo run --release --bin run_electric
```

No trust required. The inputs are cited. The derivations are in the code.

---

## What the analysis shows

**Domain structure:** Lompoc Electric is a single-point-of-entry system.
All power flows through PG&E transmission lines (S1→T1, S1→T2). Lompoc
owns zero generation assets. A PG&E PSPS event removes supply at S1 —
the entire distribution domain goes dark with no city recourse.

**Declared inputs (EIA Form 861 / public data, 2019 actuals):**
- 16,516 customers (14,423 residential, 2,068 commercial, 25 industrial)
- 130,286 MWh retail sales
- 134,737 MWh total disposition
- $23,686,000 total revenue
- Current residential rate: $0.29/kWh (EnergySage, March 2026)

**Implied distribution loss:** 4,451 MWh — 3.30% of total disposition —
visible directly in public EIA data before any instrumentation is deployed.

**Annual savings estimate — loss reduction:**

| Reduction | MWh recovered | Dollars saved |
|-----------|--------------|---------------|
| 2%        | 2,695 MWh/yr | $781,475/yr   |
| 5%        | 6,737 MWh/yr | $1,953,686/yr |

Industry benchmark: 2–5% distribution loss reduction from load optimization.
Applied to Lompoc's declared total disposition at the 2026 residential rate.

**PSPS revenue exposure:** $129,786/yr at risk (conservative — 48 hrs/yr
shutoff assumed). ABR domain monitoring surfaces anomalies during and after
PSPS restoration, supporting faster grid return-to-normal.

---

## The 90-day pilot

**What the pilot does:**
Declares observable quantities on the Lompoc distribution edges,
establishes a baseline load profile from public data, and detects
deviations in real metered throughput against that baseline.

**Pre-declared success criterion:**
At least one load anomaly detected on a declared distribution edge
(D1→D2, D1→D3, or D1→D4) that exceeds the 10% deviation threshold
and is subsequently confirmed by utility operations data. Detection
is declared before measurement — not fitted to results.

**Why this addresses the "no prior installations" concern:**
The pilot produces a verifiable declared result before any infrastructure
commitment is required. The repository itself is the proof of methodology.
Lompoc Utilities staff can reproduce every result independently with access
to metered data.

**Deliverable:** Public repository with declared domain, declared baseline,
declared detection criterion, and full measurement record.

---

## Repository structure

```
src/
  lib.rs              — module declarations
  domain.rs           — declared loci and edges (the grid topology)
  observables.rs      — load anomaly detection and loss reduction calculations
  pilot.rs            — 90-day pilot quantified case
  bin/
    run_electric.rs   — full declaration report (cargo run --release --bin run_electric)
data/
  lompoc_electric.rs  — declared inputs with sources cited inline
docs/
  (see run_electric output for full declaration)
```

---

## Requirements

Rust stable toolchain (developed against 1.75+). No other dependencies.

```
cargo build
cargo test
cargo run --release --bin run_electric
```

---

## Wholesale cost analysis — load shifting opportunity

Lompoc buys 100% of its power wholesale through NCPA in the CAISO NP15 zone, at prices that vary significantly by hour. Lompoc sells to customers at a flat rate with no time-of-use structure (confirmed from published rate schedules, September 2026).

This creates a hidden cost: Lompoc pays a **65% premium** for peak-hour power ($38/MWh) vs. off-peak ($23/MWh) but charges every customer the same $290/MWh regardless of when they consume.

Shifting load from peak to off-peak hours reduces wholesale purchase cost with no change to retail revenue:

| Load shifted | MWh/yr | Wholesale saving |
|-------------|--------|-----------------|
| 5% of peak  | 3,909  | $58,629/yr      |
| 10% of peak | 7,817  | $117,257/yr     |
| 20% of peak | 15,634 | $234,515/yr     |

Lompoc already has **Schedule CUR-1** (Firm Curtailable Load) — an existing demand response mechanism for large customers willing to accept interruption during peak hours in exchange for a rate discount. No new rate action is required to activate this lever.

**What AMI + Metatron MDM enables:** Without AMI, Lompoc cannot see which feeders are peaking or when. With the relational monitoring layer on live AMI data, peak demand is detected in real time on declared edges, CUR-1 dispatch can be targeted to actual peak feeders, and load shift savings become measurable rather than scenario outputs.

Run `cargo run --release --bin run_electric` for the full quantified analysis.

## Related repositories

- [abr-lompoc-recycling](https://github.com/Relational-Relativity-Corporation/abr-lompoc-recycling) — landfill plastics recovery analysis
- [abr-community-grid-match](https://github.com/Relational-Relativity-Corporation/abr-community-grid-match) — Lompoc community grid matching analysis
- [abr-grid-integration](https://github.com/Relational-Relativity-Corporation/abr-grid-integration) — grid integration physics and BESS analysis

---

*Bounded over D, per the governing kernel (abr-kernel). No claim beyond D.
All findings are specific to the declared domain and boundary conditions stated
in this repository.*
