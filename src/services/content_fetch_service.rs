use std::{i64, io};
use crate::{helpers::iso_8601_helper, structs::{download_request::ContentRequest, search_response::{SearchItem, SearchResponse, VideoItem}}};

const API_KEY : &str = "AIzaSyBjUaKPrI2FjVD1o9oK6f05O_1M7aRKlUs";
const BASE_URL : &str = "https://www.googleapis.com/youtube"; 

pub struct ContentFetchService{

}

pub struct SearchRequest{
    pub query : String,
    pub max_duration : i64,
    pub max_video_count: u32,
    pub trim_to: i64
}

impl ContentFetchService {

    pub async fn start_fetch() -> io::Result<Vec<ContentRequest>> {
        let client = reqwest::Client::new();

        let fetch_api_url = format!("{}/v3/search", BASE_URL).to_string();

        let request = Self::get_request_from_user();
        let params = [("part", "snippet"),("q", &request.query.to_string()), ("key", API_KEY)];

        let response = client.get(fetch_api_url)
            .query(&params)
            .send().await
            .expect("failed to send")
            .text()
            .await
            .expect("failed to get text");

        let mut vec: Vec<ContentRequest> = Vec::new();

        let result = serde_json::from_str::<SearchResponse<SearchItem>>(&response)
            .expect("failed to parse search response");

        let items = result.items; 

        println!("Total Search Results: {}", items.len());
        println!("Results after applying filters: ");

        let mut count = 0;
        for item in &items{
            if count >= request.max_video_count {
                break;
            }

            if item.id.kind != "youtube#video" {
                continue;
            }

            let video_details = Self::get_video_details(item.id.videoId.clone().expect("does not have a video Id")).await;
            let vidoe_duration = iso_8601_helper::iso8601_duration_to_seconds(&video_details.contentDetails.duration)
                .expect("failed to convert video duration to seconds");

            if vidoe_duration > request.max_duration {
                continue;
            }

            println!("{}) {}",count +1,item.snippet.title);
            let dn_request = ContentRequest{
                title: item.snippet.title.clone(),
                video_id: item.id.videoId.clone().expect("does not have a video Id"),
                max_duration_sec: request.trim_to,
                file_path: "".to_string()
            };

            vec.push(dn_request);
            count += 1;
        }

        return Ok(vec);
    }

    pub fn get_valid_user_input<T>(prompt: String, default_val : T) -> T 
    where T : std::str::FromStr {
        let mut parsed = false;
        let mut duration_input = String::new();
        let mut paresed_val = default_val;

        while !parsed {
            println!("{}", prompt);
            let _ = io::stdin().read_line(&mut duration_input);
            duration_input = duration_input.trim().to_string();

            let parse_result = duration_input.parse::<T>();

            match parse_result {
                Ok(val) => {
                    paresed_val = val;
                    parsed = true;
                }
                Err(_) => {
                    parsed = false;
                }
            }
        }
        return paresed_val;
    }

    pub fn get_request_from_user() -> SearchRequest {
        let mut query = String::new();
        println!("Enter search querry:");
        let _ = io::stdin().read_line(&mut query);
        query = query.trim().to_string();

        let max_duration =  Self::get_valid_user_input("Enter max duration: ".to_string(), i64::MAX);
        let trim_to = Self::get_valid_user_input("Trim results down to seconds, from start: ".to_string(), max_duration);
        let max_video_count = Self::get_valid_user_input("Set max video count: ".to_string(), u32::MAX);

        return SearchRequest{
            query,
            max_duration,
            max_video_count,
            trim_to : trim_to.into()
        }
    }

    pub async fn get_video_details(video_id: String) -> VideoItem {
        let client = reqwest::Client::new();

        let api_url = format!("{}/v3/videos", BASE_URL).to_string();
        let params = [("part", "snippet,contentDetails"),("key", API_KEY),("id", &video_id)];

        let response = client.get(api_url)
            .query(&params)
            .send().await
            .expect("failed to send")
            .text()
            .await
            .expect("failed to get text");

        let result = serde_json::from_str::<SearchResponse<VideoItem>>(&response).expect("failed to parse search response");

        return result.items[0].clone();
    }
}
