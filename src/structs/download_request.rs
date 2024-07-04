#[derive(Clone)]
pub struct ContentRequest{
    pub title : String,
    pub video_id : String,
    pub max_duration_sec : i64,
    pub file_path: String
}

impl ContentRequest {
    pub fn link(self: Self) -> String {
        format!("{}{}", "https://www.youtube.com/watch?v=", self.video_id).to_string()
    }
}
