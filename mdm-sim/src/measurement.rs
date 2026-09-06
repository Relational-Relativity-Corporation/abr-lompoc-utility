// measurement.rs — Formal Measurement Mapping M : O → D
// V0.1.1 — revised per Verifier findings
//
// M : O → D declared before any operator acts.
//
// Every quantity used in this repository must be traceable back to an
// observable in O through this mapping. A quantity not traceable through M
// is not a constrained result — it is a result about nothing in D.
//
// Metatron Dynamics, Inc. — relationalrelativity.dev
// Delaware C-Corp #10551645
//
// ── Observable Set O ─────────────────────────────────────────────────────────
//
// O-01: meter_count_electric
//   Source: EIA Form 861 / findenergy.com (2019 actuals)
//   Value:  16,516 electric customer accounts
//   Unit:   count (dimensionless)
//   Maps to: D — total electric endpoint population
//
// O-02: meter_count_water
//   Source: Lompoc AMI RFP (BidNet Direct, June 2024) — covers both divisions
//   Value:  ~16,000 water accounts (OC-MDM-1 — estimated, confirm with Luther)
//   Unit:   count (dimensionless)
//   Maps to: D — total water endpoint population
//
// O-03: annual_retail_sales_mwh
//   Source: EIA Form 861 / findenergy.com (2019 actuals)
//   Value:  130,286 MWh
//   Unit:   megawatt-hours per year
//   Maps to: D — annual energy throughput observable
//
// O-04: annual_revenue_usd
//   Source: EIA Form 861 / findenergy.com (2019 actuals)
//   Value:  $23,686,000
//   Unit:   US dollars per year
//   Maps to: D — annual utility revenue observable (OC-MDM-2)
//
// O-05: vendor_mdm_cost_per_meter_low
//   Source: SMART360 / Siemens Gridscale X municipal SaaS tier
//           (bynry.com MDM comparison, April 2026; Siemens product page)
//   Value:  $3.00 per meter per year
//   Unit:   USD per endpoint per year
//   Maps to: D — lower bound of independently sourced vendor MDM cost range
//
// O-06: vendor_mdm_cost_per_meter_high
//   Source: Oracle Utilities / Itron IEE enterprise tier
//           (vendorbenchmark.com Energy Software Pricing Benchmark 2026)
//   Value:  $25.00 per meter per year
//   Unit:   USD per endpoint per year
//   Maps to: D — upper bound of independently sourced vendor MDM cost range
//
// O-07: vendor_mdm_implementation_cost_low
//   Source: bynry.com MDM comparison (April 2026) — small utility tier
//   Value:  $50,000 one-time
//   Unit:   USD
//   Maps to: D — lower bound of independently sourced implementation cost
//
// O-08: vendor_mdm_implementation_cost_high
//   Source: bynry.com MDM comparison (April 2026) — mid-market tier
//   Value:  $250,000 one-time
//   Unit:   USD
//   Maps to: D — upper bound of independently sourced implementation cost
//
// O-09: billing_error_scenario_fraction
//   Source: EIA distribution loss reporting / bynry.com MDM benefits (June 2026)
//   Value:  0.5% of annual revenue
//   Unit:   fraction of annual revenue
//   Maps to: D — scenario assumption for potentially recoverable billing value
//   NOTE (Verifier finding): Actual Lompoc billing error rate is NOT observable
//   from public data. This is a scenario assumption, not a measured observable.
//   Renamed from billing_error_fraction to billing_error_scenario_fraction.
//   All derived quantities are scenario outputs, not demonstrated recoveries.
//
// O-10: vee_labor_hours_per_meter_year
//   Source: industry benchmark — bynry.com MDM benefits (June 2026)
//   Value:  0.25 hours per meter per year (manual read exception handling)
//   Unit:   labor-hours per endpoint per year
//   Maps to: D — potentially addressable manual labor per endpoint
//   NOTE (Verifier finding): This declares addressable labor cost, not
//   demonstrated elimination. Derived quantities are scenario outputs.
//
// O-11: labor_cost_per_hour
//   Source: US Bureau of Labor Statistics — municipal utility worker loaded rate
//   Value:  $35.00 per hour
//   Unit:   USD per hour
//   Maps to: D — labor cost observable
//
// ── Δ Bounds — Declared per Meter Class (NO statistical projection) ───────────
//
// These bounds declare the admissible range of Δ(rₖ) = rₖ - rₖ₋₁
// where rₖ is a CUMULATIVE REGISTER READING in kWh (see operators.rs).
//
// Formula: B_c = k_c × (E_c / N_c / N_intervals)
//   where E_c  = synthetic class throughput (allocated from O-03 by account count)
//         N_c  = account count for class c (from O-01)
//         N_intervals = billing intervals per year (declared process step: 12)
//         k_c  = declared bound multiplier for class c (intervention parameter)
//
// ── Declared bound multipliers (intervention/design parameters) ───────────────
//
//   k_R = 3  (residential)
//   k_C = 5  (commercial — more variable load profile)
//   k_I = 10 (industrial — highly variable, process-driven load)
//
//   These multipliers are declared initial bound parameters, not derived from
//   O-03 alone. Their empirical adequacy is OPEN and will be tested prospectively
//   against utility-confirmed anomalies during the pilot. See OC-MDM-5.
//
// ── Synthetic class throughput (NOT observed per-class consumption) ───────────
//
//   E_c is allocated from aggregate O-03 (130,286 MWh) using account-count
//   proportions from O-01. This is a synthetic allocation — actual per-class
//   consumption is NOT currently observable from public data.
//
//   Residential: 130,286,000 kWh × (14,423 / 16,516) = ~113,773,438 kWh
//   Per account per interval: ~113,773,438 / 14,423 / 12 = ~657 kWh
//   B_R = k_R × 657 = 3 × 657 = ~1,970 kWh
//
//   Commercial: 130,286,000 kWh × (2,068 / 16,516) = ~16,311,800 kWh
//   Per account per interval: ~16,311,800 / 2,068 / 12 = ~657 kWh
//   B_C = k_C × 657 = 5 × 657 = ~3,285 kWh
//
//   Industrial: 130,286,000 kWh × (25 / 16,516) = ~197,000 kWh
//   Per account per interval: ~197,000 / 25 / 12 = ~657 kWh
//   B_I = k_I × 657 = 10 × 657 = ~6,570 kWh
//
//   Note: account-count allocation produces the same per-account interval mean
//   (~657 kWh) for all three classes. This is an artifact of the allocation
//   method, not an observed equality. Industrial accounts in practice consume
//   orders of magnitude more per account than residential.
//
// OC-MDM-5 (broadened from V0.1.1 per Verifier finding):
//   All three class Delta bounds are SYNTHETIC initial bounds. Class throughput
//   is presently allocated from aggregate O-03 using account-count proportions;
//   k_R=3, k_C=5, k_I=10 are declared initial bound parameters, not empirically
//   derived. Replace both synthetic throughput allocation and k parameters with
//   observed per-class AMI consumption distributions during pilot calibration.
//
// These bounds replace the mean/sigma classifier from V0.1.0.
// No statistical projection is introduced.

