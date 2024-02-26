use anyhow::Result;
use colored::Colorize;
use futures::Future;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;
use tokio::sync::{Mutex, Semaphore};

use crate::config::{load_serializable, save_serializable_generic, Config, SyncedConfig};
use crate::models::ExchangeSymbol;
use crate::{db::Db, eodhd::Eodhd};

pub async fn dump<T, Ex>(
    exchange_short_code: Ex,
    eodhd: Eodhd<T>,
    db: Db,
    threads: usize,
) -> Result<()>
where
    T: Display + Send + Sync + 'static + Serialize,
    Ex: Display,
{
    let (dump_txt, error_txt) = ("DUMP".bold().magenta(), "ERROR".red());
    println!("[{}] Starting dump", &dump_txt);
    let state_file = "has-finished-prices.json";
    let (eodhd, db) = (Arc::new(eodhd), Arc::new(db));

    let has_finished_prices: bool = load_serializable(state_file).await.unwrap_or_default();

    if !has_finished_prices {
        match dump_prices(&exchange_short_code, eodhd.clone(), db.clone(), threads).await? {
            ExitedPrematurly::YES => {
                println!(
                    "[{}] Exited prematurly from dumping prices. Will shut down now",
                    &dump_txt
                );
                return Ok(());
            }

            ExitedPrematurly::NO => {
                println!(
                    "[{}] Everything went well with dumping prices. Will proceed to fundamental",
                    &dump_txt
                );
                if let Err(e) = save_serializable_generic(state_file, true).await {
                    eprintln!("[{}] ({}) Failed to write to '{state_file}'. Will pass on error now. Please remember that we has finished prices",&dump_txt, &error_txt);
                    return Err(e);
                };
            }
        }
    }

    Ok(())
}

async fn dump_prices<T, Ex>(
    exchange_short_code: Ex,
    eodhd: Arc<Eodhd<T>>,
    db: Arc<Db>,
    threads: usize,
) -> Result<ExitedPrematurly>
where
    T: Display + Send + Sync + 'static + Serialize,
    Ex: Display,
{
    let (dump_prices_txt, error_txt) = (
        Arc::new("DUMP PRICES".bold().purple()),
        Arc::new("ERROR".red()),
    );
    let max_errors_in_row: usize = threads * 2;

    let config = Arc::new(SyncedConfig::<Arc<str>>::load().await);
    let filter_content = config.get_filter().await;
    let (ctrl_c_handler, cancellation_token) = {
        let local_config = config.clone();

        let a = async move {
            local_config
                .save(None)
                .await
                .expect("Failed to save config");
        };
        get_ctrl_c_handler(a)
    };

    if filter_content.is_empty() {
        println!("[{}] Filter files was empty", &dump_prices_txt);
    }
    let filter: HashSet<&str> = HashSet::from_iter(filter_content.iter().map(|x| x.as_ref()));
    let symbols = eodhd.get_exchange_symbols(exchange_short_code).await?;
    let symbols = symbols.into_iter().filter(|symbol| {
        let s = format!("{}.{}", symbol.code, symbol.exchange);
        !filter.contains(s.as_str())
    });

    let semaphore = Arc::new(Semaphore::new(threads));
    let errors_in_row = Arc::new(Mutex::new(0 as usize));
    let mut handles = Vec::new();

    for symbol in symbols {
        {
            let errors_lock = errors_in_row.lock().await;
            if *errors_lock > max_errors_in_row {
                eprintln!(
                    "[{}] We have failed {} times in a row. Exiting now...",
                    &dump_prices_txt, max_errors_in_row
                );
                config.save(None).await.expect("Failed to save config");
                return Ok(ExitedPrematurly::YES);
            }
            if cancellation_token.load(Ordering::SeqCst) {
                eprintln!("[{}] Ctrl+C is pressed. Exiting...", &dump_prices_txt);
                return Ok(ExitedPrematurly::YES);
            }
        }

        let code_exchange = format!("{}.{}", symbol.code.as_ref(), symbol.exchange.as_ref());
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let db = db.clone();
        let errors_in_row = errors_in_row.clone();
        let eodhd = eodhd.clone();
        let config = config.clone();
        let (dump_prices_txt, error_txt) = (dump_prices_txt.clone(), error_txt.clone());
        let handle = tokio::spawn(async move {
            if let Err(e) = process_symbol(permit, dump_prices_txt.clone(), eodhd, symbol, db).await
            {
                let errors = {
                    let mut errors_lock = errors_in_row.lock().await;
                    (*errors_lock) += 1;
                    *errors_lock
                };
                eprintln!(
                    "[{}] ({}) ({}/{}) Failed to download/push {code_exchange} with error: {:?}",
                    &dump_prices_txt, &error_txt, errors, max_errors_in_row, &e
                );
                config.append_failure(code_exchange.into()).await;
            } else {
                config.append_download(code_exchange.into()).await;
            }
        });
        handles.push(handle);
    }

    config
        .save(Some(*errors_in_row.lock().await))
        .await
        .expect("Failed to save config/state");

    let dump_prices_txt = "DUMP PRICES".bold().blue();
    println!("[{}] Waiting for all tasks to finish...", &dump_prices_txt);
    for handle in handles.into_iter() {
        if let Err(e) = handle.await {
            eprintln!(
                "[{}] ({}) Encountered while awaiting all handles {:?}",
                &dump_prices_txt, &error_txt, &e
            );
        }
    }
    println!("[{}] Aborting ctrl-c handler...", &dump_prices_txt);
    ctrl_c_handler.abort();

    Ok(ExitedPrematurly::NO)
}

