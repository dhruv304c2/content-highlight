use std::{env, io::{self}};
use regex::Regex;
use crate::{helpers::iso_8601_helper, structs::{download_request::ContentRequest, search_response::{SearchItem, SearchResponse, VideoItem}}};
use super::input_service::{InputService, Validation};

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
        println!("Select a method");
        println!("1) Create highlights using search");
        println!("2) Create highlights using youtube video link");
        let selection = InputService::get_input("Enter selection number: ".to_string(), 1);
        println!("");
        if selection == 1 {
            Self::create_request_from_querry().await
        }
        else{
            Self::create_request_from_link().await
        }
    } 

    pub async fn create_request_from_querry() -> io::Result<Vec<ContentRequest>> {
        let client = reqwest::Client::new();

        let fetch_api_url = format!("{}/v3/search", BASE_URL).to_string();

        let request = Self::get_request_from_user();
        let params = [("part", "snippet"),("q", &request.query.to_string()), 
            ("key", &env::var("YT_API_KEY").expect("could not get youtube api key"))];

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
            if video_duration < request.max_time_lmt { trim = video_duration - 20; }
            else { trim = request.max_time_lmt; }

            println!("{}) {}",count +1,item.snippet.title);
            let dn_request = ContentRequest::new(
                item.snippet.title.clone(),
                item.id.videoId.clone().expect("does not have a video Id"),
                trim);

            vec.push(dn_request);
            count += 1;
        }

        return Ok(vec);
    }

    pub async fn create_request_from_link() -> io::Result<Vec<ContentRequest>> {
        let mut requests = vec![];
        let link = InputService::get_string("Enter watch link to the youtube video: ".to_string(), "".to_string(), Validation::YtLink);
        let max_vid_len = InputService::get_input("Trim to length: ".to_string(), 10);
        let vid_id = Self::extract_video_id(&link);
        match vid_id {
            Some(id) => {
                let vid_details = Self::get_video_details(id).await;
                let req = ContentRequest::new(vid_details.snippet.title, vid_details.id, max_vid_len);
                requests.push(req);
                return  Ok(requests);
            }
            None =>{
                panic!("could not extract video link from id");
            }
        } 
    }

    pub fn get_request_from_user() -> SearchRequest {
        let query = InputService::get_input("Search for: ".to_string(), "".to_string());
        let max_duration =  InputService::get_input("Minimum video length allowed (s) : ".to_string(), i64::MAX);
        let trim_to = InputService::get_input("Maximum video length allowed (s) : ".to_string(), max_duration);
        let max_video_count = InputService::get_input("Maximum result limit: ".to_string(), u32::MAX);

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
        let params = [("part", "snippet,contentDetails"),
            ("key", &env::var("YT_API_KEY").expect("failed to get youtube API key")),
            ("id", &video_id)];

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

    fn extract_video_id(link: &str) -> Option<String> {
     let re = Regex::new(r"(?:(?:https?://)?(?:www\.)?(?:youtube\.com/watch\?v=|youtu\.be/|youtube\.com/embed/|youtube\.com/v/|youtube\.com/e/|youtube\.com/.*[?&]v=))([a-zA-Z0-9_-]{11})").unwrap();
        if let Some(captures) = re.captures(link) {
            if let Some(video_id) = captures.get(1) {
                return Some(video_id.as_str().to_string());
            }
        }
        None
    }
}
