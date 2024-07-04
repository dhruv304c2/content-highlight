use std::{env, io, process::Command};
use crate::structs::download_request::DownloadRequest;


pub struct VidDownloadService{

}

impl VidDownloadService{
    pub fn download(download_request: DownloadRequest) -> io::Result<()> {
        let current_dir = env::current_dir()?;
        let download_dir_path = format!("{}/downloads", current_dir.to_str().unwrap()).to_string();

        let _make_dir_output = Command::new("mkdir")
            .arg("-p")
            .arg(download_dir_path.clone())
            .output()
            .expect("failed to create downloads directory");
 
        let download_output = Command::new("yt-dlp")
            .arg(download_request.video_id.clone())
            .current_dir(&download_dir_path.clone())
            .output()
            .expect("failed to invoke download command, please ensure you, have yt-dlp installed");

        if download_output.status.success() {
            //Do nothing
        }
        else{
            println!("yt-dlp operation failed returned: {}" , String::from_utf8_lossy(&download_output.stdout));
        }

        Ok(())
    }
}
