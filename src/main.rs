mod services{
    pub mod vid_download_service;
}

use services::vid_download_service::VidDownloadService;

fn main() {
    println!("Hello, world!");
    let url = "none".to_string();
    VidDownloadService::download(url);
}
