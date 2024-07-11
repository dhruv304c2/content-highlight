use core::str;
use std::{env, error::Error, io::{self, Write}};
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, Client};
use crate::{helpers::llm_prompts, structs::{download_request::ContentRequest, llm::{APIResponse, HighlightResponse, Message, Request}}};
use super::transcription_service::TranscriptionService;

pub struct LLMService{

}

const OPEN_AI_BASE_URL : &str = "https://api.openai.com"; 

impl LLMService {
    pub async fn get_highlights_from_transcription(content_request : &mut ContentRequest) -> Result<&mut ContentRequest, Box<dyn Error>> {
        println!("  -> Analysing transcript for: {}", content_request.title);

        let transcript = TranscriptionService::read_transcript(content_request.clone())?;

        let instruction_message = Message::new(
                "system".to_string(),
                llm_prompts::HIGHLIGHT_GENERATION_INSTRUCTIONS.to_string()
            );

        let transcript_message = Message::new(
            "system".to_string(),
            transcript.to_string()
        );

        let request_body = Request::new(
            "gpt-4o".to_string(),
            vec![
                instruction_message,
                transcript_message
            ],
            1000
        );

        // println!("Sending llm request: \n{}", 
        //     serde_json::to_string(&request_body)
        //     .expect("failed to serialize request"));

        let client = Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let auth_token = format!("Bearer {}", env::var("OPENAI_API_KEY")?);

        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_token)?);

        let url = format!("{}/v1/chat/completions", OPEN_AI_BASE_URL);
        let response = client.post(url)
            .headers(headers)
            .json(&request_body)
            .send().await
            .expect("failed to send highlight request")
            .text()
            .await
            .expect("failed to convert response to text");

        // println!("llm response: \n{}", response);
        
        let ser_response = serde_json::from_str::<APIResponse>(&response)?;
        match  ser_response.choices.first(){
            Some(response) => {
                // println!("LLM response: {}", response.message.content);
                let mut highlight_response = serde_json::from_str::<HighlightResponse>(&response.message.content)
                    .expect("failed to de serialize json");
                content_request.highlights.append(&mut highlight_response.highlights);
                return Ok(content_request);
            }
            None => {
                Err("LLM returned empty choice array".into())
            }
        }
    }
}
