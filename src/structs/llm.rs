use serde::{Deserialize, Serialize};
use super::download_request::HighLight;

#[derive(Serialize,Deserialize,Clone)]
pub struct Message{
    pub role : String,
    pub content : String,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct Request{
    pub model : String,
    pub messages : Vec<Message>,
    pub max_tokens: i64
}

#[derive(Serialize,Deserialize,Clone)]
pub struct APIResponse {
    pub choices : Vec<Response>
}

#[derive(Serialize,Deserialize,Clone)]
pub struct  Response {
    pub index : i64,
    pub message : Message,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct HighlightResponse{
    pub highlights : Vec<HighLight>
}

impl Message{
    pub fn new(role: String, content: String) ->  Message{
        Message{
            role,
            content
        }
    }
}

impl Request{
    pub fn new(model : String, messages : Vec<Message>, max_tokens: i64) -> Request{
        Request{
            model,
            messages,
            max_tokens
        }
    }
}
