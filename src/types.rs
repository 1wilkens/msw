use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub isin: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitValue {
    pub value: f32,
    pub unit_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatedValue {
    pub original_value: String,
    pub translations: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Underlying {
    // general information
    pub underlying_isin: String,
    pub underlying_name: String,
    // strike
    pub strike_value: UnitValue,
    pub strike_diff: UnitValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivativeMasterData {
    // identifiers
    pub isin: String,
    pub wkn: String,
    // dates
    pub final_valuation_date: String,
    pub first_listing_date: String,
    pub last_listing_date: String,
    // type
    pub participation_type_name: String,
    // numbers
    pub figure_option_delta: f32,
    pub figure_option_gamma: f32,
    pub figure_option_leverage: f32,
    pub figure_option_omega: f32,
    pub figure_option_rho: f32,
    pub figure_option_theta: f32,
    pub figure_option_time: f32,
    pub figure_option_vega: f32,
    // quotes
    pub figure_quote_ask: UnitValue,
    pub figure_quote_bid: UnitValue,
    // spread
    pub figure_spread_hom: f32,
    pub figure_spread_rel: f32,
    // underlyings
    pub underlyings: Vec<Underlying>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityMasterData {
    // identifiers
    pub isin: String,
    pub industry_sector: TranslatedValue,
    pub instrument_type: TranslatedValue,
}
