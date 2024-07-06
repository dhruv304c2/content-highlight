use std::{i64, io::{self, Write}};
use crate::{helpers::iso_8601_helper, structs::{download_request::ContentRequest, search_response::{SearchItem, SearchResponse, VideoItem}}};

const API_KEY : &str = "AIzaSyBjUaKPrI2FjVD1o9oK6f05O_1M7aRKlUs";
const BASE_URL : &str = "https://www.googleapis.com/youtube"; 

pub struct ContentFetchService{

}

pub struct SearchRequest{
    pub query : String,
    pub min_time_lmt : i64,
    pub max_time_lmt: i64,
    pub max_video_count: u32,
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

        println!("\nTotal Search Results: {}", items.len());
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
            let video_duration = iso_8601_helper::iso8601_duration_to_seconds(&video_details.contentDetails.duration)
                .expect("failed to convert video duration to seconds");

            if video_duration < request.min_time_lmt {
                continue;
            }

            let trim;
            if video_duration < request.max_time_lmt { trim = video_duration; }
            else { trim = video_duration; }

            println!("{}) {}",count +1,item.snippet.title);
            let dn_request = ContentRequest{
                title: item.snippet.title.clone(),
                video_id: item.id.videoId.clone().expect("does not have a video Id"),
                max_duration_sec: trim,
                aud_file: "".to_string(),
                transcript_file: "".to_string(),
            };

            vec.push(dn_request);
            count += 1;
        }

        return Ok(vec);
    }

    pub fn get_valid_user_input<T>(prompt: String, default_val : T) -> T 
    where T : std::str::FromStr {
        let mut parsed = false;
        let mut inp = String::new();
        let mut paresed_val = default_val;

        while !parsed {
            print!("{}", prompt);
            _ = io::stdout().flush();
            let _ = io::stdin().read_line(&mut inp);
            inp = inp.trim().to_string();

            let parse_result = inp.parse::<T>();

            match parse_result {
                Ok(val) => {
                    paresed_val = val;
                    break;
                }
                Err(_) => {
                    parsed = false;
                }
            }
        }
        return paresed_val;
    }

    pub fn get_request_from_user() -> SearchRequest {
        let query = Self::get_valid_user_input("Search for: ".to_string(), "".to_string());
        let max_duration =  Self::get_valid_user_input("Minimum video length allowed (s) : ".to_string(), i64::MAX);
        let trim_to = Self::get_valid_user_input("Maximum video length allowed (s) : ".to_string(), max_duration);
        let max_video_count = Self::get_valid_user_input("Maximum result limit: ".to_string(), u32::MAX);

        return SearchRequest{
            query,
            min_time_lmt: max_duration,
            max_video_count,
            max_time_lmt : trim_to.into()
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
