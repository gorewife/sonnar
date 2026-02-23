use super::DownloadError;

pub trait Downloader {
    is_valid_url(&Url) -> bool;
    async start_download(&Url) -> Result<Download, DownloadError>;
}