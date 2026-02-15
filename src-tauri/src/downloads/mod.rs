use std::path::Path;
use std::io::Error as IoError;
use tauri::ipc::Channel;
use thiserror::Error;
use serde::Serialize;
use reqwest::Error as ReqError;
use crate::util::url::Url;

mod prototype;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Lost connection with host of {0}")]
    Disconnected(String),
    #[error("Download Request {0} failed: {1}")]
    BadRequest(String, reqwest::Error),
    #[error("Coudn't create the download file at {0}: {1}")]
    FileNotCreated(String, IoError),
    #[error("Download {0} crashed with error: {1}")]
    DownloadCrash(u64, ReqError),
    #[error("Failed to write file at {0}: {1}")]
    FileWriteError(String, IoError),
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "status", content = "content")]
enum DownloadEvent {
    Started {
        id: u64,
        url: String,
        total_bytes: Option<u64>,
    },
    Progress {
        id: u64,
        current_byte: u64,
    },
    Finished {
        id: u64,
    },
}

pub trait Downloader<T: Url> {
    fn is_valid_url(url: &T) -> bool;
    async fn start_download(&self, id: u64,url: &T, path: &Path, chan: Channel<DownloadEvent>) -> Result<(), DownloadError>;
}
