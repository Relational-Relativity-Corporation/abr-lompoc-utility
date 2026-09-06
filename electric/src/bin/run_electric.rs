// run_electric.rs — Lompoc Electric Domain: Full Declaration Report
//
// Run with: cargo run --release --bin run_electric
//
// This program declares the Lompoc electric grid as a relational domain,
// computes all observables from public data, and prints a complete
// quantified report. Every number is recomputable from the declared
// inputs in src/observables.rs and src/pilot.rs.

use abr_lompoc_electric::domain;
use abr_lompoc_electric::observables::{
    LoadProfile, LoadAnomaly, disposition_retail_gap_mwh,
    disposition_retail_gap_pct, RETAIL_SALES_MWH, TOTAL_DISPOSITION_MWH,
    RATE_2026_USD_PER_KWH, TOTAL_REVENUE_USD,
};
use abr_lompoc_electric::pilot::{AnnualSavingsRange, PspsExposure, PilotDeclaration};
// Note: AnnualSavingsRange now uses DispositionGapScenario internally
use abr_lompoc_electric::wholesale_cost::{
    WHOLESALE_PEAK_USD_PER_MWH, WHOLESALE_OFFPEAK_USD_PER_MWH,
    RETAIL_FLAT_RATE_USD_PER_MWH, PEAK_HOURS_PER_YEAR, OFFPEAK_HOURS_PER_YEAR,
    peak_mwh_annual, offpeak_mwh_annual,
    wholesale_peak_cost_annual, wholesale_offpeak_cost_annual,
    wholesale_total_cost_annual, gross_margin_annual,
    LoadShiftScenario,
};

