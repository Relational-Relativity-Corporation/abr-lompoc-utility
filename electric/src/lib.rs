// abr-lompoc-electric — V0.1.0
// Relational analysis of the Lompoc municipal electric grid
// under the ABR/ABRCE kernel (abr-kernel).
//
// Metatron Dynamics, Inc. — relationalrelativity.dev
// Delaware C-Corp #10551645
//
// Domain boundary: Lompoc Electric distribution system, bounded at
// the PG&E transmission interconnect (the single point of wholesale
// purchase). All declared loci and edges are within this boundary.
//
// All numerical inputs are from public sources cited in data/lompoc_electric.rs.
// Every derived quantity is recomputable from those inputs by anyone
// who clones this repo and runs `cargo run --release --bin run_electric`.

pub mod domain;
pub mod observables;
pub mod pilot;
pub mod wholesale_cost;
