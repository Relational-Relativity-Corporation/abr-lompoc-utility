// simulation.rs — 5-Year TCO Comparison Simulation
// V0.1.1 — revised per Verifier findings:
//   - DeclaredSavings → ScenarioBenefits
//   - Circular tests reclassified as scenario-consistency checks
//   - Mid-tier pricing explicitly flagged as interpolated scenario value
//   - Metatron $1.50/meter flagged as open scenario value (OC-MDM-4)
//   - Output language: "savings" → "scenario benefit"
//   - "cost advantage" → "scenario cost difference"

use crate::measurement::*;
use crate::operators::{billing_scenario_benefit_usd, vee_labor_scenario_benefit_usd};

/// Vendor MDM scenario — independently sourced or declared as interpolated
#[derive(Debug, Clone)]
pub struct VendorScenario {
    pub name:                    &'static str,
    pub cost_per_meter_per_year: f64,
    pub impl_cost_one_time:      f64,
    pub source:                  &'static str,
    pub is_interpolated:         bool, // Verifier finding: flag non-sourced values
}

pub fn vendor_scenarios() -> Vec<VendorScenario> {
    vec![
        VendorScenario {
            name: "Low-tier SaaS (Siemens Gridscale X / SMART360 entry)",
            cost_per_meter_per_year: VENDOR_MDM_COST_LOW_PER_METER,
            impl_cost_one_time:      VENDOR_IMPL_COST_LOW,
            source: "O-05 / O-07: bynry.com MDM comparison Apr 2026, Siemens product page",
            is_interpolated: false,
        },
        VendorScenario {
            name: "Mid-tier SaaS (Itron IEE Essentials municipal)",
            cost_per_meter_per_year: 8.00,
            impl_cost_one_time:      125_000.0,
            source: "INTERPOLATED between O-05 and O-06 — not independently sourced",
            is_interpolated: true, // Verifier finding
        },
        VendorScenario {
            name: "Enterprise on-premise (Oracle Utilities / Itron IEE)",
            cost_per_meter_per_year: VENDOR_MDM_COST_HIGH_PER_METER,
            impl_cost_one_time:      VENDOR_IMPL_COST_HIGH,
            source: "O-06 / O-08: vendorbenchmark.com Energy Software Pricing 2026",
            is_interpolated: false,
        },
    ]
}

pub fn vendor_annual_cost(scenario: &VendorScenario) -> f64 {
    METERS_TOTAL as f64 * scenario.cost_per_meter_per_year
}

pub fn vendor_tco_5yr(scenario: &VendorScenario) -> f64 {
    scenario.impl_cost_one_time + vendor_annual_cost(scenario) * SIM_YEARS as f64
}

/// Metatron MDM scenario
/// Year 1 pilot: zero software license (OC-MDM-4)
/// Years 2–5: open condition — simulated at 50% of low-tier vendor (OC-MDM-4)
/// $1.50/meter/year is a scenario parameter, not an observed price
#[derive(Debug)]
pub struct MetatronScenario {
    pub pilot_year_cost:        f64,
    pub ongoing_cost_per_meter: f64, // scenario value — OC-MDM-4
    pub impl_cost:              f64,
    pub source:                 &'static str,
}

impl MetatronScenario {
    pub fn declare() -> Self {
        MetatronScenario {
            pilot_year_cost:        0.0,
            ongoing_cost_per_meter: VENDOR_MDM_COST_LOW_PER_METER * 0.50,
            impl_cost:              0.0,
            source: "OC-MDM-4: open scenario value — simulated at 50% low-tier vendor. \
                     Not an observed or contracted price.",
        }
    }

    pub fn annual_cost_year1(&self) -> f64    { self.pilot_year_cost }
    pub fn annual_cost_ongoing(&self) -> f64  { METERS_TOTAL as f64 * self.ongoing_cost_per_meter }
    pub fn tco_5yr(&self) -> f64 {
        self.impl_cost
            + self.annual_cost_year1()
            + self.annual_cost_ongoing() * (SIM_YEARS - 1) as f64
    }
}

/// Scenario benefits — NOT demonstrated savings (Verifier finding).
/// Renamed from DeclaredSavings. These are scenario outputs contingent
/// on assumptions in O-09 and O-10 being realised in deployment.
#[derive(Debug)]
pub struct ScenarioBenefits {
    /// If recoverable billing value equals O-09 fraction of revenue
    pub billing_scenario_annual:     f64,
    /// If addressable VEE labor equals O-10 × O-11 × meter count
    pub vee_labor_scenario_annual:   f64,
    pub total_scenario_annual:       f64,
    pub total_scenario_5yr:          f64,
}

