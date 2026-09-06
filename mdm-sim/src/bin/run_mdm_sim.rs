// run_mdm_sim.rs — Lompoc MDM Simulation: Full Report
// V0.1.1 — revised per Verifier findings
//
// Run with: cargo run --release --bin run_mdm_sim

use abr_lompoc_mdm_sim::measurement::*;
use abr_lompoc_mdm_sim::domain::{declare_domain, abr_operator_site};
use abr_lompoc_mdm_sim::operators::{
    IntervalRead, MeterClass, DeltaResult,
};
use abr_lompoc_mdm_sim::simulation::{
    vendor_scenarios, vendor_annual_cost, vendor_tco_5yr,
    MetatronScenario, ScenarioBenefits,
    scenario_net_position_5yr, scenario_cost_difference_5yr, scenario_breakeven_year,
};

fn main() {
    println!("=================================================================");
    println!("ABR/ABRCE — Lompoc Utilities: Relational MDM Simulation V0.1.2");
    println!("Metatron Dynamics, Inc. — relationalrelativity.dev");
    println!("=================================================================\n");

    // ── Step 1: Measurement Mapping ───────────────────────────────────────
    println!("--- STEP 1: MEASUREMENT MAPPING M : O → D ---");
    println!("  All quantities declared before operators act.");
    println!("  Full source citations in src/measurement.rs.\n");
    println!("  O-01  Electric meters:            {} endpoints", METERS_ELECTRIC);
    println!("  O-02  Water meters:               {} endpoints (OC-MDM-1)", METERS_WATER);
    println!("        Total endpoints:             {}", METERS_TOTAL);
    println!("  O-03  Annual retail sales:        {:.0} MWh", ANNUAL_RETAIL_SALES_MWH);
    println!("  O-04  Annual revenue:             ${:.0} (2019, OC-MDM-2)", ANNUAL_REVENUE_USD);
    println!("  O-05  Vendor MDM cost low:        ${:.2}/meter/yr (sourced)", VENDOR_MDM_COST_LOW_PER_METER);
    println!("  O-06  Vendor MDM cost high:       ${:.2}/meter/yr (sourced)", VENDOR_MDM_COST_HIGH_PER_METER);
    println!("  O-07  Impl cost low:              ${:.0} one-time (sourced)", VENDOR_IMPL_COST_LOW);
    println!("  O-08  Impl cost high:             ${:.0} one-time (sourced)", VENDOR_IMPL_COST_HIGH);
    println!("  O-09  Billing scenario fraction:  {:.1}% of revenue (scenario assumption, OC-MDM-3)", BILLING_ERROR_SCENARIO_FRACTION * 100.0);
    println!("  O-10  VEE hours/meter/yr:         {:.2} hrs (addressable labor, not demonstrated)", VEE_HOURS_PER_METER_YEAR);
    println!("  O-11  Labor cost/hr:              ${:.2}", LABOR_COST_PER_HOUR);
    println!();
    println!("  Δ BOUNDS DECLARED PER METER CLASS (from O-03 load profile — no statistics):");
    println!("  Residential:  [{:.0}, {:.0}] kWh/interval", DELTA_BOUND_RESIDENTIAL_LOW_KWH, DELTA_BOUND_RESIDENTIAL_HIGH_KWH);
    println!("  Commercial:   [{:.0}, {:.0}] kWh/interval", DELTA_BOUND_COMMERCIAL_LOW_KWH,  DELTA_BOUND_COMMERCIAL_HIGH_KWH);
    println!("  Industrial:   [{:.0}, {:.0}] kWh/interval  (OC-MDM-5)", DELTA_BOUND_INDUSTRIAL_LOW_KWH, DELTA_BOUND_INDUSTRIAL_HIGH_KWH);
    println!();

    // ── Step 2: Domain ─────────────────────────────────────────────────────
    println!("--- STEP 2: DECLARED RELATIONAL DOMAIN D ---");
    let (loci, edges) = declare_domain();
    println!("  Loci ({}):", loci.len());
    for l in &loci {
        let marker = if l.abr_operator_site { " ← ABR OPERATOR SITE" } else { "" };
        println!("    [{}] {}{}", l.id, l.label, marker);
    }
    println!("\n  Edges ({}):", edges.len());
    for e in &edges {
        println!("    {} → {}  |  {}",   e.from, e.to, e.label);
        println!("              Direction: {}", e.direction);
    }
    println!();

    // ── Step 3: Operator Application ───────────────────────────────────────
    println!("--- STEP 3: ABR OPERATOR SITE ---");
    println!("  Observable semantics: value_kwh is a CUMULATIVE REGISTER READING.");
    println!("  Δ(rₖ) = rₖ - rₖ₋₁ = interval consumption during period k.");
    println!("  Anomaly classification uses declared class bounds only — no statistics.\n");
    println!("  {}\n", abr_operator_site());

    println!("  Δ operator demonstration — synthetic cumulative register reads:");
    println!("  Meter 1001 (Residential), Meter 1002 (Commercial):\n");

    let reads_res = vec![
        IntervalRead { meter_id: 1001, meter_class: MeterClass::Residential, interval_k: 0, value_kwh: 45_320.0 },
        IntervalRead { meter_id: 1001, meter_class: MeterClass::Residential, interval_k: 1, value_kwh: 45_820.0 }, // Δ=500, normal
        IntervalRead { meter_id: 1001, meter_class: MeterClass::Residential, interval_k: 2, value_kwh: 45_820.0 }, // Δ=0, ZeroConsumption
        IntervalRead { meter_id: 1001, meter_class: MeterClass::Residential, interval_k: 3, value_kwh: 48_120.0 }, // Δ=2300, spike
        IntervalRead { meter_id: 1001, meter_class: MeterClass::Residential, interval_k: 4, value_kwh: 47_900.0 }, // Δ=-220, reversed
    ];

    let reads_com = vec![
        IntervalRead { meter_id: 1002, meter_class: MeterClass::Commercial, interval_k: 0, value_kwh: 120_000.0 },
        IntervalRead { meter_id: 1002, meter_class: MeterClass::Commercial, interval_k: 1, value_kwh: 122_500.0 }, // Δ=2500, normal for commercial
    ];

    for reads in [&reads_res, &reads_com] {
        for i in 1..reads.len() {
            if let Some(r) = DeltaResult::compute(&reads[i], &reads[i-1]) {
                let anomaly_str = match &r.anomaly {
                    Some(a) => format!("ANOMALY: {:?}", a),
                    None    => "normal".to_string(),
                };
                println!("    Meter {:4} [{:?}] interval {}: Δ = {:.0} kWh, bounds [{:.0},{:.0}] — {}",
                    r.meter_id, r.meter_class, r.interval_k,
                    r.delta_value, r.bound_low, r.bound_high, anomaly_str);
            }
        }
    }
    println!("  Note: Δ=2500 kWh is an anomaly for Residential but normal for Commercial.");
    println!("  Class-specific bounds are the operator criterion — not a statistical threshold.\n");

    // ── Step 4: Scenario Benefits ──────────────────────────────────────────
    println!("--- STEP 4: SCENARIO BENEFITS (contingent on assumptions) ---");
    println!("  These are scenario outputs, not demonstrated savings.");
    println!("  Actual values require pilot measurement to establish.\n");
    let b = ScenarioBenefits::compute();
    println!("  IF recoverable billing value = O-09 fraction × O-04 revenue:");
    println!("    Scenario annual benefit:   ${:.0}", b.billing_scenario_annual);
    println!("  IF addressable VEE labor = O-10 × O-11 × total meters:");
    println!("    Scenario annual benefit:   ${:.0}", b.vee_labor_scenario_annual);
    println!("  Total scenario annual:       ${:.0}", b.total_scenario_annual);
    println!("  Total scenario 5-year:       ${:.0}", b.total_scenario_5yr);
    println!();

    // ── Step 5: TCO Comparison ─────────────────────────────────────────────
    println!("--- STEP 5: 5-YEAR TCO COMPARISON ---");
    println!("  Horizon: {} years   Total endpoints: {}\n", SIM_YEARS, METERS_TOTAL);

    println!("  VENDOR MDM SCENARIOS:");
    println!("  {:-<65}", "");
    for s in vendor_scenarios() {
        let annual = vendor_annual_cost(&s);
        let tco    = vendor_tco_5yr(&s);
        let net    = scenario_net_position_5yr(tco, b.total_scenario_5yr);
        let be     = scenario_breakeven_year(annual, s.impl_cost_one_time, b.total_scenario_annual);
        let interp = if s.is_interpolated { " [INTERPOLATED — not independently sourced]" } else { "" };
        println!("  {}{}", s.name, interp);
        println!("    Source:         {}", s.source);
        println!("    Implementation: ${:.0}  Annual: ${:.0}/yr", s.impl_cost_one_time, annual);
        println!("    5-yr TCO:       ${:.0}", tco);
        println!("    5-yr scenario net: ${:.0}", net);
        match be {
            Some(y) => println!("    Scenario break-even: Year {}", y),
            None    => println!("    Scenario break-even: Not within {} years", SIM_YEARS),
        }
        println!();
    }

    println!("  METATRON RELATIONAL MDM:");
    println!("  {:-<65}", "");
    let m     = MetatronScenario::declare();
    let m_tco = m.tco_5yr();
    let m_net = scenario_net_position_5yr(m_tco, b.total_scenario_5yr);
    let m_be  = scenario_breakeven_year(m.annual_cost_ongoing(), m.impl_cost, b.total_scenario_annual);
    println!("  Metatron ABR relational MDM layer");
    println!("    Source:         {}", m.source);
    println!("    Implementation: ${:.0}   Year 1 (pilot): ${:.0}", m.impl_cost, m.annual_cost_year1());
    println!("    Years 2–5:      ${:.0}/yr  (${:.2}/meter/yr × {} meters)",
        m.annual_cost_ongoing(), m.ongoing_cost_per_meter, METERS_TOTAL);
    println!("    5-yr TCO:       ${:.0}", m_tco);
    println!("    5-yr scenario net: ${:.0}", m_net);
    match m_be {
        Some(y) => println!("    Scenario break-even: Year {}", y),
        None    => println!("    Scenario break-even: Not within {} years", SIM_YEARS),
    }
    println!();

    // ── Step 6: Scenario Cost Differences ────────────────────────────────
    println!("--- STEP 6: SCENARIO COST DIFFERENCES (not empirical results) ---");
    println!("  These are scenario outputs based on declared cost assumptions.");
    println!("  Actual vendor pricing requires independent quote verification.\n");
    println!("  {:-<65}", "");
    for s in vendor_scenarios() {
        let diff = scenario_cost_difference_5yr(vendor_tco_5yr(&s), m_tco);
        let interp = if s.is_interpolated { " [interpolated]" } else { "" };
        println!("  vs. {}{}", s.name, interp);
        println!("    Vendor 5-yr TCO:         ${:.0}", vendor_tco_5yr(&s));
        println!("    Metatron 5-yr TCO:       ${:.0}", m_tco);
        println!("    Scenario cost difference: ${:.0} over 5 years", diff);
        println!();
    }

    // ── Open Conditions ───────────────────────────────────────────────────
    println!("--- OPEN CONDITIONS ---");
    println!("  OC-MDM-1: Water meter count assumed 16,000 — confirm with Luther");
    println!("  OC-MDM-2: Revenue figure is 2019 actuals — current revenue likely higher");
    println!("  OC-MDM-3: Billing scenario fraction (0.5%) is assumption, not observed rate");
    println!("  OC-MDM-4: Metatron ongoing cost ($1.50/meter) is open scenario value");
    println!("  OC-MDM-5: All class bounds are synthetic pending actual per-class AMI data; industrial is the most significant current limitation");
    println!();
    println!("--- WHAT THE PILOT CLOSES ---");
    println!("  A 90-day pilot on live AMI data replaces all scenario assumptions with");
    println!("  actual observables through M:");
    println!("    OC-MDM-1 → confirmed meter count from Luther/AMI deployment");
    println!("    OC-MDM-3 → observed billing corrections during pilot window");
    println!("    OC-MDM-4 → actual Metatron cost structure post-pilot");
    println!("    OC-MDM-5 → actual per-class consumption from AMI stream");
    println!("  Result: every current scenario output becomes a measured quantity.");
    println!();
    println!("=================================================================");
    println!("All quantities classified by provenance: observed, derived,");
    println!("  scenario-assumed, interpolated, or open.");
    println!("  No scenario quantity is represented as an empirical result.");
    println!("Run cargo test to verify all declared constraints.");
    println!("=================================================================");
}