fn get_ctrl_c_handler<F>(save: F) -> (tokio::task::JoinHandle<()>, Arc<AtomicBool>)
where
    F: Future<Output = ()> + Send + 'static,
{
    let break_switch = Arc::new(AtomicBool::new(false));
    let break_switch_c = break_switch.clone();
    let handle = tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for ctrl_c");
        let ctrl_c_txt = "CTRL C HANDLER".blue().bold();
        println!("[{}] Ctrl+C received! Saving state", &ctrl_c_txt,);
        save.await;
        break_switch_c.store(true, Ordering::SeqCst);
    });
    (handle, break_switch)
}
enum ExitedPrematurly {
    YES,
    NO,
}
impl Default for ExitedPrematurly {
    fn default() -> Self {
        Self::NO
    }
}

async fn process_symbol<T, D>(
    _permit: tokio::sync::OwnedSemaphorePermit,
    dump_prices_txt: D,
    eodhd: Arc<Eodhd<T>>,
    symbol: ExchangeSymbol,
    db: Arc<Db>,
) -> Result<()>
where
    T: Display + Send + Sync + 'static,
    D: Display + Send + Sync + 'static,
{
    let intraday_prices = eodhd
        .get_high_resolution_historical_data(symbol.code.as_ref(), "US", None, None)
        .await?;

    if intraday_prices.is_empty() {
        println!(
            "[{}] {}.US (isin: {}) {}",
            &dump_prices_txt,
            &symbol.code,
            symbol.isin.as_ref().map(AsRef::as_ref).unwrap_or("missing"),
            "EMPTY".underline()
        );
        return Ok(());
    }

    db.push_intraday(symbol.code.as_ref(), "US", &intraday_prices)
        .await?;

    println!(
        "[{}] {}.US (isin: {}) {}st",
        &dump_prices_txt,
        &symbol.code,
        symbol.isin.as_ref().map(AsRef::as_ref).unwrap_or("missing"),
        intraday_prices.len()
    );

    Ok(())
}

pub async fn selective_sync<T, S, Ex>(
    exchange_short_code: Ex,
    short_codes: Vec<S>,
    eodhd: &Eodhd<T>,
    db: &Db,
) where
    T: Display,
    S: Display,
    Ex: Display,
{
    let fn_text = format!("[{}]", "SELECTIVE SYNC".bold().yellow());
    let exchange_short_code = exchange_short_code.to_string();
    if let Err(_) = sync_metadata(&exchange_short_code, eodhd, db).await {
        return;
    }
    eprintln!("{} Syncing {} instruments", &fn_text, short_codes.len());

    for short_code in short_codes {
        let short_code = short_code.to_string().to_uppercase();
        let mut download_txt = fn_text.clone();
        download_txt.push(' ');
        download_txt.push_str(&short_code);

        let data = match eodhd
            .get_high_resolution_historical_data(&short_code, &exchange_short_code, None, None)
            .await
        {
            Ok(k) => k,
            Err(e) => {
                download_txt.push_str(format!(", Failed with error: {:?} ", e).as_str());
                eprintln!("{}", download_txt);
                continue;
            }
        };

        download_txt.push_str(format!(", with {}st points", data.len()).as_str());
        if let Err(e) = db
            .push_intraday(&short_code, &exchange_short_code, &data)
            .await
        {
            download_txt.push_str(format!(", failed to push to DB with error: {:?}", e).as_str());
        }

        eprintln!("{}", download_txt);
    }
}

async fn sync_metadata<T>(exchange_short_code: &str, eodhd: &Eodhd<T>, db: &Db) -> Result<()>
where
    T: Display,
{
    let fn_text = "SYNC METADATA".bold().green();
    eprintln!("{} Downloading all instrument metadatas", &fn_text);
    // Make sure that we have the symbols in the DB
    let all_instruments = match eodhd.get_exchange_symbols(&exchange_short_code).await {
        Ok(k) => k,
        Err(e) => {
            eprintln!(
                "{} Failed to download instrument metadatas with error: {:?}",
                &fn_text, &e
            );
            return Err(e);
        }
    };
    eprintln!("{} Pushing metdata to DB", &fn_text);
    if let Err(e) = db
        .push_exchange_symbols(&exchange_short_code, all_instruments)
        .await
    {
        eprintln!(
            "{} Failed to push instrument metadatas with error: {:?}",
            &fn_text, &e
        );
        return Err(e);
    }
    eprintln!("{} Done", fn_text);
    Ok(())
}
