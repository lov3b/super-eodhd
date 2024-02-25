use anyhow::Result;
use structopt::StructOpt;
use super_eodhd::{db::Db, dump_routines, eodhd::Eodhd};

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    let dump = opt.is_dump();
    let co = opt.drain_opts();
    let db = Db::new(co.username, co.password, co.host, co.db_name).await?;
    let client = Eodhd::new(co.api_key, tokio::time::Duration::from_millis(700));

    if dump {
        match dump_routines::dump("US", client, db, co.threads).await {
            Ok(_) => println!("Done"),
            Err(e) => eprintln!("{:?}", e),
        }
    } else {
        println!("Update isn't supported yet");
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

/// Synchronizer/Cloner of EODHD
#[derive(StructOpt, Debug)]
#[structopt(name = "super-eodhd")]
enum Opt {
    /// Dump the database.
    Dump(CommonOpts),

    /// Update the database.
    Update(CommonOpts),
}
impl Opt {
    #[allow(unused)]
    pub fn get_common_opts(&self) -> &CommonOpts {
        match self {
            Self::Dump(co) => co,
            Self::Update(co) => co,
        }
    }

    pub fn drain_opts(self) -> CommonOpts {
        match self {
            Self::Dump(co) => co,
            Self::Update(co) => co,
        }
    }

    pub fn is_dump(&self) -> bool {
        match self {
            Self::Dump(_) => true,
            Self::Update(_) => false,
        }
    }
}
