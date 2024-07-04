mod services{
    pub mod vid_download_service;
    pub mod content_fetch_service;
    pub mod transcription_service;
}

mod structs{
    pub mod search_response;
    pub mod download_request;
}

mod helpers{
    pub mod iso_8601_helper; 
}

use std::io;
use services::{content_fetch_service::ContentFetchService, transcription_service::TranscriptionService, vid_download_service::VidDownloadService};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Enter a video url to download");
    let download_requests = ContentFetchService::start_fetch().await?;

    // println!("Transcribing videos....");
    // for download_request in &download_requests {
    //     let _= TranscriptionService::transcribe(download_request.clone());
    // }

    println!("Downloading videos....");
    for download_request in &download_requests {
        let _ = VidDownloadService::download(download_request.clone());
    }

    println!("Done!");
    Ok(())
}