pub const METERS_ELECTRIC: u32 = 16_516;
pub const METERS_WATER:    u32 = 16_000;
pub const METERS_TOTAL:    u32 = METERS_ELECTRIC + METERS_WATER;

pub const ANNUAL_RETAIL_SALES_MWH: f64 = 130_286.0;
pub const ANNUAL_REVENUE_USD:      f64 = 23_686_000.0;

pub const VENDOR_MDM_COST_LOW_PER_METER:  f64 = 3.00;
pub const VENDOR_MDM_COST_HIGH_PER_METER: f64 = 25.00;
pub const VENDOR_IMPL_COST_LOW:           f64 = 50_000.0;
pub const VENDOR_IMPL_COST_HIGH:          f64 = 250_000.0;

/// Scenario assumption — not a measured observable (Verifier finding)
pub const BILLING_ERROR_SCENARIO_FRACTION: f64 = 0.005;

/// Potentially addressable labor hours per endpoint per year (Verifier finding)
pub const VEE_HOURS_PER_METER_YEAR: f64 = 0.25;
pub const LABOR_COST_PER_HOUR:      f64 = 35.00;

pub const SIM_YEARS: u32 = 5;

/// Billing intervals per year — declared process step (monthly billing)
pub const BILLING_INTERVALS_PER_YEAR: u32 = 12;

// ── Δ bounds per meter class ─────────────────────────────────────────────────
// Cumulative register reading: Δ cannot be negative (lower bound = 0).
// Upper bounds: B_c = k_c × (E_c / N_c / N_intervals)
// See full derivation and provenance in M declaration above.

/// Declared bound multipliers — intervention/design parameters (Verifier finding).
/// Not derived from O-03. Empirical adequacy is OPEN. See OC-MDM-5.
pub const BOUND_MULTIPLIER_RESIDENTIAL: f64 = 3.0;   // k_R
pub const BOUND_MULTIPLIER_COMMERCIAL:  f64 = 5.0;   // k_C
pub const BOUND_MULTIPLIER_INDUSTRIAL:  f64 = 10.0;  // k_I

