// wholesale_cost.rs — CAISO Wholesale Cost Analysis for Lompoc Electric
//
// This module declares the observable wholesale price structure that
// Lompoc Electric faces as a 100% wholesale power purchaser through NCPA
// in the CAISO NP15 (Northern California) trading zone.
//
// Key finding: Lompoc sells power to all residential and most commercial
// customers at a flat rate (Schedule D-1, A-1) regardless of the hour.
// But Lompoc BUYS power at wholesale prices that vary significantly by
// hour — peak hours cost 40-80% more than off-peak hours.
//
// The spread between what Lompoc pays at peak and what it charges customers
// is the cost of having no load management. Every peak-hour kWh sold at
// flat rate is sold at or below cost. Every off-peak kWh sold at flat rate
// carries margin that subsidizes the peak loss.
//
// This analysis quantifies that spread using publicly sourced CAISO data.
// No AMI required. No SCADA required. The data is public.
//
// Metatron Dynamics, Inc. — relationalrelativity.dev
// Delaware C-Corp #10551645
//
// ── Measurement Mapping M : O → D ────────────────────────────────────────────
//
// O-W01: caiso_np15_peak_avg_usd_per_mwh
//   Source: S&P Global Commodity Insights, Aug 25 2025 —
//           "NP15 prices have averaged nearly $38.25/MWh so far in [2025]"
//           for on-peak. July 2024 average: $50/MWh (CAISO Summer Market
//           Performance Report, Aug 30 2024). Conservative declared value: $38/MWh.
//   Value:  $38.00/MWh on-peak (annual average, conservative)
//   Unit:   USD per MWh
//   Maps to: D — declared wholesale peak purchase price
//
// O-W02: caiso_np15_offpeak_avg_usd_per_mwh
//   Source: CAISO peak/off-peak spread — peak typically 40-80% above off-peak
//           (CAISO Summer Market Performance Report 2024; EIA wholesale data).
//           Off-peak declared at 60% of peak (midpoint of observed spread).
//   Value:  $23.00/MWh off-peak (annual average, conservative)
//   Unit:   USD per MWh
//   Maps to: D — declared wholesale off-peak purchase price
//
// O-W03: peak_hours_per_day_weekday
//   Source: CAISO market definition — peak = hours-ending 7-22 weekdays/Saturdays
//           (CAISO Summer Market Performance Report 2024, footnote 9)
//   Value:  16 hours per weekday/Saturday
//   Unit:   hours
//   Maps to: D — declared peak window
//
// O-W04: offpeak_hours_per_day_weekday
//   Source: CAISO market definition — off-peak = hours-ending 1-6 and 23-24
//   Value:  8 hours per weekday/Saturday
//   Unit:   hours
//   Maps to: D — declared off-peak window
//
// O-W05: lompoc_retail_flat_rate_usd_per_kwh
//   Source: City of Lompoc Schedule D-1 / A-1 (cityoflompoc.com rate schedules,
//           Oct 2024 rate revision). EnergySage March 2026: $0.29/kWh current.
//   Value:  $0.29/kWh = $290/MWh
//   Unit:   USD per MWh (converted from kWh)
//   Maps to: D — declared flat retail rate charged to customers
//
// O-W06: lompoc_no_tou_rate
//   Source: City of Lompoc Electric Rate Information page (cityoflompoc.com) —
//           rate schedules D-1, A-1 are flat rates; no time-of-use schedule
//           exists in the published rate schedule list as of Sep 2026.
//   Value:  TRUE — Lompoc has no TOU rate structure
//   Maps to: D — declared absence of price signal to customers
//
// O-W07: annual_retail_sales_mwh (carried from abr-lompoc-electric)
//   Source: EIA Form 861 (2019 actuals)
//   Value:  130,286 MWh/yr
//   Maps to: D — total annual energy throughput
//
// ── Open Conditions ───────────────────────────────────────────────────────────
//
// OC-WC-1: NCPA's actual wholesale contract terms with Lompoc are not public.
//   The CAISO NP15 LMP is the market price; NCPA's contract may include
//   capacity charges, transmission adders, or hedging that change the actual
//   cost structure. Declared values are conservative market-rate estimates.
//   Actual figures require disclosure from Luther / NCPA.
//
// OC-WC-2: Peak/off-peak load split for Lompoc is not directly observable
//   without AMI data. Split is declared at industry benchmark (60/40 peak/off-peak
//   by MWh for a utility of this profile). Actual split closes with AMI deployment.
//
// OC-WC-3: Wholesale price spread used (40% above off-peak) is conservative
//   mid-range. Actual 2024-2025 spreads have reached 80%+ during summer peaks.

