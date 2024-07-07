mod services{
    pub mod vid_download_service;
    pub mod content_fetch_service;
    pub mod transcription_service;
    pub mod file_manager_service;
    pub mod llm_service;
}

mod structs{
    pub mod search_response;
    pub mod download_request;
    pub mod llm;
}

mod helpers{
    pub mod iso_8601_helper; 
    pub mod llm_prompts;
}

use services::{
    content_fetch_service::ContentFetchService,
    file_manager_service::FileManagerService,
    llm_service::LLMService,
    transcription_service::TranscriptionService,
    vid_download_service::VidDownloadService
};

use std::io;
use structs::download_request::ContentRequest;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let mut transcription_req : Vec<&mut ContentRequest> = Vec::new();
    let mut llm_highlight_req: Vec<&mut ContentRequest> = Vec::new();
    let mut highlight_download_req: Vec<&mut ContentRequest> = Vec::new();

    FileManagerService::create_cache_dirs();

    println!("");
    let mut download_requests = ContentFetchService::start_fetch().await?;

    let mut download_count = 0;
    println!("\nDownloading videos....");
    for download_request in download_requests.iter_mut() {
        match  VidDownloadService::download_audio(download_request) {
            Ok(req) => {
                transcription_req.push(req);
                download_count += 1;
            },
            Err(msg) => {
                eprintln!("{}", msg); 
            }
        }
    }
    println!("Download finished....({})", download_count);

    let mut transcription_count = 0;
    println!("\nTranscribing videos....");
    for transcription_req in transcription_req.iter_mut(){
         match  TranscriptionService::transcribe(transcription_req) {
            Ok(req) => {
                llm_highlight_req.push(req);
                transcription_count += 1;
            }
            Err(msg) => {
                eprintln!("{}", msg);
            }
        }
    }
    println!("Transcription finished....({})", transcription_count);

    let mut highlights_count = 0;
    println!("\nUsing LLM for highlights....");
    for req in llm_highlight_req.iter_mut() {
        println!("-> Generating Highlights for {}", req.lable);
        match LLMService::get_highlights_from_transcription(req).await {
            Ok(response) => {
                highlights_count += 1;
                highlight_download_req.push(response)
            }
            Err(msg) => {
                eprintln!("{}", msg);
            }
        }
    };
    println!("Highlight generation finished....({})", highlights_count);

    println!("\nDownloading highlights....");
    for req in highlight_download_req.iter() {
        println!("-> Downloading {} Highlights for video: {}", req.highlights.len(), req.lable);
        _ = VidDownloadService::download_highlights(req);
    }

    println!("\nDone!");
    Ok(())
}