/// Synthetic per-account interval mean — allocated from O-03 by account count.
/// Same value for all classes due to account-count allocation method.
/// NOT observed per-class consumption. See OC-MDM-5.
pub const SYNTHETIC_INTERVAL_MEAN_KWH: f64 = 657.0;

pub const DELTA_BOUND_RESIDENTIAL_LOW_KWH:  f64 = 0.0;
pub const DELTA_BOUND_RESIDENTIAL_HIGH_KWH: f64 =
    BOUND_MULTIPLIER_RESIDENTIAL * SYNTHETIC_INTERVAL_MEAN_KWH; // 3 × 657 = 1,971

pub const DELTA_BOUND_COMMERCIAL_LOW_KWH:   f64 = 0.0;
pub const DELTA_BOUND_COMMERCIAL_HIGH_KWH:  f64 =
    BOUND_MULTIPLIER_COMMERCIAL * SYNTHETIC_INTERVAL_MEAN_KWH;  // 5 × 657 = 3,285

pub const DELTA_BOUND_INDUSTRIAL_LOW_KWH:   f64 = 0.0;
pub const DELTA_BOUND_INDUSTRIAL_HIGH_KWH:  f64 =
    BOUND_MULTIPLIER_INDUSTRIAL * SYNTHETIC_INTERVAL_MEAN_KWH;  // 10 × 657 = 6,570

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_meters_is_sum_of_electric_and_water() {
        assert_eq!(METERS_TOTAL, METERS_ELECTRIC + METERS_WATER);
    }

    #[test]
    fn vendor_cost_high_exceeds_low() {
        assert!(VENDOR_MDM_COST_HIGH_PER_METER > VENDOR_MDM_COST_LOW_PER_METER);
    }

    #[test]
    fn billing_error_fraction_is_positive_and_under_one() {
        assert!(BILLING_ERROR_SCENARIO_FRACTION > 0.0);
        assert!(BILLING_ERROR_SCENARIO_FRACTION < 1.0);
    }

    #[test]
    fn labor_cost_positive() {
        assert!(LABOR_COST_PER_HOUR > 0.0);
    }

    #[test]
    fn sim_years_positive() {
        assert!(SIM_YEARS > 0);
    }

    #[test]
    fn delta_bounds_lower_is_zero_for_all_classes() {
        assert_eq!(DELTA_BOUND_RESIDENTIAL_LOW_KWH, 0.0);
        assert_eq!(DELTA_BOUND_COMMERCIAL_LOW_KWH, 0.0);
        assert_eq!(DELTA_BOUND_INDUSTRIAL_LOW_KWH, 0.0);
    }

    #[test]
    fn delta_bounds_high_increases_with_class_variability() {
        assert!(DELTA_BOUND_COMMERCIAL_HIGH_KWH > DELTA_BOUND_RESIDENTIAL_HIGH_KWH);
        assert!(DELTA_BOUND_INDUSTRIAL_HIGH_KWH > DELTA_BOUND_COMMERCIAL_HIGH_KWH);
    }

    #[test]
    fn delta_bounds_are_product_of_k_and_synthetic_mean() {
        // Verifier finding: k values are declared parameters, not derived from O-03.
        // This test verifies the formula B_c = k_c × synthetic_mean is correctly
        // implemented — it does not validate the empirical adequacy of k_c.
        assert!((DELTA_BOUND_RESIDENTIAL_HIGH_KWH
            - BOUND_MULTIPLIER_RESIDENTIAL * SYNTHETIC_INTERVAL_MEAN_KWH).abs() < 1.0,
            "Residential bound must equal k_R × synthetic mean");
        assert!((DELTA_BOUND_COMMERCIAL_HIGH_KWH
            - BOUND_MULTIPLIER_COMMERCIAL * SYNTHETIC_INTERVAL_MEAN_KWH).abs() < 1.0,
            "Commercial bound must equal k_C × synthetic mean");
        assert!((DELTA_BOUND_INDUSTRIAL_HIGH_KWH
            - BOUND_MULTIPLIER_INDUSTRIAL * SYNTHETIC_INTERVAL_MEAN_KWH).abs() < 1.0,
            "Industrial bound must equal k_I × synthetic mean");
    }

    #[test]
    fn bound_multipliers_increase_with_class_variability() {
        assert!(BOUND_MULTIPLIER_COMMERCIAL > BOUND_MULTIPLIER_RESIDENTIAL,
            "k_C must exceed k_R — commercial loads more variable");
        assert!(BOUND_MULTIPLIER_INDUSTRIAL > BOUND_MULTIPLIER_COMMERCIAL,
            "k_I must exceed k_C — industrial loads highly variable");
    }
}
