use std::{error::Error, future::Future, pin::Pin};
use crate::{
    services::{
        llm_service::LLMService, 
        transcription_service::TranscriptionService, 
        vid_download_service::VidDownloadService
    },
    structs::download_request::ContentRequest
};

pub trait  Step {
    fn name(&self) -> String;
    fn execute<'a>(&self, content_request : &'a mut ContentRequest) -> 
        Pin<Box<dyn Future<Output=Result<&'a mut ContentRequest, Box<dyn Error>>> + Send + 'a>>;
} 

pub struct AudioDownloadStep;

impl Step for AudioDownloadStep{
    fn name(&self) -> String{
        "Download Audio".to_string()
    }

    fn execute<'a>(&self, content_request : &'a mut ContentRequest) -> 
        Pin<Box<dyn Future<Output=Result<&'a mut ContentRequest, Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            let result = VidDownloadService::download_audio(content_request).await?;
            Ok(result)
        })
    }
}

pub struct TranscriptionStep;

impl Step for TranscriptionStep{
    fn name(&self) -> String {
        "Transribing Audio".to_string()
    }

    fn execute<'a>(&self, content_request : &'a mut ContentRequest) -> 
        Pin<Box<dyn Future<Output=Result<&'a mut ContentRequest, Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            let result = TranscriptionService::transcribe(content_request).await?;
            Ok(result)
        })
    }
}

pub struct LLMAnalysisStep;

impl Step for LLMAnalysisStep{
    fn name(&self) -> String {
        "Analysing using LLM".to_string()
    }

    fn execute<'a>(&self, content_request : &'a mut ContentRequest) -> 
        Pin<Box<dyn Future<Output=Result<&'a mut ContentRequest, Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            let result = LLMService::generate_highlights(content_request).await?;
            Ok(result)
        })
    }
}

pub struct DownloadHighlightsStep;

impl Step for DownloadHighlightsStep{
    fn name(&self) -> String {
        "Downloading Generated Highlights".to_string()
    }

    fn execute<'a>(&self, content_request : &'a mut ContentRequest) -> 
        Pin<Box<dyn Future<Output=Result<&'a mut ContentRequest, Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            let result = VidDownloadService::download_highlights(content_request).await?;
            Ok(result)
        })
    }
}
