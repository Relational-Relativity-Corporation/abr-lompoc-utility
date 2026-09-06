# abr-lompoc-utility

**ABR/ABRCE relational analysis of Lompoc municipal utilities — electric grid, water distribution, MDM simulation, and BESS integration.**

Metatron Dynamics, Inc. — [relationalrelativity.dev](https://relationalrelativity.dev)  
Delaware C-Corp #10551645

---

## What this repository is

This repository contains a suite of relational analyses of the City of Lompoc's municipal utility infrastructure — electric and water — under the ABR/ABRCE kernel. Each subdirectory is an independently verifiable declared domain. Every numerical claim is recomputable from cited public sources by anyone who clones the repo and runs `cargo test` and `cargo run`.

This work was developed in direct engagement with Lompoc Utilities as a proposed framework for improving grid monitoring, load management, and meter data processing at lower cost than conventional vendor solutions.

---

## Repository structure

```
electric/       — Relational domain of the Lompoc electric grid
mdm-sim/        — Relational MDM simulation covering electric + water endpoints
water/          — Relational domain of the Lompoc water distribution system (in development)
bess/           — Battery storage integration analysis (planned)
```

---

## electric/

**Declared domain:** Lompoc Electric distribution system, bounded at the PG&E transmission interconnect.

**Key findings (from public data):**
- 16,516 customer endpoints (14,423 residential, 2,068 commercial, 25 industrial)
- 3.30% implied distribution loss visible in public EIA data before any instrumentation
- 100% PSPS exposure — all power flows through PG&E transmission; Lompoc owns zero generation assets
- Annual savings from 2–5% loss reduction: **$781K – $1.95M/yr**
- PSPS revenue exposure: **$130K/yr** (conservative, 48 hrs/yr assumed)

**Run it:**
```
cd electric
cargo test --release
cargo run --release --bin run_electric
```

---

## mdm-sim/

**Declared domain:** The meter-to-bill process as a directed relational system (M → H → V → A → S → B), covering both electric (~16,516) and water (~16,000) endpoints — 32,516 total.

**What it simulates:** A 5-year total cost of ownership comparison between conventional vendor MDM platforms and a Metatron relational MDM layer, with all quantities provenance-classified as observed, derived, scenario-assumed, interpolated, or open.

**Verifier status:** PASS — V0.1.2  
39/39 tests. Quantities are provenance-classified; no scenario output is represented as an empirical finding.

**Key scenario outputs (contingent on pilot confirmation):**
- Scenario cost difference vs. low-tier vendor SaaS: **$343K over 5 years**
- Scenario cost difference vs. mid-tier (Itron IEE): **$1.23M over 5 years**
- Metatron 5-yr TCO: **$195K** (Year 1 pilot at zero license cost)

**ABR operator:** Δ applied at locus A (AnomalyDetection) on validated cumulative register readings. Anomaly classification uses declared class bounds derived from the load profile — no statistical projection.

**Open conditions resolved by the 90-day pilot:**
- OC-MDM-1: Actual water meter count (assumed 16,000)
- OC-MDM-3: Actual billing error rate (assumed 0.5% of revenue)
- OC-MDM-4: Metatron post-pilot cost structure
- OC-MDM-5: Actual per-class AMI consumption distributions

**Run it:**
```
cd mdm-sim
cargo test --release
cargo run --release --bin run_mdm_sim
```

---

## Requirements

Rust stable toolchain (developed against 1.75+). No external dependencies.

---

## The 90-day pilot

The proposed pilot deploys the relational MDM layer on live Lompoc AMI data and replaces all scenario assumptions with actual observables. The pilot produces a verifiable declared result before any infrastructure commitment is required. Every open condition in the MDM simulation is explicitly assigned to pilot closure.

Contact: [relationalrelativity.dev](https://relationalrelativity.dev)

---

*Bounded over D, per the governing kernel (abr-kernel). No claim is made beyond D. All findings specific to declared domain and boundary conditions stated in each subdirectory.*
