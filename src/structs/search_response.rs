use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchResponse<T> where T : Clone{
    pub items : Vec<T>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchItem {
    pub id : Id,
    pub snippet : Snippet
}

#[derive(Serialize, Deserialize,Clone)]
pub struct VideoItem {
    pub id : String,
    pub snippet : Snippet,
    pub contentDetails : ContentDetails
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Id{
    pub kind : String,
    pub videoId : Option<String>,
    pub channelId : Option<String>
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Snippet{
    pub title : String 
}

#[derive(Serialize, Deserialize,Clone)]
pub struct ContentDetails{
    pub duration : String
} 
