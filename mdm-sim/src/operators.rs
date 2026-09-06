// operators.rs — ABR Operators Acting at Locus A
// V0.1.1 — revised per Verifier findings:
//   - value_kwh explicitly declared as CUMULATIVE REGISTER READING
//   - FlatLine removed (not implemented — Verifier finding)
//   - mean/σ classifier replaced with declared Δ bounds from M (no statistics)
//
// Metatron Dynamics, Inc. — relationalrelativity.dev

use crate::measurement::{
    DELTA_BOUND_RESIDENTIAL_LOW_KWH, DELTA_BOUND_RESIDENTIAL_HIGH_KWH,
    DELTA_BOUND_COMMERCIAL_LOW_KWH,  DELTA_BOUND_COMMERCIAL_HIGH_KWH,
    DELTA_BOUND_INDUSTRIAL_LOW_KWH,  DELTA_BOUND_INDUSTRIAL_HIGH_KWH,
    ANNUAL_REVENUE_USD, BILLING_ERROR_SCENARIO_FRACTION,
};

// ── Observable semantics declaration ─────────────────────────────────────────
//
// value_kwh is a CUMULATIVE REGISTER READING — the total kWh registered
// on the meter since installation (or last reset), as read at interval k.
//
// This is the standard observable reported by smart meters in MDM systems.
// It is NOT interval consumption. Interval consumption is derived as Δ(rₖ).
//
// Consequence: Δ(rₖ) = rₖ - rₖ₋₁ = consumption during interval k.
//   Δ = 0  → zero consumption during interval (ZeroConsumption — admissible)
//   Δ < 0  → register decreased — directional violation (ReversedReading)
//   Δ > declared upper bound → consumption spike beyond declared class bound
//
// This declaration satisfies the Verifier finding on observable semantics.

/// Meter class — determines which declared Δ bounds apply at locus A
#[derive(Debug, Clone, PartialEq)]
pub enum MeterClass {
    Residential,
    Commercial,
    Industrial,
}

impl MeterClass {
    /// Declared Δ bounds for this class, derived from M (no statistics)
    pub fn delta_bounds(&self) -> (f64, f64) {
        match self {
            MeterClass::Residential => (DELTA_BOUND_RESIDENTIAL_LOW_KWH, DELTA_BOUND_RESIDENTIAL_HIGH_KWH),
            MeterClass::Commercial  => (DELTA_BOUND_COMMERCIAL_LOW_KWH,  DELTA_BOUND_COMMERCIAL_HIGH_KWH),
            MeterClass::Industrial  => (DELTA_BOUND_INDUSTRIAL_LOW_KWH,  DELTA_BOUND_INDUSTRIAL_HIGH_KWH),
        }
    }
}

/// A single interval read from one meter endpoint.
/// value_kwh: CUMULATIVE REGISTER READING in kWh.
/// Traceable to O-01 (electric) or O-02 (water) through M.
#[derive(Debug, Clone)]
pub struct IntervalRead {
    pub meter_id:   u32,
    pub meter_class: MeterClass,
    pub interval_k: u32,
    pub value_kwh:  f64,  // cumulative register — NOT interval consumption
}

/// Δ operator: change across one declared adjacency relation.
/// Result is interval consumption = rₖ - rₖ₋₁.
/// Returns None if reads are not adjacent on the same meter.
pub fn delta(current: &IntervalRead, prior: &IntervalRead) -> Option<f64> {
    if current.meter_id != prior.meter_id {
        return None; // Δ not declared across meter boundaries
    }
    if current.interval_k != prior.interval_k + 1 {
        return None; // Δ only declared across adjacent intervals
    }
    Some(current.value_kwh - prior.value_kwh)
}

/// Declared anomaly types — what Δ can surface at locus A.
/// FlatLine removed (not implemented — Verifier finding V0.1.0).
/// Mean/σ-based spike removed — replaced by declared class bounds.
#[derive(Debug, Clone, PartialEq)]
pub enum AnomalyType {
    /// Δ < 0: cumulative register decreased — directional violation
    ReversedReading,
    /// Δ = 0: zero consumption during interval on active meter
    ZeroConsumption,
    /// Δ > declared class upper bound: consumption exceeds declared envelope
    ConsumptionSpike,
}

/// Result of applying Δ at locus A to one interval boundary.
/// Anomaly classification uses only declared bounds from M — no statistics.
#[derive(Debug, Clone)]
pub struct DeltaResult {
    pub meter_id:     u32,
    pub meter_class:  MeterClass,
    pub interval_k:   u32,
    pub delta_value:  f64,
    pub anomaly:      Option<AnomalyType>,
    pub bound_low:    f64,
    pub bound_high:   f64,
}

impl DeltaResult {
    /// Apply Δ and classify using declared class bounds only.
    /// No mean, no σ, no statistical projection.
    pub fn compute(current: &IntervalRead, prior: &IntervalRead) -> Option<Self> {
        let d = delta(current, prior)?;
        let (low, high) = current.meter_class.delta_bounds();
        let anomaly = if d < low {
            // d < 0 (since low = 0): register decreased
            Some(AnomalyType::ReversedReading)
        } else if d == 0.0 {
            Some(AnomalyType::ZeroConsumption)
        } else if d > high {
            Some(AnomalyType::ConsumptionSpike)
        } else {
            None
        };
        Some(DeltaResult {
            meter_id:    current.meter_id,
            meter_class: current.meter_class.clone(),
            interval_k:  current.interval_k,
            delta_value: d,
            anomaly,
            bound_low:   low,
            bound_high:  high,
        })
    }
}

