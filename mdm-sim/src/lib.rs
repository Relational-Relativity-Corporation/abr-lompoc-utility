// abr-lompoc-mdm-sim — V0.1.0
// Relational MDM simulation for Lompoc Utilities
// under the ABR/ABRCE kernel (abr-kernel).
//
// Metatron Dynamics, Inc. — relationalrelativity.dev
// Delaware C-Corp #10551645
//
// Module order reflects the kernel's required sequence:
//   1. measurement  — M declared first (O → D mapping)
//   2. domain       — relational domain D declared
//   3. operators    — ABR operators declared and applied at locus A
//   4. simulation   — TCO comparison derived from 1–3

pub mod measurement;
pub mod domain;
pub mod operators;
pub mod simulation;
