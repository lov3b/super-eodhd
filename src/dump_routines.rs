use anyhow::Result;
use colored::Colorize;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Display;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::signal;
use tokio::sync::{Mutex, Semaphore};
use tokio::{fs::File, io::AsyncReadExt};

use crate::models::ExchangeSymbol;
use crate::{db::Db, eodhd::Eodhd};

pub async fn dump<T, Ex>(
    exchange_short_code: Ex,
    eodhd: Eodhd<T>,
    db: Db,
    threads: usize,
) -> Result<()>
where
    T: Display + Send + Sync + 'static,
    Ex: Display,
{
    let (dump_txt, error_txt) = ("DUMP".bold().magenta(), "ERROR".red());
    println!("[{}] Starting dump", &dump_txt);
    let state_file = "has-finished-prices.json";
    let (eodhd, db) = (Arc::new(eodhd), Arc::new(db));

    let has_finished_prices: bool = aux_config_load(state_file).await.unwrap_or_default();

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
                if let Err(e) = aux_config_save(state_file, true).await {
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
    T: Display + Send + Sync + 'static,
    Ex: Display,
{
    let (dump_prices_txt, error_txt) = (
        Arc::new("DUMP PRICES".bold().purple()),
        Arc::new("ERROR".red()),
    );
    let max_errors_in_row: usize = threads * 2;

    let (downloaded, failed) = get_filter::<Arc<str>>().await;
    let mut filter_content = Vec::with_capacity(downloaded.len() + failed.len());

    downloaded
        .iter()
        .map(Arc::clone)
        .for_each(|x| filter_content.push(x));
    let downloaded = Arc::new(Mutex::new(downloaded));

    failed
        .iter()
        .map(Arc::clone)
        .for_each(|x| filter_content.push(x));
    let failed = Arc::new(Mutex::new(failed));

    let (ctrl_c_handler, ctrl_c_break_switch) = get_ctrl_c_handler(
        "downloaded.json",
        downloaded.clone(),
        "failed.json",
        failed.clone(),
    );

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

    let mut exited_prematurly = ExitedPrematurly::NO;
    for symbol in symbols {
        {
            let (errors_lock, ctrl_c_switch) = (errors_in_row.lock(), ctrl_c_break_switch.lock());
            let (errors_lock, ctrl_c_switch) = tokio::join!(errors_lock, ctrl_c_switch);
            if *errors_lock > max_errors_in_row {
                eprintln!(
                    "[{}] We have failed {} times in a row. Exiting now...",
                    &dump_prices_txt, max_errors_in_row
                );
                exited_prematurly = ExitedPrematurly::YES;
                break;
            }
            if *ctrl_c_switch {
                eprintln!("[{}] Ctrl+C is pressed. Exiting...", &dump_prices_txt);
                break;
            }
        }

        let code_exchange = format!("{}.{}", symbol.code.as_ref(), symbol.exchange.as_ref());
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let db = db.clone();
        let errors_in_row = errors_in_row.clone();
        let eodhd = eodhd.clone();
        let failed = failed.clone();
        let downloaded = downloaded.clone();
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
                let mut failed_lock = failed.lock().await;
                failed_lock.push(code_exchange.into());
            } else {
                let mut downloaded_lock = downloaded.lock().await;
                downloaded_lock.push(code_exchange.into());
            }
        });
        handles.push(handle);
    }

    let (downloads, failed, errors_in_row) =
        (downloaded.lock(), failed.lock(), errors_in_row.lock());
    let (downloads, failed, errors_in_row) = tokio::join!(downloads, failed, errors_in_row);

    let downloads = &*downloads;
    let (failed, removed_fails) = failed.split_at(failed.len().saturating_sub(*errors_in_row));
    //let downloads = *downloads;
    let saves = (
        aux_config_save("./downloaded.json", downloads),
        aux_config_save("./failed.json", failed),
        aux_config_save("./last_fails_which_are_removed.json", removed_fails),
    );
    let results = tokio::join!(saves.0, saves.1, saves.2);
    for result in [results.0, results.1, results.2] {
        if let Err(e) = result {
            eprintln!(
                "[{}] ({}) occured while saving configuration: {:?}",
                &dump_prices_txt, &error_txt, e
            );
        }
    }

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

    Ok(exited_prematurly)
}

fn get_ctrl_c_handler<T, S>(
    downloaded_file_name: T,
    downloaded: Arc<Mutex<Vec<S>>>,
    failed_file_name: T,
    failed: Arc<Mutex<Vec<S>>>,
) -> (tokio::task::JoinHandle<()>, Arc<Mutex<bool>>)
where
    T: AsRef<Path> + Display + Send + Sync + 'static,
    S: Serialize + Send + Sync + 'static,
{
    let break_switch = Arc::new(Mutex::new(false));
    let break_switch_c = break_switch.clone();
    let handle = tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for ctrl_c");
        let ctrl_c_txt = "CTRL C HANDLER".blue().bold();
        let error_txt = "ERROR".red();
        println!(
            "[{}] Ctrl+C received! Saving {} and {}...",
            &ctrl_c_txt, &downloaded_file_name, &failed_file_name
        );

        {
            let (downloaded, failed) = (downloaded.lock(), failed.lock());
            let (downloaded, failed) = tokio::join!(downloaded, failed);
            let (downloaded, failed) = (downloaded.as_slice(), failed.as_slice());
            let (downloaded_status, failed_status) = tokio::join!(
                aux_config_save(&downloaded_file_name, downloaded),
                aux_config_save(&failed_file_name, failed)
            );

            for (file_name, status) in [
                (&downloaded_file_name, downloaded_status),
                (&failed_file_name, failed_status),
            ] {
                match status {
                    Ok(_) => println!("[{}] Saved {}", &ctrl_c_txt, file_name),
                    Err(e) => eprintln!(
                        "[{}] ({}) Failed to save {file_name} with: {:?}",
                        &ctrl_c_txt, &error_txt, &e
                    ),
                }
            }
        }

        let mut lock = break_switch_c.lock().await;
        *lock = true;
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
            "EMPTY".blink()
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

async fn get_filter<S>() -> (Vec<S>, Vec<S>)
where
    S: Display + DeserializeOwned,
{
    let get_filter = "GET FILTER".bold().blue();
    let error = "ERROR".red();
    let download_future = aux_config_load("./downloaded.json");
    let failed_future = aux_config_load("./failed.json");

    let (downloaded, failed) = tokio::join!(download_future, failed_future);
    if let Err(_) = downloaded {
        eprintln!(
            "[{}] ({}) Downloaded filter is empty (downloaded.json)",
            &get_filter, &error
        );
    }
    if let Err(_) = failed {
        eprintln!(
            "[{}] ({}) Failed filter is empty (failed.json)",
            &get_filter, &error
        );
    }

    let downloaded: Vec<_> = downloaded.unwrap_or_default();
    let failed: Vec<_> = failed.unwrap_or_default();

    (downloaded, failed)
}

async fn aux_config_load<T: DeserializeOwned>(file_name: impl AsRef<Path>) -> Result<T> {
    let mut file = File::open(file_name).await?;

    let size = file.metadata().await?.size() as usize;
    let mut contents = Vec::with_capacity(size);
    file.read_to_end(&mut contents).await?;

    Ok(serde_json::from_slice(&contents)?)
}

async fn aux_config_save<T: Serialize>(file_name: impl AsRef<Path>, conf: T) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file_name)
        .await?;
    let bytes = serde_json::to_vec(&conf)?;
    let mut bytes = bytes.as_ref();
    file.write_all_buf(&mut bytes).await?;
    Ok(())
}
