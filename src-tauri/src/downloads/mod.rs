use std::path::Path;
use std::io::Error as IoError;
use tauri::ipc::Channel;
use thiserror::Error;
use serde::Serialize;
use reqwest::Error as ReqError;
use crate::util::url::{
    Url,
    UrlParser,
    ParseError,
};

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
    FileWriteError(String, String),
    #[error("Internal Error: {0}")]
    InternalError(String),
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "status", content = "content")]
pub enum DownloadEvent {
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

pub trait Downloader<U>
    where
        U: Url
{
    fn is_valid_url(url: &U) -> bool;
    async fn get_download_info(&mut self, url: &U) -> Result<DownloadInfo, DownloadError>;
    fn early_download_cancel(&mut self, id: u64) -> bool;
    async fn start_download(&mut self, id: u64, path: &Path, chan: Channel<DownloadEvent>) -> Result<(), DownloadError>;
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadInfo{
    id: u64,
    url: String,
    default_name: String,
    total_bytes: Option<u64>,
}

pub struct Download<S> {
    info: DownloadInfo,
    source: S,
}


pub trait DownloadFacade<P, U>
    where
        P: UrlParser<U>,
        U: Url
{
    async fn get_download_info(&mut self, link: String) -> Result<DownloadInfo, DownloadError>;
    fn early_download_cancel(&mut self, id: u64) -> Result<(), ParseError>;
    async fn start_download(&mut self, id: u64, chan: Channel<DownloadEvent>) -> Result<(), DownloadError>;
}
