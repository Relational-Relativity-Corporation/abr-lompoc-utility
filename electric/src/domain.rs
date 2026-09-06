// domain.rs — Declared relational domain for Lompoc Electric
//
// The Lompoc electric grid is declared as a directed relational system D
// with the following loci (nodes) and edges (directed relations).
//
// Locus classes:
//   S  — Supply: the single wholesale purchase point (PG&E transmission interconnect)
//   T  — Transmission: PG&E-owned transmission lines feeding Lompoc
//   D  — Distribution: Lompoc-owned distribution infrastructure
//   L  — Load: customer load classes (residential, commercial, industrial)
//
// This is the structural skeleton. Anomaly detection operates on
// declared edges — a detected deviation at any edge is an observable.

#[derive(Debug, Clone, PartialEq)]
pub enum LocusClass {
    Supply,
    Transmission,
    Distribution,
    Load,
}

#[derive(Debug, Clone)]
pub struct Locus {
    pub id: &'static str,
    pub class: LocusClass,
    pub label: &'static str,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: &'static str,
    pub to:   &'static str,
    pub label: &'static str,
}

/// Declare the Lompoc electric grid domain.
/// Returns (loci, edges).
pub fn declare_domain() -> (Vec<Locus>, Vec<Edge>) {
    let loci = vec![
        Locus { id: "S1",  class: LocusClass::Supply,       label: "PG&E wholesale purchase point" },
        Locus { id: "T1",  class: LocusClass::Transmission,  label: "PG&E transmission line A" },
        Locus { id: "T2",  class: LocusClass::Transmission,  label: "PG&E transmission line B" },
        Locus { id: "D1",  class: LocusClass::Distribution,  label: "Lompoc substation / distribution entry" },
        Locus { id: "D2",  class: LocusClass::Distribution,  label: "Residential distribution network" },
        Locus { id: "D3",  class: LocusClass::Distribution,  label: "Commercial distribution network" },
        Locus { id: "D4",  class: LocusClass::Distribution,  label: "Industrial distribution network" },
        Locus { id: "L1",  class: LocusClass::Load,          label: "Residential load (14,423 accounts)" },
        Locus { id: "L2",  class: LocusClass::Load,          label: "Commercial load (2,068 accounts)" },
        Locus { id: "L3",  class: LocusClass::Load,          label: "Industrial load (25 accounts)" },
    ];

    let edges = vec![
        Edge { from: "S1", to: "T1", label: "wholesale power flow — line A" },
        Edge { from: "S1", to: "T2", label: "wholesale power flow — line B" },
        Edge { from: "T1", to: "D1", label: "transmission to Lompoc substation" },
        Edge { from: "T2", to: "D1", label: "transmission to Lompoc substation (redundant)" },
        Edge { from: "D1", to: "D2", label: "substation to residential distribution" },
        Edge { from: "D1", to: "D3", label: "substation to commercial distribution" },
        Edge { from: "D1", to: "D4", label: "substation to industrial distribution" },
        Edge { from: "D2", to: "L1", label: "residential delivery" },
        Edge { from: "D3", to: "L2", label: "commercial delivery" },
        Edge { from: "D4", to: "L3", label: "industrial delivery" },
    ];

    (loci, edges)
}

/// The single structural vulnerability in the declared domain:
/// S1 → T1 and S1 → T2 are both PG&E-controlled.
/// A PSPS event at S1 removes all power to the entire domain.
/// Lompoc Electric has zero generation assets and cannot island.
pub fn psps_vulnerability() -> &'static str {
    "PSPS EXPOSURE: All power flows through PG&E transmission (S1→T1, S1→T2). \
     Lompoc Electric owns zero generation assets. A PG&E PSPS event \
     removes supply at S1 — the entire domain goes dark. \
     The city has no operational recourse."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_has_correct_locus_count() {
        let (loci, _) = declare_domain();
        assert_eq!(loci.len(), 10, "Expected 10 declared loci");
    }

    #[test]
    fn domain_has_correct_edge_count() {
        let (_, edges) = declare_domain();
        assert_eq!(edges.len(), 10, "Expected 10 declared edges");
    }

    #[test]
    fn supply_locus_exists() {
        let (loci, _) = declare_domain();
        let supply: Vec<_> = loci.iter().filter(|l| l.class == LocusClass::Supply).collect();
        assert_eq!(supply.len(), 1, "Expected exactly 1 supply locus (single wholesale point)");
    }

    #[test]
    fn all_load_loci_have_incoming_edges() {
        let (_, edges) = declare_domain();
        for load_id in &["L1", "L2", "L3"] {
            let has_incoming = edges.iter().any(|e| e.to == *load_id);
            assert!(has_incoming, "Load locus {} has no incoming edge", load_id);
        }
    }

    #[test]
    fn transmission_loci_all_originate_from_supply() {
        let (_, edges) = declare_domain();
        let t_edges: Vec<_> = edges.iter().filter(|e| e.to == "T1" || e.to == "T2").collect();
        // T1 and T2 receive from S1 only — there are no other supply sources
        // (Actually in our model S1→T1 and S1→T2 are the supply edges)
        let supply_to_t: Vec<_> = edges.iter()
            .filter(|e| e.from == "S1" && (e.to == "T1" || e.to == "T2"))
            .collect();
        assert_eq!(supply_to_t.len(), 2, "Both transmission lines must originate at S1");
        let _ = t_edges; // used above
    }
}
