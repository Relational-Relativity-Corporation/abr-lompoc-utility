// domain.rs — Declared Relational Domain for Lompoc MDM Simulation
//
// The MDM process is declared as a directed relational system D.
// Each locus is a declared process stage. Each edge is a declared
// relation between stages — the direction is the direction of data
// flow through the meter-to-bill process, which is the observable
// property of the process that establishes ordering (V7 relational
// evolution direction requirement).
//
// Locus classes:
//   M  — Meter: physical endpoint (source of observable reads)
//   H  — HeadEnd: AMI head-end system receiving raw reads
//   V  — VEE: validation, estimation, editing process
//   S  — Storage: processed read repository
//   B  — Billing: bill generation from validated reads
//   A  — Anomaly: relational anomaly detection layer (ABR operator site)
//
// The A locus is where ABR operators act. It sits on the edge between
// V (VEE) and S (Storage) — reading the validated interval data stream
// and applying declared relational operators to detect anomalies.
// This is the precise location where M → D → operator application occurs.

#[derive(Debug, Clone, PartialEq)]
pub enum LocusClass {
    Meter,
    HeadEnd,
    VEE,
    Storage,
    Billing,
    AnomalyDetection,
}

#[derive(Debug, Clone)]
pub struct Locus {
    pub id:    &'static str,
    pub class: LocusClass,
    pub label: &'static str,
    pub abr_operator_site: bool,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from:      &'static str,
    pub to:        &'static str,
    pub label:     &'static str,
    pub direction: &'static str, // observable property establishing direction
}

pub fn declare_domain() -> (Vec<Locus>, Vec<Edge>) {
    let loci = vec![
        Locus { id: "M",  class: LocusClass::Meter,
            label: "Smart meter endpoints (16,516 electric + ~16,000 water)",
            abr_operator_site: false },
        Locus { id: "H",  class: LocusClass::HeadEnd,
            label: "AMI head-end: raw interval read collection",
            abr_operator_site: false },
        Locus { id: "V",  class: LocusClass::VEE,
            label: "VEE: validation, estimation, editing of raw reads",
            abr_operator_site: false },
        Locus { id: "A",  class: LocusClass::AnomalyDetection,
            label: "ABR relational anomaly detection (Metatron MDM layer)",
            abr_operator_site: true },
        Locus { id: "S",  class: LocusClass::Storage,
            label: "Storage: validated, anomaly-flagged read repository",
            abr_operator_site: false },
        Locus { id: "B",  class: LocusClass::Billing,
            label: "Billing: bill generation from validated reads",
            abr_operator_site: false },
    ];

    let edges = vec![
        Edge { from: "M", to: "H",
            label: "interval read transmission",
            direction: "meter → head-end (radio/network, one direction)" },
        Edge { from: "H", to: "V",
            label: "raw reads passed to VEE",
            direction: "collection precedes validation (process order)" },
        Edge { from: "V", to: "A",
            label: "validated reads passed to ABR anomaly layer",
            direction: "validation precedes relational detection (process order)" },
        Edge { from: "A", to: "S",
            label: "anomaly-flagged reads committed to storage",
            direction: "detection precedes storage (process order)" },
        Edge { from: "S", to: "B",
            label: "validated reads pulled for billing",
            direction: "storage precedes billing (process order)" },
    ];

    (loci, edges)
}

/// The ABR operator site: locus A on edge V→A→S
/// This is where Δ (change detection) and the relational field operators
/// act on the validated interval data stream from M through H and V.
/// The input to A is M(o) — the declared observation from the meter
/// endpoint, validated by VEE, arriving as a declared sequence of
/// interval reads. This satisfies V7 sequential observation requirement.
pub fn abr_operator_site() -> &'static str {
    "Locus A (AnomalyDetection) on edge V→A→S. \
     Input: validated interval reads — declared sequence {M(o₁),...,M(oₙ)} \
     where each oₖ is a meter read at one declared billing interval. \
     Direction: process order (validation precedes detection precedes storage). \
     Operators: Δ applied across declared adjacency relations between \
     consecutive interval reads on each declared meter edge."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exactly_one_abr_operator_site() {
        let (loci, _) = declare_domain();
        let sites: Vec<_> = loci.iter().filter(|l| l.abr_operator_site).collect();
        assert_eq!(sites.len(), 1, "Exactly one ABR operator site must be declared");
        assert_eq!(sites[0].id, "A");
    }

    #[test]
    fn all_edges_have_direction_declared() {
        let (_, edges) = declare_domain();
        for e in &edges {
            assert!(!e.direction.is_empty(),
                "Edge {}→{} must have declared direction", e.from, e.to);
        }
    }

    #[test]
    fn meter_locus_has_no_incoming_edges() {
        let (_, edges) = declare_domain();
        let incoming_to_m = edges.iter().filter(|e| e.to == "M").count();
        assert_eq!(incoming_to_m, 0,
            "Meter locus M is the source — no incoming edges");
    }

    #[test]
    fn billing_locus_has_no_outgoing_edges() {
        let (_, edges) = declare_domain();
        let outgoing_from_b = edges.iter().filter(|e| e.from == "B").count();
        assert_eq!(outgoing_from_b, 0,
            "Billing locus B is the sink — no outgoing edges");
    }

    #[test]
    fn abr_site_is_on_validated_reads_edge() {
        let (_, edges) = declare_domain();
        let v_to_a = edges.iter().any(|e| e.from == "V" && e.to == "A");
        let a_to_s = edges.iter().any(|e| e.from == "A" && e.to == "S");
        assert!(v_to_a, "Edge V→A must be declared");
        assert!(a_to_s, "Edge A→S must be declared");
    }

    #[test]
    fn domain_has_six_loci() {
        let (loci, _) = declare_domain();
        assert_eq!(loci.len(), 6);
    }

    #[test]
    fn domain_has_five_edges() {
        let (_, edges) = declare_domain();
        assert_eq!(edges.len(), 5);
    }
}
