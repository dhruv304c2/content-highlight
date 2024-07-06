mod services{
    pub mod vid_download_service;
    pub mod content_fetch_service;
    pub mod transcription_service;
    pub mod file_manager_service;
}

mod structs{
    pub mod search_response;
    pub mod download_request;
}

mod helpers{
    pub mod iso_8601_helper; 
}

use std::io;
use services::{content_fetch_service::ContentFetchService, file_manager_service::FileManagerService, transcription_service::TranscriptionService, vid_download_service::VidDownloadService};
use structs::download_request::ContentRequest;

#[tokio::main]
async fn main() -> io::Result<()> {

    let mut transcription_req : Vec<ContentRequest> = Vec::new();
    let mut llm_highlight_req: Vec<ContentRequest> = Vec::new();

    FileManagerService::create_cache_dirs();

    println!("");
    let download_requests = ContentFetchService::start_fetch().await?;

    let mut download_count = 0;
    println!("\nDownloading videos....");
    for download_request in &download_requests {
        match  VidDownloadService::download_audio(download_request.clone()) {
            Ok(req) => {
                transcription_req.push(req);
                download_count += 1;
            },
            Err(msg) => {
                eprintln!("{}", msg); 
            }
        }
    }
    println!("Download complete....({})", download_count);

    // if !TranscriptionService::ensure_dependencies() {
    //     eprintln!("failed to install assembly ai cli, exiting");
    //     return  Ok(());
    // }

    let mut transcription_count = 0;
    println!("\nTranscribing videos....");
    for transcription_req in &transcription_req{
         match  TranscriptionService::transcribe(transcription_req.clone()) {
            Ok(req) => {
                llm_highlight_req.push(req);
                transcription_count += 1;
            }
            Err(msg) => {
                eprintln!("{}", msg);
            }
        }
    }
    println!("Transcription complete....({})", transcription_count);

    println!("\nDone!");
    Ok(())
}
