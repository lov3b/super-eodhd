use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Intraday {
    pub timestamp: i64,
    #[serde(rename = "gmtoffset")]
    pub gmt_offset: i32,
    #[serde(
        deserialize_with = "deserialize_datetime",
        serialize_with = "serialize_datetime"
    )]
    pub datetime: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}
fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let naivedt =
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)?;
    Ok(DateTime::from_naive_utc_and_offset(naivedt, Utc))
}

fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = date.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&s)
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
