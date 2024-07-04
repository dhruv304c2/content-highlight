use std::{clone, env::{self, current_dir}, io, process::{Command, Output}};
use crate::{helpers::iso_8601_helper::seconds_to_time_format, structs::download_request::ContentRequest};

pub struct VidDownloadService{

}

impl VidDownloadService{
    pub fn download(download_request: ContentRequest) -> io::Result<ContentRequest> {
        let current_dir = env::current_dir()?;
        let download_dir_path = format!("{}/downloads", current_dir.to_str().unwrap()).to_string();

        let _make_dir_output = Command::new("mkdir")
            .arg("-p")
            .arg(download_dir_path.clone())
            .output()
            .expect("failed to create downloads directory");

        let trim_to = seconds_to_time_format(download_request.max_duration_sec);
 
        let download_output = Self::download_in_range(download_request.video_id.clone(), "00:00:00".to_string(), trim_to).expect("failed to return output from download command");

        if download_output.status.success() {
            println!("{}", String::from_utf8_lossy(&download_output.stdout));
            let new_req = ContentRequest{
                title : download_request.title,
                video_id: download_request.video_id.clone(),
                max_duration_sec: download_request.max_duration_sec,
                file_path: download_request.file_path
            };

            Ok(new_req)
        }
        else{
            println!("yt-dlp operation failed returned: {}" , String::from_utf8_lossy(&download_output.stderr));
            panic!();
        }
    }

    pub fn download_in_range(vid_id: String, start_stamp: String, end_stamp: String) -> io::Result<Output> {
        let current_dir = env::current_dir()?;
        let download_dir_path = format!("{}/downloads", current_dir.to_str().unwrap()).to_string();

        let section_regex = format!("*{}-{}",start_stamp, end_stamp );
 
        let download_output = Command::new("yt-dlp")
            .arg(vid_id)
            .arg("--download-section")
            .arg(section_regex)
            .current_dir(&download_dir_path.clone())
            .output()
            .expect("failed to invoke download command, please ensure you, have yt-dlp installed");

        Ok(download_output)
    } 
}
