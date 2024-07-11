use std::{env, error::Error, fs::{File, OpenOptions}, io::{self, Read, Write}, path::PathBuf, process::Command};
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

    pub async fn transcribe(content_request: &mut ContentRequest) -> Result<&mut ContentRequest, Box<dyn Error>> {

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
                    let transcript_name = content_request.transcipt_name.clone();
                    let title = content_request.title.clone();
                    let result = Self::write_transcription_to_file(transcript_name,
                        title,
                        transcription);
                    match result {
                        Ok(_) => {
                            return Ok(content_request);
                        },
                        Err(e) => {
                            return Err(Box::new(e));
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
        Err(Box::new(err))
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

    pub fn read_transcript(content_request: ContentRequest) -> io::Result<String> {
        let transcriptions_path = FileManagerService::get_transcription_path();
        let full_path = transcriptions_path?.join(content_request.transcipt_name);

        let mut file = OpenOptions::new()
            .read(true)
            .open(&full_path)?;

        let mut string = String::new();
        file.read_to_string(&mut string)?;

        return Ok(string);
    }
}
