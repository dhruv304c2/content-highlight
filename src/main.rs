mod services{
    pub mod vid_download_service;
    pub mod content_fetch_service;
}

mod structs{
    pub mod search_response;
}

use std::io;
use services::{content_fetch_service::ContentFetchService, vid_download_service::VidDownloadService};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Enter a video url to download");
    let fetched_urls = ContentFetchService::start_fetch().await?;
    for url in &fetched_urls {
        let _ = VidDownloadService::download(url.to_string());
    }
    println!("Done!");
    Ok(())
}
