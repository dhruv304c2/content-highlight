use std::{env, error::Error, sync::Arc};
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, Client};
use crate::{helpers::llm_prompts, structs::{download_request::ContentRequest, llm::{APIResponse, HighlightResponse, Message, Request}}};
use crate::structs::download_request::Highlight;
use super::transcription_service::TranscriptionService;
use tokio::task;
use futures::future::join_all;


pub struct LLMService;

const OPEN_AI_BASE_URL : &str = "https://api.openai.com"; 

impl LLMService {

    pub async fn generate_highlights(content_request : &mut ContentRequest) -> Result<&mut ContentRequest, Box<dyn Error>> {
        println!("  -> Analysing transcript for: {}", content_request.title);
        let transcript = TranscriptionService::read_transcript(content_request.clone())?;

        let chunks = Self::create_transcript_chunks(&transcript, 100);

        let mut tasks = vec![];
        for chunk in chunks {
            let chunk = Arc::new(chunk);
            let task = task::spawn(async move {
                Self::process_chunk(chunk).await
            });
            tasks.push(task);
        }

        let results = join_all(tasks).await;

        for result in results {
            let highlights : Vec<Highlight> = result?;
            content_request.highlights.extend(highlights.clone());
        }

        Self::print_highlights(&content_request.highlights);
        Ok(content_request)
    }

    async fn process_chunk(chunk: Arc<String>) -> Vec<Highlight> {
        let highlights = Self::send_highlight_request(&chunk).await.expect("LLM Service: failed to process transcript chunk");
        highlights
    }

    fn create_transcript_chunks(transcript: &str, chunk_size : i64) -> Vec<String> {
        let mut chunks : Vec<String> = vec![];
        let mut lines : Vec<&str> = transcript.split("\n").collect();
        let title = lines[0];
        lines.remove(0);
        let mut active_chunk_size = 0;
        let mut active_chunk = vec![title];

        for line in lines.iter() {
            if chunk_size == active_chunk_size {
                let chunk_str = active_chunk.join("\n"); 
                chunks.push(chunk_str.clone());
                active_chunk_size = 0;
                active_chunk = vec![title];
                // println!("Chunk: ");
                // println!("{}", chunk_str);
                continue;
            }
            active_chunk.push(line); 
            active_chunk_size += 1;
        }

        return chunks;
    }

    async fn send_highlight_request(transcript: &str) -> Result<Vec<Highlight>, Box<dyn Error>> {
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
            .text().await
            .expect("failed to convert response to text");

        // println!("llm response: \n{}", response);
        
        let ser_response = serde_json::from_str::<APIResponse>(&response)?;
        match ser_response.choices.first(){
            Some(response) => {
                let highlight_response = serde_json::from_str::<HighlightResponse>(&response.message.content)
                    .expect("failed to de serialize json");
                Ok(highlight_response.highlights)
            }
            None => {
                Err("LLM returned empty choice array".into())
            }
        }
    }

    fn print_highlights(highlights: &Vec<Highlight>){
        println!("  -> Generated Highlights:");
        let mut count = 1;
        for highlight in highlights {
            println!("  {}) {} [{}]-[{}]", count, highlight.title, highlight.startStamp, highlight.endStamp);
            count += 1;
        }
    }
}