fn main() {
    println!("=================================================================");
    println!("ABR/ABRCE — Lompoc Electric Grid: Relational Domain Declaration");
    println!("Metatron Dynamics, Inc. — relationalrelativity.dev");
    println!("=================================================================\n");

    // 1. Domain structure
    let (loci, edges) = domain::declare_domain();
    println!("--- DECLARED DOMAIN ---");
    println!("Loci ({} total):", loci.len());
    for l in &loci {
        println!("  [{:>12?}] {} — {}", l.class, l.id, l.label);
    }
    println!("\nEdges ({} total):", edges.len());
    for e in &edges {
        println!("  {} → {}  |  {}", e.from, e.to, e.label);
    }
    println!();

    // 2. Structural vulnerability
    println!("--- PSPS STRUCTURAL VULNERABILITY ---");
    println!("{}\n", domain::psps_vulnerability());

    // 3. Declared inputs (public data)
    println!("--- DECLARED INPUTS (PUBLIC DATA, 2019 ACTUALS) ---");
    println!("  Total customers:           16,516");
    println!("    Residential:             14,423");
    println!("    Commercial:               2,068");
    println!("    Industrial:                  25");
    println!("  Retail sales:              {:.0} MWh", RETAIL_SALES_MWH);
    println!("  Total disposition:         {:.0} MWh", TOTAL_DISPOSITION_MWH);
    println!("  Total revenue:             ${:.0}", TOTAL_REVENUE_USD);
    println!("  Current residential rate:  ${:.4}/kWh (2026)", RATE_2026_USD_PER_KWH);
    println!();

    // 4. Disposition-retail gap (renamed from distribution loss per Verifier finding)
    let gap_mwh = disposition_retail_gap_mwh();
    let gap_pct = disposition_retail_gap_pct();
    println!("--- DISPOSITION-RETAIL GAP ---");
    println!("  Disposition - Retail:      {:.0} MWh ({:.2}%)", gap_mwh, gap_pct);
    println!("  Source: total disposition minus retail sales (EIA Form 861 / public data)");
    println!("  Note: gap is not confirmed as distribution loss — may include metering");
    println!("  differences or accounting adjustments. Treated as speculative upper bound.");
    println!();

    // 5. Load profile
    let profile = LoadProfile::from_retail_sales(RETAIL_SALES_MWH);
    println!("--- DECLARED LOAD PROFILE (proportional by account count) ---");
    println!("  Residential:               {:.0} MWh/yr ({:.1}%)",
        profile.residential_mwh, profile.residential_mwh / RETAIL_SALES_MWH * 100.0);
    println!("  Commercial:                {:.0} MWh/yr ({:.1}%)",
        profile.commercial_mwh, profile.commercial_mwh / RETAIL_SALES_MWH * 100.0);
    println!("  Industrial:                {:.0} MWh/yr ({:.1}%)",
        profile.industrial_mwh, profile.industrial_mwh / RETAIL_SALES_MWH * 100.0);
    println!("  Note: proportional allocation by account count — actual per-account");
    println!("  consumption differs by class; this baseline is for anomaly detection.");
    println!();

    // 6. Load anomaly demo (synthetic example for pilot illustration)
    println!("--- LOAD ANOMALY DETECTION (ILLUSTRATIVE EXAMPLE) ---");
    println!("  Illustrates what the deployed system produces.");
    println!("  Declared baseline on D1→D2 (residential): {:.0} MWh/yr", profile.residential_mwh);
    let anomaly_normal = LoadAnomaly::compute("D1→D2", profile.residential_mwh,
        profile.residential_mwh * 1.04, 10.0);
    let anomaly_flagged = LoadAnomaly::compute("D1→D2", profile.residential_mwh,
        profile.residential_mwh * 1.15, 10.0);
    println!("  Example A (4% deviation):  {:.0} MWh observed — deviation {:.1}% — FLAGGED: {}",
        anomaly_normal.observed_mwh, anomaly_normal.deviation_pct, anomaly_normal.flagged);
    println!("  Example B (15% deviation): {:.0} MWh observed — deviation {:.1}% — FLAGGED: {}",
        anomaly_flagged.observed_mwh, anomaly_flagged.deviation_pct, anomaly_flagged.flagged);
    println!();

    // 7. Disposition gap scenario (speculative — renamed per Verifier finding)
    let range = AnnualSavingsRange::compute();
    println!("--- DISPOSITION GAP SCENARIO (SPECULATIVE) ---");
    println!("  Industry benchmark: 2-5% of disposition recoverable through active management.");
    println!("  Applied to total disposition: {:.0} MWh", TOTAL_DISPOSITION_MWH);
    println!("  Rate: ${:.4}/kWh (2026 Lompoc residential rate, EnergySage March 2026)", RATE_2026_USD_PER_KWH);
    println!("  SCENARIO OUTPUT — actual Lompoc recovery requires pilot measurement.");
    println!("  Actual NCPA contract terms and Lompoc-specific losses not yet observable.");
    println!();
    println!("  At 2% benchmark:");
    println!("    MWh scenario:              {:.0} MWh/yr", range.low_savings.mwh_recovered);
    println!("    Scenario benefit:          ${:.0}/yr (speculative)", range.low_savings.dollars_saved);
    println!();
    println!("  At 5% benchmark:");
    println!("    MWh scenario:              {:.0} MWh/yr", range.high_savings.mwh_recovered);
    println!("    Scenario benefit:          ${:.0}/yr (speculative)", range.high_savings.dollars_saved);
    println!();

    // 8. PSPS revenue exposure
    let psps = PspsExposure::compute();
    println!("--- PSPS REVENUE EXPOSURE ---");
    println!("  Annual revenue:            ${:.0}", psps.annual_revenue_usd);
    println!("  Assumed PSPS hours/yr:     {:.0} hrs (conservative)", psps.psps_hours_assumed);
    println!("  Revenue at risk:           ${:.0}/yr", psps.revenue_at_risk_usd);
    println!("  Note: {}", psps.note);
    println!();

    // 9. Pilot declaration
    let pilot = PilotDeclaration::declare();
    println!("--- 90-DAY PILOT DECLARATION ---");
    println!("  Duration:                  {} days", pilot.duration_days);
    println!("  Declared domain loci:      {}", pilot.declared_domain_loci);
    println!("  Declared domain edges:     {}", pilot.declared_domain_edges);
    println!("  Baseline source:           {}", pilot.baseline_source);
    println!("  Detection threshold:       {:.0}% deviation from declared load profile", pilot.detection_threshold_pct);
    println!("  Success criterion:         {}", pilot.success_criterion);
    println!("  Deliverable:               {}", pilot.deliverable);
    println!();

    // 10. Wholesale cost analysis
    println!("--- WHOLESALE COST ANALYSIS: PEAK vs OFF-PEAK ---");
    println!("  Lompoc buys 100% wholesale via NCPA in CAISO NP15 zone.");
    println!("  Lompoc sells at flat rate (Schedule D-1/A-1) — NO time-of-use.");
    println!("  Source: Lompoc rate schedule page (cityoflompoc.com, Sep 2026).");
    println!();
    println!("  Declared wholesale prices (O-W01, O-W02 — CAISO NP15 annual avg):");
    println!("    On-peak:   ${:.2}/MWh  ({} hrs/yr)", WHOLESALE_PEAK_USD_PER_MWH, PEAK_HOURS_PER_YEAR);
    println!("    Off-peak:  ${:.2}/MWh  ({} hrs/yr)", WHOLESALE_OFFPEAK_USD_PER_MWH, OFFPEAK_HOURS_PER_YEAR);
    println!("    Spread:    ${:.2}/MWh  ({:.0}% premium for peak power)",
        WHOLESALE_PEAK_USD_PER_MWH - WHOLESALE_OFFPEAK_USD_PER_MWH,
        (WHOLESALE_PEAK_USD_PER_MWH / WHOLESALE_OFFPEAK_USD_PER_MWH - 1.0) * 100.0);
    println!("  Retail flat rate: ${:.2}/MWh (same charged regardless of hour)", RETAIL_FLAT_RATE_USD_PER_MWH);
    println!();
    println!("  Declared load split (OC-WC-2 — industry benchmark, closes with AMI):");
    println!("    Peak MWh:     {:.0} MWh/yr (60% of annual)", peak_mwh_annual());
    println!("    Off-peak MWh: {:.0} MWh/yr (40% of annual)", offpeak_mwh_annual());
    println!();
    println!("  Annual wholesale cost:");
    println!("    Peak hours:   ${:.0}", wholesale_peak_cost_annual());
    println!("    Off-peak hrs: ${:.0}", wholesale_offpeak_cost_annual());
    println!("    Total:        ${:.0}", wholesale_total_cost_annual());
    println!("  Annual retail revenue (flat rate): ${:.0}", gross_margin_annual() + wholesale_total_cost_annual());
    println!("  Gross margin (revenue - wholesale): ${:.0}", gross_margin_annual());
    println!();
    println!("--- LOAD SHIFTING SCENARIO (wholesale cost reduction, no AMI needed) ---");
    println!("  Shifting load from peak to off-peak hours saves the spread on");
    println!("  wholesale purchases. Retail revenue unchanged — flat rate either way.");
    println!("  OC-WC-1: Actual NCPA contract terms may differ from market LMP.");
    println!();
    for pct in [0.05_f64, 0.10, 0.20] {
        let s = LoadShiftScenario::compute(pct);
        println!("  Shift {:.0}% of peak load to off-peak:", pct * 100.0);
        println!("    MWh shifted:       {:.0} MWh/yr", s.mwh_shifted);
        println!("    Wholesale saving:  ${:.0}/yr", s.wholesale_saving);
        println!();
    }
    println!("--- CUR-1 ANALYSIS: EXISTING DEMAND RESPONSE MECHANISM ---");
    println!("  Lompoc already has Schedule CUR-1 (Firm Curtailable Load).");
    println!("  Large customers accept interruption during peak in exchange for discount.");
    println!("  25 industrial accounts are primary candidates.");
    println!("  No new rate structure needed — CUR-1 is live today.");
    println!("  OC-MDM-5 applies: industrial load underrepresented by account-count allocation.");
    println!();
    println!("  Industrial load by account count is a known significant underestimate");
    println!("  (OC-MDM-5 — account-count allocation understates actual industrial MWh).");
    println!("  Dollar figure for CUR-1 NOT REPORTED until actual industrial-class");
    println!("  consumption is available from Luther/AMI deployment.");
    println!("  Structural opportunity: confirmed. Quantitative opportunity: OPEN.");
    println!();
    println!("--- WHAT VISIBILITY CHANGES ---");
    println!("  Without AMI: Lompoc cannot see which feeders are peaking or when.");
    println!("  With Metatron MDM layer on AMI data:");
    println!("    - Peak demand detected in real time on declared edges");
    println!("    - CUR-1 dispatch can be targeted to actual peak feeders");
    println!("    - Load shift savings become measurable, not scenario outputs");
    println!("    - OC-WC-2 closes: actual peak/off-peak split observed per meter");
    println!("  The relational layer converts the spread from a hidden cost");
    println!("  into a declared, observable, actionable quantity.");
    println!();
    println!("=================================================================");
    println!("All quantities classified by provenance: observed, derived,");
    println!("  scenario-assumed, interpolated, or open.");
    println!("Run cargo test to verify all declared constraints.");
    println!("=================================================================");
}
