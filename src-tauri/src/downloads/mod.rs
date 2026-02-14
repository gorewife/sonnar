use std::path::Path;
use std::io::Error as IoError;
use thiserror::Error;
use serde::Serialize;
use crate::util::url::Url;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Lost connection with host of {0}")]
    Disconnected(String),
    #[error("Download Request {0} failed before start")]
    BadRequest(String),
    #[error("Coudn't create the download file at {0}")]
    FileNotCreated(String),
    #[error("Download crashed with error: {0}")]
    DownloadCrash(IoError)
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "status", content = "content")]
enum DownloadEvent {
    Started {
        id: usize,
        url: String,
        total_bytes: Option<usize>,
    },
    Progress {
        id: usize,
        current_byte: usize,
    },
    Finished {
        id: usize,
    },
}

pub trait Downloader<T: Url> {
    fn is_valid_url(url: &T) -> bool;
    async fn start_download(&self ,url: &T, path: Path) -> Result<(), DownloadError>;
}
