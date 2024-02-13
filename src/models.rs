use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Intraday {
    #[serde(rename = "Timestamp")]
    pub timestamp: i64,
    #[serde(rename = "Gmtoffset")]
    pub gmt_offset: i32,
    #[serde(rename = "Datetime")]
    pub datetime: DateTime<Utc>,
    #[serde(rename = "Open")]
    pub open: f64,
    #[serde(rename = "High")]
    pub high: f64,
    #[serde(rename = "Low")]
    pub low: f64,
    #[serde(rename = "Close")]
    pub close: f64,
    #[serde(rename = "Volume")]
    pub volume: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeSymbol {
    #[serde(rename = "Code")]
    pub code: Box<str>,
    #[serde(rename = "Name")]
    pub name: Box<str>,
    #[serde(rename = "Country")]
    pub country: Box<str>,
    #[serde(rename = "Exchange")]
    pub exchange: Box<str>,
    #[serde(rename = "Currency")]
    pub currency: Box<str>,
    #[serde(rename = "Type")]
    pub symbol_type: Box<str>,
    #[serde(rename = "Isin")]
    pub isin: Option<Box<str>>,
}