impl ScenarioBenefits {
    pub fn compute() -> Self {
        let billing = billing_scenario_benefit_usd();
        let labor   = vee_labor_scenario_benefit_usd(
            METERS_TOTAL, VEE_HOURS_PER_METER_YEAR, LABOR_COST_PER_HOUR
        );
        let annual = billing + labor;
        ScenarioBenefits {
            billing_scenario_annual:   billing,
            vee_labor_scenario_annual: labor,
            total_scenario_annual:     annual,
            total_scenario_5yr:        annual * SIM_YEARS as f64,
        }
    }
}

/// Scenario net position: scenario benefits minus TCO over 5 years.
/// This is a scenario output, not an empirical result.
pub fn scenario_net_position_5yr(tco: f64, scenario_benefits_5yr: f64) -> f64 {
    scenario_benefits_5yr - tco
}

/// Scenario cost difference between vendor and Metatron over 5 years.
/// NOT an empirical cost advantage — scenario result only (Verifier finding).
pub fn scenario_cost_difference_5yr(vendor_tco: f64, metatron_tco: f64) -> f64 {
    vendor_tco - metatron_tco
}

/// Break-even year within scenario: when cumulative scenario benefits
/// exceed cumulative cost. Returns None if not reached within horizon.
pub fn scenario_breakeven_year(
    annual_cost: f64,
    impl_cost:   f64,
    annual_scenario_benefit: f64,
) -> Option<u32> {
    let mut cumulative_cost    = impl_cost;
    let mut cumulative_benefit = 0.0;
    for year in 1..=SIM_YEARS {
        cumulative_cost    += annual_cost;
        cumulative_benefit += annual_scenario_benefit;
        if cumulative_benefit >= cumulative_cost {
            return Some(year);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_annual_cost_scales_with_meters() {
        let scenarios = vendor_scenarios();
        let low = vendor_annual_cost(&scenarios[0]);
        assert!((low - METERS_TOTAL as f64 * VENDOR_MDM_COST_LOW_PER_METER).abs() < 0.01);
    }

    #[test]
    fn vendor_tco_5yr_exceeds_annual() {
        for s in vendor_scenarios() {
            assert!(vendor_tco_5yr(&s) > vendor_annual_cost(&s));
        }
    }

    #[test]
    fn enterprise_tco_exceeds_low_tier() {
        let scenarios = vendor_scenarios();
        assert!(vendor_tco_5yr(&scenarios[2]) > vendor_tco_5yr(&scenarios[0]));
    }

    #[test]
    fn mid_tier_is_flagged_as_interpolated() {
        let scenarios = vendor_scenarios();
        assert!(scenarios[1].is_interpolated,
            "Mid-tier scenario must be flagged as interpolated");
    }

    #[test]
    fn low_and_high_tier_are_not_interpolated() {
        let scenarios = vendor_scenarios();
        assert!(!scenarios[0].is_interpolated);
        assert!(!scenarios[2].is_interpolated);
    }

    #[test]
    fn metatron_year1_is_zero() {
        let m = MetatronScenario::declare();
        assert_eq!(m.annual_cost_year1(), 0.0);
    }

    /// Scenario-consistency check only — verifies the scenario definition.
    /// Does not constitute evidence about real-world cost (Verifier finding).
    #[test]
    fn scenario_consistency_metatron_ongoing_below_low_vendor() {
        let m = MetatronScenario::declare();
        let scenarios = vendor_scenarios();
        assert!(m.annual_cost_ongoing() < vendor_annual_cost(&scenarios[0]),
            "Scenario consistency: Metatron cost is defined as 50% of low-tier");
    }

    /// Scenario-consistency check only — verifies the scenario definition.
    /// Does not constitute evidence about real-world cost (Verifier finding).
    #[test]
    fn scenario_consistency_metatron_tco_below_all_vendor_scenarios() {
        let m = MetatronScenario::declare();
        for s in vendor_scenarios() {
            assert!(m.tco_5yr() < vendor_tco_5yr(&s),
                "Scenario consistency: Metatron TCO is defined below vendor: {}", s.name);
        }
    }

    #[test]
    fn scenario_benefits_annual_positive() {
        let b = ScenarioBenefits::compute();
        assert!(b.total_scenario_annual > 0.0);
        assert!(b.billing_scenario_annual > 0.0);
        assert!(b.vee_labor_scenario_annual > 0.0);
    }

    #[test]
    fn scenario_benefits_5yr_equals_annual_times_5() {
        let b = ScenarioBenefits::compute();
        assert!((b.total_scenario_5yr - b.total_scenario_annual * SIM_YEARS as f64).abs() < 0.01);
    }

    #[test]
    fn scenario_breakeven_within_5yr_for_metatron() {
        let m = MetatronScenario::declare();
        let b = ScenarioBenefits::compute();
        let be = scenario_breakeven_year(
            m.annual_cost_ongoing(), m.impl_cost, b.total_scenario_annual,
        );
        assert!(be.is_some(), "Within scenario assumptions, break-even reached within 5 years");
        assert!(be.unwrap() <= SIM_YEARS);
    }
}
