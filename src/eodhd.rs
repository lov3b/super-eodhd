use std::{fmt::Display, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use colored::{ColoredString, Colorize};
use futures::lock::Mutex;
use reqwest::Client;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::models::ExchangeSymbol;
use crate::models::Intraday;

const TIMEDELTA: TimeDelta = TimeDelta::days(120);
const API_URL: &'static str = "https://eodhd.com/api";

pub struct Eodhd<T>
where
    T: Display,
{
    client: Client,
    api_token: T,
    total_requests: Arc<Mutex<usize>>,
    get_url: ColoredString,
    error: ColoredString,
    lower_intraday_bound_timestamp: DateTime<Utc>,
}

impl<T> Eodhd<T>
where
    T: Display,
{
    pub fn new(token: T) -> Self {
        let lower = Utc.with_ymd_and_hms(2020, 10, 1, 0, 0, 0);
        let lower_intraday_bound_timestamp = lower.unwrap();

        Self {
            client: Client::new(),
            api_token: token,
            total_requests: Arc::new(Mutex::new(0)),
            get_url: "GET URL".bold(),
            error: "ERROR".red(),
            lower_intraday_bound_timestamp,
        }
    }

    pub async fn get_high_resolution_historical_data(
        &self,
        ticker: impl Display,
        exchange_short_code: impl Display,
        to_date: Option<DateTime<Utc>>,
        max_from_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Intraday>> {
        let mut to_date = to_date.unwrap_or_else(|| chrono::Local::now().to_utc());
        let max_from_date = max_from_date.unwrap_or_else(|| self.lower_intraday_bound_timestamp);

        let v_size = {
            let week_days = (to_date - max_from_date).num_days() * 5 / 7;
            if week_days < 0 {
                return Err(anyhow!("to_date must be more than max_from_date"));
            }
            week_days as usize
        };

        let mut intradays = Vec::with_capacity(v_size);
        while to_date > max_from_date {
            let from_date = if max_from_date - to_date >= TIMEDELTA {
                max_from_date
            } else {
                to_date - TIMEDELTA
            };

            let url = format!(
            "{API_URL}/intraday/{ticker}.{exchange_short_code}?api_token={}&interval=5m&fmt=json&from={}&to={}",
            self.api_token, from_date.timestamp(), to_date.timestamp());

            let mut result = self
                .get_url::<Vec<Value>, _>(&url, Some(5))
                .await?
                .into_iter()
                .map(serde_json::from_value)
                .filter_map(Result::ok)
                .collect::<Vec<Intraday>>();

            intradays.append(&mut result);

            to_date -= TIMEDELTA;
        }

        Ok(intradays)
    }

    pub async fn get_exchange_symbols(
        &self,
        exchange_short_code: impl Display,
    ) -> Result<Vec<ExchangeSymbol>> {
        let url = format!(
            "{API_URL}/exchange-symbol-list/{exchange_short_code}?api_token={}&fmt=json",
            self.api_token
        );

        Ok(self
            .get_url::<Vec<Value>, _>(&url, Some(10))
            .await?
            .into_iter()
            .map(serde_json::from_value)
            .filter_map(Result::ok)
            .collect())
    }

    /**
     * Will return Default::default() if 404 is gotten
     */
    async fn get_url<'a, D, U>(
        &'a self,
        url: &'a U,
        increment_requests_by: Option<usize>,
    ) -> Result<D>
    where
        D: DeserializeOwned + Default,
        &'a U: IntoUrl + Display,
    {
        println!("url {}", &url);
        let response = {
            let mut i = 0;
            loop {
                if let Some(increment) = increment_requests_by {
                    let mut total_requests = self.total_requests.lock().await;
                    (*total_requests) += increment;
                }

                i += 1;
                if i > 10 {
                    return Err(anyhow!("Got status code '429' 10 times in a row"));
                }

                let response = match self.client.get(url).send().await {
                    Ok(resp) => resp,
                    Err(e) => {
                        eprintln!("[{}] ({}) for url '{}'", self.get_url, self.error, &url);
                        return Err(e.into());
                    }
                };
                if response.status().as_u16() == 429 {
                    let sleep_seconds = i * 2;
                    eprintln!(
                        "[{}] ({}) Got statuscode 429. Sleeping for {}s",
                        self.get_url, self.error, sleep_seconds
                    );
                    tokio::time::sleep(Duration::from_secs(sleep_seconds)).await;
                    continue;
                }
                break response;
            }
        };

        if response.status().as_u16() == 404 {
            return Ok(Default::default());
        }

        if !response.status().is_success() {
            return Err(anyhow!(
                "Statuscode was {} for url '{}'",
                response.status().as_u16(),
                &url
            ));
        }

        Ok(response.json().await?)
    }
}
