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

mod content_pipeline{
    pub mod steps;
}

use content_pipeline::steps::{AudioDownloadStep, 
    DownloadHighlightsStep, 
    LLMAnalysisStep, 
    Step, 
    TranscriptionStep};
use services::{
    content_fetch_service::ContentFetchService,
    file_manager_service::FileManagerService
};

use std::io;
use structs::download_request::ContentRequest;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    FileManagerService::create_cache_dirs().expect("Failed to create required directories");

    println!("");
    let mut download_requests = ContentFetchService::start_fetch().await?;
    let mut requestes: Vec<&mut ContentRequest> = vec![];

    for req in download_requests.iter_mut(){
        requestes.push(req)
    }

    let mut steps : Vec<Box<dyn Step>> = vec![
        Box::new(AudioDownloadStep),
        Box::new(TranscriptionStep),
        Box::new(LLMAnalysisStep),
        Box::new(DownloadHighlightsStep)
    ];

    for step in steps.iter_mut(){
        println!("\nExecuting step: {}", step.name());
        let mut success = 0;
        for req in requestes.iter_mut(){
            if req.cancelled {
                continue;
            }
            match step.execute(req).await {
                Ok(_) => success += 1,
                Err(msg) => {
                    println!("-> Failed: {}", msg);
                    println!(" Cancelling request for {} [{}]", req.title, req.video_id);
                    req.cancel()
                },
            }
        }
        println!("Step completed.....({} success)", success.to_string());
    }

    println!("\nDone!");
    Ok(())
}
