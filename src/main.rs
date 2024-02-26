use anyhow::Result;
use structopt::StructOpt;
use super_eodhd::{
    db::Db,
    dump_routines::{self, selective_sync},
    eodhd::Eodhd,
};

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Dump(co) => {
            let db = Db::new(co.username, co.password, co.host, co.db_name).await?;
            let client = Eodhd::new(co.api_key, tokio::time::Duration::from_millis(700));
            match dump_routines::dump("US", client, db, co.threads).await {
                Ok(_) => println!("Done"),
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Opt::Selective(so) => {
            let db = Db::new(so.username, so.password, so.host, so.db_name).await?;
            let client = Eodhd::new(so.api_key, tokio::time::Duration::from_millis(700));
            selective_sync("US", so.codes, &client, &db).await;
        }
        Opt::Update(_) => {
            println!("Update isn't supported yet");
            std::process::exit(0);
        }
    }

    Ok(())
}

/// Common options for authentication and database access.
#[derive(StructOpt, Debug)]
struct CommonOpts {
    /// API key for authentication.
    #[structopt(long = "api-key")]
    api_key: String,

    /// Username for database.
    #[structopt(long = "username")]
    username: String,

    /// Password for database
    #[structopt(long = "password")]
    password: String,

    /// Database host.
    #[structopt(long = "host")]
    host: String,

    /// Name of the database.
    #[structopt(long = "db-name")]
    db_name: String,

    /// Threads to download and push to Db with
    #[structopt(long = "threads", short = "-t", default_value = "8")]
    threads: usize,
}

#[derive(StructOpt, Debug)]
struct SelectiveOpts {
    /// Short codes (US tickers) to sync
    #[structopt(long = "codes")]
    pub codes: Vec<String>,

    /// API key for authentication.
    #[structopt(long = "api-key")]
    api_key: String,

    /// Username for database.
    #[structopt(long = "username")]
    username: String,

    /// Password for database
    #[structopt(long = "password")]
    password: String,

    /// Database host.
    #[structopt(long = "host")]
    host: String,

    /// Name of the database.
    #[structopt(long = "db-name")]
    db_name: String,
}

/// Synchronizer/Cloner of EODHD
#[derive(StructOpt, Debug)]
#[structopt(name = "super-eodhd")]
enum Opt {
    /// Dump the database.
    Dump(CommonOpts),

    /// Just dump these codes
    Selective(SelectiveOpts),

    /// Update the database.
    Update(CommonOpts),
}
