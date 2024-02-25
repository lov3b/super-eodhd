use anyhow::{bail, Result};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{os::unix::fs::MetadataExt, path::Path, sync::Arc};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

pub const DOWNLOADED_FILE_NAME: &'static str = "downloaded.json";
pub const FAILED_FILE_NAME: &'static str = "failed.json";

#[async_trait]
pub trait Config<S>
where
    S: Serialize + DeserializeOwned,
{
    async fn save(&self, not_the_last: Option<usize>) -> Result<()>;
    async fn load() -> Self;
    async fn get_filter(&self) -> Vec<S>;
    async fn append_failure(&self, value: S);
    async fn append_download(&self, value: S);
}

pub struct SyncedConfig<S>
where
    S: Serialize + DeserializeOwned + Send + Sync + 'static + Clone,
{
    pub downloaded: Arc<Mutex<Vec<S>>>,
    pub failed: Arc<Mutex<Vec<S>>>,
}

#[async_trait]
impl<S> Config<S> for SyncedConfig<S>
where
    S: DeserializeOwned + Serialize + Send + Sync + 'static + Clone,
{
    async fn save(&self, not_the_last: Option<usize>) -> Result<()> {
        let (downloaded, failed) = (self.downloaded.lock(), self.failed.lock());
        let (downloaded, failed) = tokio::join!(downloaded, failed);
        let failed = if let Some(not_the_last) = not_the_last {
            let (failed, removed_fails) =
                failed.split_at(failed.len().saturating_sub(not_the_last));
            save_serializable_iter("./last_fails_which_are_removed.json", removed_fails).await.expect("Failed to save removed fails");
            failed
        } else {
            failed.as_ref()
        };

        let (status_downloaded, status_failed) = (
            save_serializable_iter("./downloaded.json", downloaded.as_ref()),
            save_serializable_iter("./failed.json", failed),
        );
        let (status_downloaded, status_failed) = tokio::join!(status_downloaded, status_failed);
        if let Err(err_downloaded) = status_downloaded {
            if let Err(err_failed) = status_failed {
                bail!(
                    "Neither downloaded.json nor failed.json succedded. Errors was: '{:?}', '{:?}'",
                    err_downloaded,
                    err_failed
                );
            }
            return Err(err_downloaded);
        }

        Ok(())
    }

    async fn load() -> Self {
        let (downloaded, failed) = (
            load_serializable("./downloaded.json"),
            load_serializable("./failed.json"),
        );
        let (downloaded, failed) = tokio::join!(downloaded, failed);
        let downloaded = Arc::new(Mutex::new(downloaded.unwrap_or_default()));
        let failed = Arc::new(Mutex::new(failed.unwrap_or_default()));

        Self { downloaded, failed }
    }

    async fn get_filter(&self) -> Vec<S> {
        let (downloaded, failed) = tokio::join!(self.downloaded.lock(), self.failed.lock());
        let mut ret = Vec::with_capacity(downloaded.len() + failed.len());
        downloaded.iter().cloned().for_each(|x| ret.push(x));
        failed.iter().cloned().for_each(|x| ret.push(x));

        ret
    }

    async fn append_failure(&self, value: S) {
        let mut failed = self.failed.lock().await;
        failed.push(value);
    }

    async fn append_download(&self, value: S) {
        let mut downloaded = self.downloaded.lock().await;
        downloaded.push(value);
    }
}

/**
 * For some reason rust complains about the values not living long enough in SyncedConfig::save if we don't have &[S]
 */
async fn save_serializable_iter<S>(file_name: impl AsRef<Path>, conf: &[S]) -> Result<()>
where
    S: Serialize + Sync + Send + 'static,
{
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

pub async fn save_serializable_generic<S>(file_name: impl AsRef<Path>, conf: S) -> Result<()>
where
    S: Serialize + Sync + Send + 'static,
{
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
pub async fn load_serializable<S>(file_name: impl AsRef<Path>) -> Result<S>
where
    S: DeserializeOwned + Sync + Send + 'static,
{
    let mut file = File::open(file_name).await?;

    let size = file.metadata().await?.size() as usize;
    let mut contents = Vec::with_capacity(size);
    file.read_to_end(&mut contents).await?;

    Ok(serde_json::from_slice(&contents)?)
}
