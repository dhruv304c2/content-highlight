use std::io;
use crate::structs::search_response::SearchResponse;

pub struct ContentFetchService{

}

pub struct SearchRequest{
    pub query : String,
    pub max_duration : f32,
}

impl ContentFetchService {

    pub async fn start_fetch() -> io::Result<Vec<String>> {
        let client = reqwest::Client::new();

        let fetch_api_url = "https://www.googleapis.com/youtube/v3/search".to_string();
        let api_key = "AIzaSyBjUaKPrI2FjVD1o9oK6f05O_1M7aRKlUs".to_string();

        let request = Self::get_request_from_user();

        let params = [("part", "snippet"),("q", &request.query.to_string()), ("key", &api_key)];

        let fetch_video_response = client.get(fetch_api_url)
            .query(&params)
            .send().await
            .expect("failed to send")
            .text()
            .await
            .expect("failed to get text");

        println!("Fetched data: {}", fetch_video_response);

        let mut vec: Vec<String> = Vec::new();

        let result = serde_json::from_str::<SearchResponse>(&fetch_video_response).expect("failed to parse search response");
        let items = result.items; 

        for item in &items{
            vec.push(item.id.videoId.to_string());
        }

        return Ok(vec);
    }

    pub fn get_request_from_user() -> SearchRequest {
        let mut query = String::new();
        let mut duration_input = String::new();
        let mut max_duration = f32::MAX;
       
        println!("Enter search querry:");
        let _ = io::stdin().read_line(&mut query);
        query = query.trim().to_string();

        let mut parsed = false;
        while !parsed {
            println!("Enter max duration:");
            let _ = io::stdin().read_line(&mut duration_input);
            duration_input = duration_input.trim().to_string();

            let parse_result = duration_input.parse::<f32>();

            match parse_result {
                Ok(val) => {
                    max_duration = val;
                    parsed = true;
                }
                Err(_) => {
                    parsed = false;
                }
            }
        }

        return SearchRequest{
            query,
            max_duration
        }
    }
}
