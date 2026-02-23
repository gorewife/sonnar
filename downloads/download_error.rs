use thiserror.Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Lost connection with host of {0}")]
    Disconnected(String),
}