/// Scenario benefit: potentially recoverable billing value.
/// NOT "savings" — Verifier finding. This is a scenario output.
/// Traceable to O-09 (scenario fraction) × O-04 (revenue) through M.
pub fn billing_scenario_benefit_usd() -> f64 {
    ANNUAL_REVENUE_USD * BILLING_ERROR_SCENARIO_FRACTION
}

/// Scenario benefit: potentially addressable VEE labor cost.
/// NOT "savings" — Verifier finding. This is a scenario output.
/// Traceable to O-10 × O-11 × meter count through M.
pub fn vee_labor_scenario_benefit_usd(
    meters_total: u32,
    vee_hours_per_meter: f64,
    labor_cost_per_hour: f64,
) -> f64 {
    meters_total as f64 * vee_hours_per_meter * labor_cost_per_hour
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(meter_id: u32, class: MeterClass, interval_k: u32, value_kwh: f64) -> IntervalRead {
        IntervalRead { meter_id, meter_class: class, interval_k, value_kwh }
    }

    #[test]
    fn delta_computes_correct_change() {
        let prior   = read(1, MeterClass::Residential, 0, 1000.0);
        let current = read(1, MeterClass::Residential, 1, 1115.0);
        assert_eq!(delta(&current, &prior), Some(115.0));
    }

    #[test]
    fn delta_returns_none_for_non_adjacent_intervals() {
        let prior   = read(1, MeterClass::Residential, 0, 1000.0);
        let current = read(1, MeterClass::Residential, 2, 1115.0);
        assert_eq!(delta(&current, &prior), None);
    }

    #[test]
    fn delta_returns_none_across_meter_boundaries() {
        let prior   = read(1, MeterClass::Residential, 0, 1000.0);
        let current = read(2, MeterClass::Residential, 1, 1115.0);
        assert_eq!(delta(&current, &prior), None);
    }

    #[test]
    fn reversed_reading_detected_by_declared_bound() {
        let prior   = read(1, MeterClass::Residential, 0, 2000.0);
        let current = read(1, MeterClass::Residential, 1, 1800.0); // Δ = -200
        let result  = DeltaResult::compute(&current, &prior).unwrap();
        assert_eq!(result.anomaly, Some(AnomalyType::ReversedReading));
    }

    #[test]
    fn zero_consumption_detected_by_declared_bound() {
        let prior   = read(1, MeterClass::Residential, 0, 1000.0);
        let current = read(1, MeterClass::Residential, 1, 1000.0); // Δ = 0
        let result  = DeltaResult::compute(&current, &prior).unwrap();
        assert_eq!(result.anomaly, Some(AnomalyType::ZeroConsumption));
    }

    #[test]
    fn spike_detected_beyond_declared_residential_bound() {
        // Δ = 2500 kWh > declared residential high of 1970
        let prior   = read(1, MeterClass::Residential, 0, 1000.0);
        let current = read(1, MeterClass::Residential, 1, 3500.0);
        let result  = DeltaResult::compute(&current, &prior).unwrap();
        assert_eq!(result.anomaly, Some(AnomalyType::ConsumptionSpike));
    }

    #[test]
    fn normal_residential_read_produces_no_anomaly() {
        // Δ = 500 kWh — within declared residential bounds [0, 1970]
        let prior   = read(1, MeterClass::Residential, 0, 1000.0);
        let current = read(1, MeterClass::Residential, 1, 1500.0);
        let result  = DeltaResult::compute(&current, &prior).unwrap();
        assert_eq!(result.anomaly, None);
    }

    #[test]
    fn commercial_bound_higher_than_residential() {
        let (_, res_high) = MeterClass::Residential.delta_bounds();
        let (_, com_high) = MeterClass::Commercial.delta_bounds();
        assert!(com_high > res_high);
    }

    #[test]
    fn industrial_bound_higher_than_commercial() {
        let (_, com_high) = MeterClass::Commercial.delta_bounds();
        let (_, ind_high) = MeterClass::Industrial.delta_bounds();
        assert!(ind_high > com_high);
    }

    #[test]
    fn residential_spike_within_commercial_bound_is_not_anomaly_for_commercial() {
        // Δ = 2500 — spike for residential but normal for commercial
        let prior   = read(1, MeterClass::Commercial, 0, 1000.0);
        let current = read(1, MeterClass::Commercial, 1, 3500.0);
        let result  = DeltaResult::compute(&current, &prior).unwrap();
        assert_eq!(result.anomaly, None,
            "2500 kWh delta is within commercial bounds and should not be flagged");
    }

    #[test]
    fn billing_scenario_benefit_traceable_to_revenue() {
        let benefit = billing_scenario_benefit_usd();
        assert!((benefit - ANNUAL_REVENUE_USD * BILLING_ERROR_SCENARIO_FRACTION).abs() < 0.01);
    }

    #[test]
    fn vee_labor_scenario_benefit_positive() {
        let benefit = vee_labor_scenario_benefit_usd(32_516, 0.25, 35.0);
        assert!(benefit > 0.0);
    }
}
