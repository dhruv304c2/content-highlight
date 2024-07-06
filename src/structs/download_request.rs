#[derive(Clone)]
pub struct ContentRequest{
    pub title : String,
    pub video_id : String,
    pub max_duration_sec : i64,
    pub aud_file: String,
    pub transcript_file: String
}

impl ContentRequest {
    pub fn link(self: Self) -> String {
        format!("{}{}", "https://www.youtube.com/watch?v=", self.video_id).to_string()
    }
}
