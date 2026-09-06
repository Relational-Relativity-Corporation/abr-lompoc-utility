// observables.rs — Declared observables for the Lompoc electric domain
//
// An observable in the ABR sense is a quantity that can be measured
// on a declared edge or locus — a change in that quantity under
// normal vs. anomalous conditions is detectable without knowing
// in advance what caused it.
//
// Two observable classes are declared here:
//   1. Load anomaly — deviation from declared load profile at a distribution edge
//   2. Disposition gap scenario — speculative scenario computation applying a
//      declared benchmark fraction to total disposition (not to the 4,451 MWh gap itself).

// Source: EIA Form 861 / findenergy.com (2019)
pub const RETAIL_SALES_MWH: f64 = 130_286.0;
pub const TOTAL_DISPOSITION_MWH: f64 = 134_737.0;
pub const TOTAL_REVENUE_USD: f64 = 23_686_000.0;
pub const RATE_2026_USD_PER_KWH: f64 = 0.29;

// Customer class fractions (from declared customer counts)
// Residential: 14,423 / 16,516 = 0.8733
// Commercial:   2,068 / 16,516 = 0.1252
// Industrial:      25 / 16,516 = 0.0015
pub const FRACTION_RESIDENTIAL: f64 = 14_423.0 / 16_516.0;
pub const FRACTION_COMMERCIAL:  f64 =  2_068.0 / 16_516.0;
pub const FRACTION_INDUSTRIAL:  f64 =     25.0 / 16_516.0;

/// Declared load profile: expected MWh throughput per customer class
/// derived from retail sales and customer class fractions.
/// Note: this is a proportional allocation by account count —
/// actual per-account consumption differs by class; this is
/// declared as the baseline for anomaly detection, not as a
/// per-account billing model.
#[derive(Debug)]
pub struct LoadProfile {
    pub residential_mwh: f64,
    pub commercial_mwh:  f64,
    pub industrial_mwh:  f64,
    pub total_mwh:       f64,
}

impl LoadProfile {
    pub fn from_retail_sales(retail_mwh: f64) -> Self {
        LoadProfile {
            residential_mwh: retail_mwh * FRACTION_RESIDENTIAL,
            commercial_mwh:  retail_mwh * FRACTION_COMMERCIAL,
            industrial_mwh:  retail_mwh * FRACTION_INDUSTRIAL,
            total_mwh:       retail_mwh,
        }
    }
}

/// Load anomaly: deviation from declared load profile on a given edge.
/// In a deployed system, actual metered throughput is compared to
/// the declared profile. A deviation beyond threshold is an observable.
#[derive(Debug)]
pub struct LoadAnomaly {
    pub edge_id:         &'static str,
    pub declared_mwh:    f64,
    pub observed_mwh:    f64,
    pub deviation_mwh:   f64,
    pub deviation_pct:   f64,
    pub threshold_pct:   f64,
    pub flagged:         bool,
}

impl LoadAnomaly {
    pub fn compute(
        edge_id: &'static str,
        declared_mwh: f64,
        observed_mwh: f64,
        threshold_pct: f64,
    ) -> Self {
        let deviation_mwh = observed_mwh - declared_mwh;
        let deviation_pct = (deviation_mwh / declared_mwh).abs() * 100.0;
        let flagged = deviation_pct > threshold_pct;
        LoadAnomaly {
            edge_id,
            declared_mwh,
            observed_mwh,
            deviation_mwh,
            deviation_pct,
            threshold_pct,
            flagged,
        }
    }
}

/// Disposition gap scenario benefit: speculative scenario output.
/// Applies an industry benchmark reduction fraction to the disposition-retail gap.
/// NOT a demonstrated saving — actual Lompoc recovery requires pilot measurement.
/// Renamed from DispositionGapScenario per Verifier finding.
#[derive(Debug)]
pub struct DispositionGapScenario {
    pub reduction_fraction: f64,
    pub mwh_recovered:      f64,
    pub dollars_saved:      f64,
}

impl DispositionGapScenario {
    pub fn compute(total_disposition_mwh: f64, reduction_fraction: f64, rate_usd_per_kwh: f64) -> Self {
        let mwh_recovered = total_disposition_mwh * reduction_fraction;
        let kwh_recovered = mwh_recovered * 1000.0;
        let dollars_saved = kwh_recovered * rate_usd_per_kwh;
        DispositionGapScenario {
            reduction_fraction,
            mwh_recovered,
            dollars_saved,
        }
    }
}

/// Disposition-retail gap: total disposition minus retail sales.
/// This gap is not confirmed as distribution loss — it may include
/// metering differences, estimation adjustments, or other accounting.
/// Renamed from implied_distribution_loss_mwh per Verifier finding.
pub fn disposition_retail_gap_mwh() -> f64 {
    TOTAL_DISPOSITION_MWH - RETAIL_SALES_MWH
}

pub fn disposition_retail_gap_pct() -> f64 {
    disposition_retail_gap_mwh() / TOTAL_DISPOSITION_MWH * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_profile_fractions_sum_to_one() {
        let sum = FRACTION_RESIDENTIAL + FRACTION_COMMERCIAL + FRACTION_INDUSTRIAL;
        assert!((sum - 1.0).abs() < 0.001,
            "Customer class fractions should sum to ~1.0, got {:.6}", sum);
    }

    #[test]
    fn load_profile_total_matches_retail_sales() {
        let profile = LoadProfile::from_retail_sales(RETAIL_SALES_MWH);
        assert!((profile.total_mwh - RETAIL_SALES_MWH).abs() < 0.001,
            "Load profile total should match retail sales");
    }

    #[test]
    fn load_anomaly_flagged_when_over_threshold() {
        let anomaly = LoadAnomaly::compute("D2→L1", 1000.0, 1200.0, 10.0);
        assert!(anomaly.flagged, "20% deviation should be flagged at 10% threshold");
    }

    #[test]
    fn load_anomaly_not_flagged_when_under_threshold() {
        let anomaly = LoadAnomaly::compute("D2→L1", 1000.0, 1050.0, 10.0);
        assert!(!anomaly.flagged, "5% deviation should not be flagged at 10% threshold");
    }

    #[test]
    fn disposition_scenario_2pct_positive() {
        let savings = DispositionGapScenario::compute(TOTAL_DISPOSITION_MWH, 0.02, RATE_2026_USD_PER_KWH);
        assert!(savings.dollars_saved > 0.0, "Savings should be positive");
        assert!(savings.mwh_recovered > 0.0, "MWh recovered should be positive");
    }

    #[test]
    fn disposition_scenario_5pct_greater_than_2pct() {
        let low  = DispositionGapScenario::compute(TOTAL_DISPOSITION_MWH, 0.02, RATE_2026_USD_PER_KWH);
        let high = DispositionGapScenario::compute(TOTAL_DISPOSITION_MWH, 0.05, RATE_2026_USD_PER_KWH);
        assert!(high.dollars_saved > low.dollars_saved,
            "5% reduction should yield greater savings than 2%");
    }

    #[test]
    fn disposition_gap_is_positive() {
        assert!(disposition_retail_gap_mwh() > 0.0,
            "Disposition-retail gap should be positive");
    }

    #[test]
    fn disposition_gap_pct_under_10() {
        let pct = disposition_retail_gap_pct();
        assert!(pct < 10.0,
            "Implied loss pct should be under 10%, got {:.2}%", pct);
    }
}
