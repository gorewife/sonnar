use std::path::Path;

use tokio::sync::Mutex;
use tauri::{
    ipc::Channel,
    State,
    command,
};
use dashmap::DashMap;
use crate::util::url::{
    Url,
    UrlParser,
    ParseError,
    ReqwestUrl,
    ReqwestParser,
};

use super::{
    DownloadFacade,
    Downloader,
    DownloadInfo,
    DownloadError,
    DownloadEvent,
    prototype::Prototype,
};


pub struct Facade {
    prototype_downloader: Prototype,
    cache: DashMap<u64, Choice>,
}

enum Choice {
    Prototype,
}

impl DownloadFacade<ReqwestParser, ReqwestUrl> for Facade {
        async fn get_download_info(&mut self, link: String) -> Result<DownloadInfo, DownloadError> {
            match ReqwestParser::parse(link) {
                Ok(url) => match url.host() {
                    _ => {
                        let info = self.prototype_downloader.get_download_info(&url).await?;
                        let id = info.id;
                        self.cache.insert(id, Choice::Prototype);
                        Ok(info)
                    }
                },
                Err(err) => Err(DownloadError::UrlParseError(err))
            }
        }

        fn early_download_cancel(&mut self, id: u64) -> bool {
            match self.cache.remove(&id) {
                Some((_, Choice::Prototype)) => self.prototype_downloader.early_download_cancel(id),
                None => false
            }
        }

        async fn start_download(&mut self, id: u64, path: &Path, chan: Channel<DownloadEvent>) -> Result<(), DownloadError> {
            match self.cache.remove(&id) {
                Some((_, Choice::Prototype)) => Ok(self.prototype_downloader.start_download(id, path, chan).await?),
                None => Err(DownloadError::InternalError(String::from("The download isn't cached"))),
            }
        }
}

//hanlder
#[command]
async fn get_download_info(facade: &State<'_, Mutex<Facade>>, link: String) -> Result<DownloadInfo, DownloadError> {
    facade.lock()
        .await
        .get_download_info(link).await
}

#[command]
fn early_download_cancel(facade: &State<'_, Mutex<Facade>>, id: u64) -> bool {
    facade.blocking_lock()
        .early_download_cancel(id)
}

#[command]
async fn start_download(facade: &State<'_, Mutex<Facade>>, id: u64, path_string: String, chan: Channel<DownloadEvent>) -> Result<(), DownloadError> {
     let path = Path::new(&path_string);
     facade.lock()
         .await
         .start_download(id, path, chan)
         .await
}
