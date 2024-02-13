use crate::models::{ExchangeSymbol, Intraday};
use anyhow::Result;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{FromRow, MySql, Pool, Row};
use std::fmt::Display;

#[derive(FromRow, Debug)]
pub struct OutdatedSymbolPrice {
    pub code: Box<str>,
    pub last_updated: chrono::NaiveDateTime,
}

#[derive(Debug, FromRow)]
pub struct OutdatedSymbolPriceEOD {
    pub code: Box<str>,
    pub last_updated: Option<chrono::NaiveDate>,
}

#[derive(Debug, FromRow)]
pub struct OutdatedSymbolNews {
    pub code: Box<str>,
    pub exchange: Box<str>,
    pub last_updated: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, FromRow)]
pub struct OutdatedSymbolFundamental {
    pub code: Box<str>,
    pub exchange: Box<str>,
}

pub struct Db {
    pool: Pool<MySql>,
}

impl Db {
    pub async fn new(
        username: impl Display,
        password: impl Display,
        host: impl Display,
        db_name: impl Display,
    ) -> sqlx::Result<Self> {
        let connect_string = format!("mysql://{}:{}@{}/{}", username, password, host, db_name);
        let pool = MySqlPoolOptions::new().connect(&connect_string).await?;
        Ok(Self { pool })
    }

    pub async fn push_intraday(
        &self,
        code: &str,
        exchange: &str,
        intraday_prices: &Vec<Intraday>,
    ) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        for intraday in intraday_prices {
            sqlx::query("INSERT IGNORE INTO StockPrice (code, exchange, timestamp, gmtoffset, open, high, low, close, volume) VALUES (?, ?, FROM_UNIXTIME(?), ?, ?, ?, ?, ?, ?)")
            .bind(code) // Convert to String once
            .bind(exchange) // Convert to String once
            .bind(intraday.timestamp)
            .bind(intraday.gmt_offset)
            .bind(intraday.open)
            .bind(intraday.high)
            .bind(intraday.low)
            .bind(intraday.close)
            .bind(intraday.volume)
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    pub async fn push_exchange_symbols(
        &self,
        exchange_short_code: &str,
        symbols: Vec<ExchangeSymbol>,
    ) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        for symbol in symbols {
            // For US stocks EODHD stores their exchange as 'US' for all exchanges
            let real_exchange = symbol.exchange.as_ref();
            sqlx::query("INSERT IGNORE INTO ExchangeSymbol (name, code, exchange, type, country, currency, isin, realExchange) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&symbol.name)
            .bind(&symbol.code)
            .bind(exchange_short_code)
            .bind(&symbol.symbol_type)
            .bind(&symbol.country)
            .bind(&symbol.currency)
            .bind(&symbol.isin)
            .bind(real_exchange)
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    pub async fn get_outdated_symbol_prices(
        &self,
        exchange_short_code: &str,
    ) -> sqlx::Result<Vec<OutdatedSymbolPrice>> {
        let result = sqlx::query_as::<_, OutdatedSymbolPrice>(
            "SELECT ES.code, MAX(SP.timestamp) as last_updated
             FROM ExchangeSymbol AS ES
             LEFT JOIN DownloadedSymbol AS DS ON ES.code = DS.code AND ES.exchange = DS.exchange
             LEFT JOIN StockPrice SP on ES.code = SP.code AND ES.exchange = SP.exchange
             WHERE ES.exchange = ?
             GROUP BY ES.code
             HAVING MAX(SP.timestamp) IS NULL OR DATE(MAX(SP.timestamp)) < DATE_SUB(CURDATE(), INTERVAL 21 DAY)"
        )
        .bind(exchange_short_code)
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }
    pub async fn get_outdated_symbol_prices_eod(
        &self,
        exchange_short_code: &str,
    ) -> sqlx::Result<Vec<OutdatedSymbolPriceEOD>> {
        let result = sqlx::query_as::<_, OutdatedSymbolPriceEOD>(
            "SELECT es.code, MAX(sp.date) as last_updated
             FROM ExchangeSymbol es
             LEFT JOIN StockPriceEOD sp ON es.code = sp.code AND es.exchange = sp.exchange
             WHERE es.exchange = ?
             GROUP BY es.code
             HAVING MAX(sp.date) IS NULL OR DATE(MAX(sp.date)) < DATE_SUB(CURDATE(), INTERVAL 21 DAY)"
        )
        .bind(exchange_short_code)
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn get_outdated_symbols_news(
        &self,
        exchange_short_code: &str,
    ) -> sqlx::Result<Vec<OutdatedSymbolNews>> {
        let result = sqlx::query_as::<_, OutdatedSymbolNews>(
            "SELECT es.code, es.exchange, MAX(NA.date) as last_updated
             FROM ExchangeSymbol es
             LEFT JOIN NewsSymbol NS ON es.code = NS.code AND es.exchange = NS.exchange
             LEFT JOIN NewsArticle NA on NA.id = NS.newsId
             LEFT JOIN NewsUpdated NU on es.code = NU.code AND es.exchange = NU.exchange
             WHERE es.exchange = ?
             GROUP BY es.code
             HAVING MAX(NA.date) IS NULL OR DATE(MAX(NA.date)) < DATE_SUB(CURDATE(), INTERVAL 1 DAY)"
        )
        .bind(exchange_short_code)
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn add_stage(&self, exchange_short_code: &str, stage: &str) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO StageDone (exchange, stage)
             VALUES (?, ?)
             ON DUPLICATE KEY UPDATE exchange = exchange",
        )
        .bind(exchange_short_code)
        .bind(stage)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_stages(&self, exchange_short_code: &str) -> sqlx::Result<Vec<Box<str>>> {
        let stages = sqlx::query("SELECT stage FROM StageDone WHERE exchange = ?")
            .bind(exchange_short_code)
            .map(|row: sqlx::mysql::MySqlRow| row.get(0))
            .fetch_all(&self.pool)
            .await?;
        Ok(stages)
    }

    pub async fn update_downloaded_symbol(
        &self,
        code: &str,
        exchange: &str,
        is_empty: bool,
    ) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO DownloadedSymbol (code, exchange, isEmpty) 
             VALUES (?, ?, ?)
             ON DUPLICATE KEY UPDATE code = code",
        )
        .bind(code)
        .bind(exchange)
        .bind(is_empty)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_outdated_symbols_fundamental(
        &self,
        exchange_short_code: &str,
    ) -> sqlx::Result<Vec<OutdatedSymbolFundamental>> {
        let result = sqlx::query_as::<_, OutdatedSymbolFundamental>(
            "SELECT es.code, es.exchange
             FROM ExchangeSymbol es
             LEFT JOIN FundamentalMetadata fm ON es.code = fm.code AND es.exchange = fm.exchange
             WHERE es.exchange = ?
             GROUP BY es.code
             HAVING MAX(fm.UpdatedAt) IS NULL OR DATE(MAX(fm.UpdatedAt)) < DATE_SUB(CURDATE(), INTERVAL 1 DAY)"
        )
        .bind(exchange_short_code)
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }
}
