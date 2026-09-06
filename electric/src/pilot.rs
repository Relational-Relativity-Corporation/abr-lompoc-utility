// pilot.rs — 90-Day Pilot Proposal: Quantified Case
//
// This module computes the quantified case for a 90-day ABR pilot
// on the Lompoc electric distribution system.
//
// The pilot declares observable quantities on the distribution edges
// of the domain, establishes a baseline load profile from public data,
// and provides a pre-declared criterion for what constitutes a
// detectable result within the pilot window.
//
// Luther concern addressed: no prior installations.
// Response: the pilot produces a verifiable declared result
// before any infrastructure commitment is required.
// The repository itself is the proof of methodology.

use crate::observables::{
    LossReductionSavings, LoadProfile,
    TOTAL_DISPOSITION_MWH, RATE_2026_USD_PER_KWH, RETAIL_SALES_MWH,
};

/// Annual savings range from ABR load optimization
/// at industry benchmark 2–5% loss reduction
#[derive(Debug)]
pub struct AnnualSavingsRange {
    pub low_fraction:  f64,
    pub high_fraction: f64,
    pub low_savings:   LossReductionSavings,
    pub high_savings:  LossReductionSavings,
}

impl AnnualSavingsRange {
    pub fn compute() -> Self {
        AnnualSavingsRange {
            low_fraction:  0.02,
            high_fraction: 0.05,
            low_savings:   LossReductionSavings::compute(TOTAL_DISPOSITION_MWH, 0.02, RATE_2026_USD_PER_KWH),
            high_savings:  LossReductionSavings::compute(TOTAL_DISPOSITION_MWH, 0.05, RATE_2026_USD_PER_KWH),
        }
    }
}

/// PSPS revenue exposure: estimated annual revenue at risk from
/// PG&E transmission shutoffs. Based on average PSPS duration
/// and Lompoc total revenue.
/// 
/// Inputs:
///   Annual revenue: $23,686,000 (2019 actuals)
///   Assumed PSPS hours/year: 48 hours (conservative — 2 events × 24hr)
///   Hours in year: 8,760
#[derive(Debug)]
pub struct PspsExposure {
    pub annual_revenue_usd:    f64,
    pub psps_hours_assumed:    f64,
    pub hours_per_year:        f64,
    pub revenue_at_risk_usd:   f64,
    pub note:                  &'static str,
}

impl PspsExposure {
    pub fn compute() -> Self {
        let annual_revenue = 23_686_000.0_f64;
        let psps_hours = 48.0_f64;
        let hours_per_year = 8_760.0_f64;
        let revenue_at_risk = annual_revenue * (psps_hours / hours_per_year);
        PspsExposure {
            annual_revenue_usd:  annual_revenue,
            psps_hours_assumed:  psps_hours,
            hours_per_year,
            revenue_at_risk_usd: revenue_at_risk,
            note: "Conservative estimate — actual PSPS duration varies by year and fire conditions. \
                   ABR domain monitoring surfaces load anomalies during and after PSPS restoration.",
        }
    }
}

/// Pilot declaration: what the 90-day pilot produces
/// and what constitutes a verifiable result
#[derive(Debug)]
pub struct PilotDeclaration {
    pub duration_days:         u32,
    pub declared_domain_loci:  u32,
    pub declared_domain_edges: u32,
    pub baseline_source:       &'static str,
    pub detection_threshold_pct: f64,
    pub success_criterion:     &'static str,
    pub deliverable:           &'static str,
}

impl PilotDeclaration {
    pub fn declare() -> Self {
        PilotDeclaration {
            duration_days: 90,
            declared_domain_loci: 10,
            declared_domain_edges: 10,
            baseline_source: "EIA Form 861 / public Lompoc Utilities data (2019 actuals)",
            detection_threshold_pct: 10.0,
            success_criterion:
                "At least one load anomaly detected on a declared distribution edge \
                 (D1→D2, D1→D3, or D1→D4) that exceeds the 10% deviation threshold \
                 and is subsequently confirmed by utility operations data. \
                 Detection is declared before measurement — not fitted to results.",
            deliverable:
                "Public repository with declared domain, declared baseline, \
                 declared detection criterion, and full measurement record. \
                 Independently reproducible by Lompoc Utilities staff or any \
                 third party with access to metered data.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annual_savings_low_is_positive() {
        let range = AnnualSavingsRange::compute();
        assert!(range.low_savings.dollars_saved > 0.0);
    }

    #[test]
    fn annual_savings_high_exceeds_low() {
        let range = AnnualSavingsRange::compute();
        assert!(range.high_savings.dollars_saved > range.low_savings.dollars_saved);
    }

    #[test]
    fn annual_savings_low_in_expected_range() {
        let range = AnnualSavingsRange::compute();
        // 2% of 134,737 MWh at $0.29/kWh = ~$781K
        assert!(range.low_savings.dollars_saved > 500_000.0,
            "Low savings should exceed $500K, got ${:.0}", range.low_savings.dollars_saved);
        assert!(range.low_savings.dollars_saved < 2_000_000.0,
            "Low savings should be under $2M, got ${:.0}", range.low_savings.dollars_saved);
    }

    #[test]
    fn psps_exposure_positive() {
        let exposure = PspsExposure::compute();
        assert!(exposure.revenue_at_risk_usd > 0.0);
    }

    #[test]
    fn psps_exposure_in_expected_range() {
        let exposure = PspsExposure::compute();
        // 48hr / 8760hr * $23.7M ≈ $130K
        assert!(exposure.revenue_at_risk_usd > 50_000.0,
            "PSPS exposure should exceed $50K");
        assert!(exposure.revenue_at_risk_usd < 500_000.0,
            "PSPS exposure should be under $500K at conservative estimate");
    }

    #[test]
    fn pilot_duration_is_90_days() {
        let pilot = PilotDeclaration::declare();
        assert_eq!(pilot.duration_days, 90);
    }

    #[test]
    fn pilot_domain_loci_matches_declared() {
        let pilot = PilotDeclaration::declare();
        assert_eq!(pilot.declared_domain_loci, 10);
        assert_eq!(pilot.declared_domain_edges, 10);
    }

    #[test]
    fn load_profile_residential_is_dominant() {
        let profile = LoadProfile::from_retail_sales(RETAIL_SALES_MWH);
        assert!(profile.residential_mwh > profile.commercial_mwh,
            "Residential load should exceed commercial");
        assert!(profile.commercial_mwh > profile.industrial_mwh,
            "Commercial load should exceed industrial (by account count)");
    }
}
