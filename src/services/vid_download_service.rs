use std::{error::Error, fs::OpenOptions, io::{self, Write}, path::PathBuf, process::{Command, Output}};
use regex::Regex;
use crate::{helpers::iso_8601_helper::seconds_to_time_format, structs::download_request::{ContentRequest}};
use super::file_manager_service::FileManagerService;

const AUDIO_FMT : &str = "mp3";

pub struct VidDownloadService{

}

impl VidDownloadService{

    pub async fn download_audio(download_request: &mut ContentRequest) -> Result<&mut ContentRequest, Box<dyn Error>> {

        let trim_to = seconds_to_time_format(download_request.max_duration_sec);
 
        let download_output = Self::download_in_range(download_request.video_id.clone(), 
            "00:00:00".to_string(),
            trim_to,
            true,
            None)?;

        if download_output.status.success() {

            let output = String::from_utf8_lossy(&download_output.stdout);
            let mut file_name = Self::extract_file_names(&output).expect("failed to extract file name");
            file_name = Self::replace_extension(&file_name, AUDIO_FMT);

            println!("  -> Downloaded file: {}", file_name);

            download_request.aud_file = file_name;

            Ok(download_request)
        }
        else{
            let err = io::Error::new(io::ErrorKind::Other, format!("    -> Download failed for {}", download_request.title).to_string());
            Err(Box::new(err))
        }
    }

    pub async fn download_highlights(content_request: &mut ContentRequest) -> Result<&mut ContentRequest, Box<dyn Error>>{
        println!("  -> Downloading highlights for : {}", content_request.title);
        for highlight in content_request.highlights.iter() {
            let relative_path = format!("{}/{}",content_request.lable,highlight.title); 
            let download_path = FileManagerService::create_highlight_dir(relative_path)?;

            print!("        -> Downloading highlight : {} [{}]-[{}]....", highlight.title, highlight.startStamp, highlight.endStamp);
            _ = io::stdout().flush();
            let output = Self::download_in_range(content_request.video_id.clone(), 
                highlight.startStamp.clone(),
                highlight.endStamp.clone(),
                false, 
                Some(download_path.clone()));


            match output {
                Ok(out) => {
                    if out.status.success() {
                        println!("Done ✔");

                        //saving highlight data
                        let mut data = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .create(true)
                            .open(download_path.join(format!("{}.json", highlight.title)))?;

                        writeln!(data, "{}" ,serde_json::to_string(highlight).expect("failed to write highlight as json"))?;
                    }
                    else{
                        println!("Failed ✗");
                    }
                }
                Err(_) => {
                    println!("Failed ✗");
                },
            }
        }

        Ok(content_request)
    }

    pub fn download_in_range(vid_id: String,
        start_stamp: String, 
        end_stamp: String,
        audio_only : bool,
        at_path: Option<PathBuf>) -> io::Result<Output> {
        let download_dir_path;
        
        match at_path {
            Some(p) => download_dir_path = p,
            None => download_dir_path = FileManagerService::get_downloads_path()?,
        }

        let section_regex = format!("*{}-{}", start_stamp, end_stamp);

        let mut cmd = Command::new("yt-dlp");

        if audio_only {
            cmd.arg("-x")
                .arg("--audio-format")
                .arg(AUDIO_FMT); // Convert to mp3 for consistent audio format
        }

        let download_output = cmd
            .arg("--download-sections")
            .arg(&section_regex)
            .arg(&vid_id)
            .current_dir(download_dir_path)
            .output()?;

        if download_output.status.success() {
            // let out = String::from_utf8_lossy(&download_output.stdout);
            // println!("{}",out);
            Ok(download_output)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to download video section",
            ))
        }
    }

    fn extract_file_names(output: &str) ->  Result<String, String> {
        let regex_patterns = [
            r#"\[download\] (.*) has already been downloaded"#,
            r#"\[download\] Destination: (.*)"#,
            r#"\[ExtractAudio\] Destination: (.*)"#,
        ];

        for &pattern in &regex_patterns {
            let re = Regex::new(pattern).map_err(|e| e.to_string())?;
            for line in output.lines() {
                if let Some(captures) = re.captures(line) {
                    if let Some(file_name) = captures.get(1) {
                        return Ok(file_name.as_str().to_string());
                    }
                }
            }
        }

        Err("Failed to extract file name".to_string())
    }

    fn replace_extension(file_name: &str, new_extension: &str) -> String {
        if let Some(dot_index) = file_name.rfind('.') {
            let (name, _) = file_name.split_at(dot_index);
            format!("{}.{}", name, new_extension)
        } else {
            format!("{}.{}", file_name, new_extension)
        }
    }
}