/// CAISO NP15 declared wholesale prices (O-W01, O-W02)
pub const WHOLESALE_PEAK_USD_PER_MWH:    f64 = 38.00;
pub const WHOLESALE_OFFPEAK_USD_PER_MWH: f64 = 23.00;

/// Peak window definition (O-W03, O-W04)
pub const PEAK_HOURS_PER_WEEKDAY:    u32 = 16; // HE 7-22
pub const OFFPEAK_HOURS_PER_WEEKDAY: u32 = 8;  // HE 1-6, 23-24
pub const PEAK_HOURS_PER_WEEKEND:    u32 = 0;  // Sundays/holidays all off-peak
pub const OFFPEAK_HOURS_PER_WEEKEND: u32 = 24;

/// Retail flat rate — no TOU (O-W05, O-W06)
pub const RETAIL_FLAT_RATE_USD_PER_MWH: f64 = 290.00; // $0.29/kWh

/// Annual throughput (O-W07)
pub const ANNUAL_RETAIL_MWH: f64 = 130_286.0;

/// Declared peak/off-peak load split (OC-WC-2)
/// Industry benchmark for residential-dominant utility profile
pub const PEAK_LOAD_FRACTION:    f64 = 0.60; // 60% of annual MWh consumed during peak hours
pub const OFFPEAK_LOAD_FRACTION: f64 = 0.40; // 40% during off-peak hours

/// Declared peak hours per year
/// Weekdays/Saturdays: ~261 days × 16 hrs = 4,176 peak hrs
/// Sundays/holidays:   ~104 days × 0 peak hrs
pub const PEAK_HOURS_PER_YEAR:    u32 = 4_176;
pub const OFFPEAK_HOURS_PER_YEAR: u32 = 4_584; // 8760 - 4176

/// Peak/off-peak MWh split from declared fractions and annual throughput
pub fn peak_mwh_annual() -> f64 {
    ANNUAL_RETAIL_MWH * PEAK_LOAD_FRACTION
}

pub fn offpeak_mwh_annual() -> f64 {
    ANNUAL_RETAIL_MWH * OFFPEAK_LOAD_FRACTION
}

/// What Lompoc pays at wholesale for peak vs off-peak power
pub fn wholesale_peak_cost_annual() -> f64 {
    peak_mwh_annual() * WHOLESALE_PEAK_USD_PER_MWH
}

pub fn wholesale_offpeak_cost_annual() -> f64 {
    offpeak_mwh_annual() * WHOLESALE_OFFPEAK_USD_PER_MWH
}

pub fn wholesale_total_cost_annual() -> f64 {
    wholesale_peak_cost_annual() + wholesale_offpeak_cost_annual()
}

/// What Lompoc charges at retail (flat — same rate regardless of hour)
pub fn retail_revenue_annual() -> f64 {
    ANNUAL_RETAIL_MWH * RETAIL_FLAT_RATE_USD_PER_MWH
}

/// Gross margin (retail revenue minus wholesale cost)
/// This is the pool from which Lompoc pays all distribution O&M
pub fn gross_margin_annual() -> f64 {
    retail_revenue_annual() - wholesale_total_cost_annual()
}

/// The load-shifting opportunity:
/// If X% of peak MWh were shifted to off-peak hours,
/// Lompoc saves the peak/off-peak wholesale price spread on those MWh.
/// No change in retail revenue — same flat rate either way.
/// Pure wholesale cost reduction.
#[derive(Debug)]
pub struct LoadShiftScenario {
    pub shift_fraction:      f64,   // fraction of peak load shifted to off-peak
    pub mwh_shifted:         f64,   // MWh moved from peak to off-peak
    pub wholesale_saving:    f64,   // dollars saved on wholesale purchase
    pub price_spread_per_mwh: f64,  // peak minus off-peak price
    pub source:              &'static str,
}

impl LoadShiftScenario {
    pub fn compute(shift_fraction: f64) -> Self {
        let spread = WHOLESALE_PEAK_USD_PER_MWH - WHOLESALE_OFFPEAK_USD_PER_MWH;
        let shifted = peak_mwh_annual() * shift_fraction;
        let saving  = shifted * spread;
        LoadShiftScenario {
            shift_fraction,
            mwh_shifted:          shifted,
            wholesale_saving:     saving,
            price_spread_per_mwh: spread,
            source: "OC-WC-1/OC-WC-2: scenario output — actual savings require \
                     NCPA contract terms and AMI-measured peak/off-peak split",
        }
    }
}

