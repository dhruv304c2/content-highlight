use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Clone)]
pub struct ContentRequest{
    pub title : String,
    pub video_id : String,
    pub max_duration_sec : i64,
    pub aud_file: String,
    pub link : String,
    pub transcipt_name: String,
    pub highlights: Vec<Highlight>,
    pub lable : String,
    pub cancelled : bool
}

#[derive(Serialize,Deserialize,Clone)]
pub struct Highlight {
    pub title : String,
    pub description : String,
    pub startStamp : String,
    pub endStamp : String
}

impl ContentRequest {
    pub fn new(title: String, video_id: String, max_duration_sec: i64) -> ContentRequest {
        ContentRequest{
            title : title.clone(),
            video_id: video_id.clone(),
            max_duration_sec,
            aud_file: "".to_string(),
            highlights: vec![],
            link: format!("{}{}", "https://www.youtube.com/watch?v=", video_id).to_string(),
            transcipt_name: format!("{} [{}].txt", title, video_id).to_string(),
            lable: format!("{} [{}]", title, video_id).to_string(),
            cancelled : false
        }
    }

    pub fn cancel(&mut self){
        self.cancelled = true
    }
}
