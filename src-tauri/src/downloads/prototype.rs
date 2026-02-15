use std::path::Path;
use std::io::Result as IoResult;
use futures_lite::stream::StreamExt;
use reqwest::{Client, Response};
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
};
use tauri::ipc::Channel;

use super::{
    Downloader,
    DownloadEvent,
    DownloadError,
};

use crate::util::url::{Url, ReqwestUrl};

pub struct Prototype {
    client: Client,
}

impl  Prototype {
    fn new() ->  Self {
        Prototype {
            client: Client::new(),
        }
    }

    async fn download(&self, id: u64, res: Response, path: &Path, chan: Channel<DownloadEvent>) -> Result<(), DownloadError> {
        let file_path = {
            let mut buffer = path.to_path_buf();
            buffer.push("download");
            buffer.as_path().to_owned()
        };
        
        let file_open = OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_path)
                .await;
        
        match file_open {
            Ok(mut file) => {
                let total_bytes = res.content_length();
                let start = DownloadEvent::Started {
                    id,
                    url: res.url().as_str().to_owned(),
                    total_bytes,
                };
                let mut stream = res.bytes_stream();
                let mut byte_counter = 0;
            
                chan.send(start).unwrap();
                
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(good_chunk) => {
                            let len = good_chunk.len() as u64;
                            if let Err(err) = file.write_all(&good_chunk).await as IoResult<()> {
                                return Err(DownloadError::FileWriteError(path.to_string_lossy().to_string(),err))
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
    }
}

impl  Downloader<ReqwestUrl> for Prototype {
    #[inline]
    fn is_valid_url(_: &ReqwestUrl) -> bool {
        true
    }
    
    async fn start_download(&self, id: u64, url: &ReqwestUrl, path: &Path, chan: Channel<DownloadEvent>) -> Result<(), DownloadError> {
        let response: Result<Response, reqwest::Error> = self.client.get(url.inner())
            .send()
            .await;
        match response {
            Ok(res) => {
                self.download(id, res, path, chan).await
            },
            Err(e) => Err(DownloadError::BadRequest(url.full_string().to_owned(), e))
        }
    }
    
}