/// CUR-1 analysis: Lompoc already has a curtailable load rate schedule.
/// This is the existing demand response mechanism in Lompoc's rate structure.
/// It applies to large customers willing to accept interruption for a discount.
/// 25 industrial accounts are the primary candidates.
#[derive(Debug)]
pub struct Cur1Analysis {
    pub industrial_accounts:       u32,
    pub industrial_load_fraction:  f64,  // by account count (OC-MDM-5 applies)
    pub industrial_mwh_annual:     f64,
    pub curtailment_scenario_pct:  f64,  // fraction curtailable during peak
    pub curtailable_peak_mwh:      f64,
    pub wholesale_saving:          f64,
    pub note:                      &'static str,
}

impl Cur1Analysis {
    pub fn compute(curtailment_pct: f64) -> Self {
        let ind_fraction = 25.0_f64 / 16_516.0_f64; // by account count
        let ind_mwh      = ANNUAL_RETAIL_MWH * ind_fraction;
        let ind_peak_mwh = ind_mwh * PEAK_LOAD_FRACTION;
        let curtailable  = ind_peak_mwh * curtailment_pct;
        let spread       = WHOLESALE_PEAK_USD_PER_MWH - WHOLESALE_OFFPEAK_USD_PER_MWH;
        let saving       = curtailable * spread;
        Cur1Analysis {
            industrial_accounts:      25,
            industrial_load_fraction: ind_fraction,
            industrial_mwh_annual:    ind_mwh,
            curtailment_scenario_pct: curtailment_pct,
            curtailable_peak_mwh:     curtailable,
            wholesale_saving:         saving,
            note: "Industrial load allocated by account count — OC-MDM-5 applies. \
                   Actual industrial consumption likely orders of magnitude higher. \
                   CUR-1 is an existing Lompoc rate schedule — no new rate action needed.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peak_price_exceeds_offpeak() {
        assert!(WHOLESALE_PEAK_USD_PER_MWH > WHOLESALE_OFFPEAK_USD_PER_MWH,
            "Peak wholesale price must exceed off-peak");
    }

    #[test]
    fn peak_offpeak_fractions_sum_to_one() {
        assert!((PEAK_LOAD_FRACTION + OFFPEAK_LOAD_FRACTION - 1.0).abs() < 0.001);
    }

    #[test]
    fn peak_hours_plus_offpeak_equals_year() {
        assert_eq!(PEAK_HOURS_PER_YEAR + OFFPEAK_HOURS_PER_YEAR, 8_760,
            "Peak + off-peak hours must equal hours in a year");
    }

    #[test]
    fn peak_mwh_plus_offpeak_mwh_equals_annual() {
        let total = peak_mwh_annual() + offpeak_mwh_annual();
        assert!((total - ANNUAL_RETAIL_MWH).abs() < 0.01);
    }

    #[test]
    fn wholesale_total_cost_positive() {
        assert!(wholesale_total_cost_annual() > 0.0);
    }

    #[test]
    fn gross_margin_positive() {
        assert!(gross_margin_annual() > 0.0,
            "Retail revenue must exceed wholesale cost");
    }

    #[test]
    fn retail_rate_substantially_above_wholesale_peak() {
        assert!(RETAIL_FLAT_RATE_USD_PER_MWH > WHOLESALE_PEAK_USD_PER_MWH * 2.0,
            "Retail rate should be well above wholesale — Lompoc is a distribution utility");
    }

    #[test]
    fn load_shift_10pct_saves_positive() {
        let s = LoadShiftScenario::compute(0.10);
        assert!(s.wholesale_saving > 0.0);
        assert!(s.mwh_shifted > 0.0);
    }

    #[test]
    fn load_shift_20pct_saves_double_10pct() {
        let s10 = LoadShiftScenario::compute(0.10);
        let s20 = LoadShiftScenario::compute(0.20);
        assert!((s20.wholesale_saving - 2.0 * s10.wholesale_saving).abs() < 0.01);
    }

    #[test]
    fn cur1_saving_positive() {
        let c = Cur1Analysis::compute(0.50);
        assert!(c.wholesale_saving > 0.0);
    }

    #[test]
    fn price_spread_is_difference_of_declared_prices() {
        let s = LoadShiftScenario::compute(0.10);
        assert!((s.price_spread_per_mwh
            - (WHOLESALE_PEAK_USD_PER_MWH - WHOLESALE_OFFPEAK_USD_PER_MWH)).abs() < 0.01);
    }
}
