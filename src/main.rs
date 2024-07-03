mod services{
    pub mod vid_download_service;
}


use std::io;

use services::vid_download_service::VidDownloadService;

fn main() {
    println!("Enter a video url to download");
    let mut url = String::new();
    
    io::stdin()
        .read_line(&mut url)
        .expect("failed to read user input");

    VidDownloadService::download(url.trim().to_string());
}
