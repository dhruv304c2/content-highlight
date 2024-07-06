use std::{env, fs::{File, OpenOptions}, io::{self, Write}, path::PathBuf, process::Command};
use crate::structs::download_request::ContentRequest;
use super::file_manager_service::FileManagerService;

pub const INSTALL_SCRIPT_URL: &str = "https://raw.githubusercontent.com/AssemblyAI/assemblyai-cli/main/install.sh";
pub const INSTALL_SCRIPT_PATH: &str = "/tmp/install_assemblyai.sh";

pub struct TranscriptionService {

}

impl TranscriptionService {
    pub fn ensure_dependencies() -> bool {

        let download_output = Command::new("curl")
            .arg("-fsSL")
            .arg(INSTALL_SCRIPT_URL)
            .output();

        match download_output {
            Ok(output) => {
                if output.status.success() {
                    let mut file = File::create(INSTALL_SCRIPT_PATH).expect("Failed to create install script file");
                    file.write_all(&output.stdout).expect("Failed to write to install script file");

                    let install_command = Command::new("/bin/bash")
                        .arg(INSTALL_SCRIPT_PATH)
                        .output();

                    match install_command {
                        Ok(output) => {
                            if output.status.success() {
                                println!("{}", String::from_utf8_lossy(&output.stdout));
                                let check_command = Command::new("assemblyai")
                                    .arg("config")
                                    .arg(env::var("ASSEMBLYAI_API_KEY")
                                        .expect("failed to get assembly ai API key"))
                                    .output();
                                if let Ok(check_output) = check_command {
                                    return check_output.status.success();
                                }
                            } else {
                                eprintln!("Install command failed with output: {}", String::from_utf8_lossy(&output.stderr));
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to execute install command: {}", e);
                        }
                    }
                } else {
                    eprintln!("Download command failed with output: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                eprintln!("Failed to download install script: {}", e);
            }
        }

        false
    }

    pub fn transcribe(content_request: ContentRequest) -> io::Result<ContentRequest> {

        let download_dir_path = FileManagerService::get_downloads_path().expect("failed to get download path");

        println!("-> Transcribing: \"{}\"", content_request.aud_file);
        let command = format!("assemblyai transcribe \"{}\" -l", content_request.aud_file);
        let transcription_output = Command::new("zsh")
            .arg("-c")
            .arg(&command)
            .current_dir(&download_dir_path)
            .output();

        let err_msg;
        match transcription_output {
            Ok(output) => {
                if output.status.success() {
                    let transcription = String::from_utf8_lossy(&output.stdout).to_string();
                    let transcirpt_file_name = format!("{} [{}].txt", content_request.title, content_request.video_id);
                    let result = Self::write_transcription_to_file(transcirpt_file_name.clone(),
                        content_request.title.to_string(),
                        transcription);
                    match result {
                        Ok(_) => {
                            let new_req = ContentRequest{
                                title: content_request.title,
                                video_id: content_request.video_id,
                                max_duration_sec: content_request.max_duration_sec,
                                aud_file: content_request.aud_file,
                                transcript_file: transcirpt_file_name
                            };
                            return Ok(new_req);
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else {
                    err_msg = format!("Transcription command failed with output: {}", String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(e) => {
                err_msg = format!("Failed to run transcribe command: {}", e);
            }
        }

        let err = io::Error::new(io::ErrorKind::Other, err_msg);
        Err(err)
    }

    fn write_transcription_to_file(file_name: String, title: String, transcription: String) -> io::Result<PathBuf> {
        let transcriptions_path = FileManagerService::get_transcription_path();
        let full_path = transcriptions_path?.join(file_name);

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&full_path)?;

        writeln!(file, "Title: {}", title)?;
        writeln!(file, "{}", transcription)?;

        Ok(full_path)
    }
}
