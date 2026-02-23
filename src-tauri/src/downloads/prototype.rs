use std::path::Path;
use std::io::Result as IoResult;
use std::sync::Arc;
use dashmap::DashMap;
use futures_lite::stream::StreamExt;
use reqwest::{
    Client, 
    Response,
};
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
};
use tauri::ipc::Channel;

use super::{
    Downloader,
    DownloadEvent,
    DownloadError,
    Download,
    DownloadInfo,
};

use crate::util::url::{
    Url, 
    ReqwestUrl,
};

pub struct Prototype {
    client: Client,
    cache: Arc<DashMap<u64, Download<Response>>>,
}

impl  Prototype {
    pub fn new() ->  Self {
        Prototype {
            client: Client::new(),
            cache: Arc::from(DashMap::new()),
        }
    }

    fn get_file_name(_res: &Response) -> String {
        "file".to_owned()
    }
}

impl  Downloader<ReqwestUrl> for Prototype {
    #[inline]
    fn is_valid_url(_: &ReqwestUrl) -> bool {
        true
    }

    fn early_download_cancel(&mut self, id: u64) -> bool {
        self.cache.remove(&id).is_some()
    }

    async fn get_download_info(&mut self, url: &ReqwestUrl) -> Result<DownloadInfo, DownloadError> {
        let response = self.client.get(url.to_owned())
            .send()
            .await;
        match response {
            Ok(res) => {
                let info = DownloadInfo {
                    id: 1,
                    url: res.url().as_str().to_owned(),
                    default_name: Self::get_file_name(&res),
                    total_bytes: res.content_length(),
                };

                let download = Download {
                    info: info.clone(),
                    source: res,
                };
                self.cache.insert(info.id, download);
                Ok(info)
            },
            Err(e) => Err(DownloadError::BadRequest(url.full_string().to_owned(), e))
        }
    }
    
    async fn start_download(&mut self, id: u64, path: &Path, chan: Channel<DownloadEvent>) -> Result<(), DownloadError> {
        let file_open = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .await;

        let cached_download = self.cache.remove(&id);

        if let Some((_, download)) = cached_download {
            let Download{source, ..} = download;
            match file_open {
                Ok(mut file) => {
                    let mut stream = source.bytes_stream();
                    let mut byte_counter = 0;
                
                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(good_chunk) => {
                                let len = good_chunk.len() as u64;
                                if let Err(err) = file.write_all(&good_chunk).await as IoResult<()> {
                                    return Err(DownloadError::FileWriteError(path.to_string_lossy().to_string(), err.to_string()))
                                }
                                byte_counter += len;
                                let progress = DownloadEvent::Progress{
                                    id,
                                    current_byte: byte_counter,
                                };
                                chan.send(progress).unwrap();
                            },
                            Err(err) => return Err(DownloadError::DownloadCrash(id,err)),
                        }
                    };
                    
                    chan.send(DownloadEvent::Finished{id}).unwrap();
                    Ok(())
                },
                Err(err) => Err(DownloadError::FileNotCreated(path.to_string_lossy().to_string(), err))
            }
        } else {
            Err(DownloadError::InternalError(String::from("La respuesta de la solicitud {id} no se guardó en el hashmap de caché")))
        }
    }

}
