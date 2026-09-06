// Lompoc Electric Grid — Declared Domain Data
// All figures sourced from public records. Sources cited inline.
//
// Source 1: EIA Form 861 / findenergy.com — 2019 actuals
//   Total customers: 16,516 (14,423 residential, 2,068 commercial, 25 industrial)
//   Total retail sales: 130,286 MWh
//   Total sales and disposition: 134,737 MWh
//   Total revenue: $23,686,000
//   Average residential rate: $0.2090/kWh
//
// Source 2: City of Lompoc Electric Division (cityoflompoc.com)
//   Lompoc Electric purchases 100% of power at wholesale via PG&E transmission lines
//   PG&E PSPS events can shut off all Lompoc power with no city control
//
// Source 3: EnergySage (March 2026)
//   Current Lompoc residential rate: $0.29/kWh (up from $0.2090 in 2019)
//   California average: $0.3062/kWh
//   National average: $0.20/kWh
//
// Source 4: City of Lompoc rate increase announcement (October 2024)
//   8% annual residential rate increase planned over 5 years
//   Emergency 30% rate increase occurred May 2023

pub const CUSTOMERS_RESIDENTIAL: u32 = 14_423;
pub const CUSTOMERS_COMMERCIAL:   u32 = 2_068;
pub const CUSTOMERS_INDUSTRIAL:   u32 = 25;
pub const CUSTOMERS_TOTAL:        u32 = 16_516;

// MWh — 2019 actuals (EIA Form 861)
pub const RETAIL_SALES_MWH:       f64 = 130_286.0;
pub const TOTAL_DISPOSITION_MWH:  f64 = 134_737.0;

// Transmission loss implied by difference: disposition - retail = ~4,451 MWh
pub const DISPOSITION_LESS_RETAIL_MWH: f64 = TOTAL_DISPOSITION_MWH - RETAIL_SALES_MWH;

// Revenue — 2019 actuals
pub const TOTAL_REVENUE_USD:      f64 = 23_686_000.0;
pub const RETAIL_REVENUE_USD:     f64 = 21_782_000.0; // 91.94% of total

// Rates
pub const RATE_RESIDENTIAL_2019:  f64 = 0.2090; // $/kWh
pub const RATE_RESIDENTIAL_2026:  f64 = 0.29;   // $/kWh (EnergySage March 2026)

// Load balancing savings — industry benchmark range
pub const LOSS_REDUCTION_LOW:     f64 = 0.02; // 2%
pub const LOSS_REDUCTION_HIGH:    f64 = 0.05; // 5%

// PSPS exposure — PG&E transmission dependency
// Lompoc Electric purchases 100% wholesale; zero generation assets
// All power flows through PG&E transmission — full exposure to PSPS shutoffs
pub const WHOLESALE_PURCHASE_FRACTION: f64 = 1.0;
