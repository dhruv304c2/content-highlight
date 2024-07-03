use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchResponse{
    pub items : Vec<SearchItem>
}

#[derive(Serialize, Deserialize)]
pub struct SearchItem {
    pub id : Id
}

#[derive(Serialize, Deserialize)]
pub struct Id{
    pub kind : String,
    pub videoId : String
}
