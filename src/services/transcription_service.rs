use std::{env, fs::File, io::{self, Error, Write}, process::Command, task::Wake};
use crate::structs::download_request::ContentRequest;

pub const INSTALL_SCRIPT_URL: &str = "https://raw.githubusercontent.com/AssemblyAI/assemblyai-cli/main/install.sh";
pub const INSTALL_SCRIPT_PATH: &str = "/tmp/install_assemblyai.sh";
pub const ASSEMBLY_API_KEY: &str = "c3d28c7ba9f1412db5a4297b2fb3b3a1"; 

pub struct TranscriptionService {}

impl TranscriptionService {
    pub fn ensure_dependencies() -> bool {
        
        let download_output = Command::new("/bin/bash")
            .arg("-fsSL")
            .arg(INSTALL_SCRIPT_URL)
            .output();

        match download_output {
            Ok(output) => {
                if output.status.success() {
                    // Save the script to a file
                    let mut file = File::create(INSTALL_SCRIPT_PATH).expect("Failed to create install script file");
                    file.write_all(&output.stdout).expect("Failed to write to install script file");


                    // Step 2: Execute the install script with sudo
                    let install_command = Command::new("/bin/bash")
                        .arg(INSTALL_SCRIPT_PATH)
                        .output();

                    match install_command {
                        Ok(output) => {
                            if output.status.success() {
                                println!("{}", String::from_utf8_lossy(&output.stdout));
                                // Check if `assemblyai` is now available
                                let check_command = Command::new("assemblyai")
                                    .arg("config")
                                    .arg(ASSEMBLY_API_KEY)
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

    pub fn transcribe(content_request: ContentRequest) -> io::Result<()> {
        
        let current_dir = env::current_dir()?;
        let download_dir_path = format!("{}/downloads", current_dir.to_str().unwrap()).to_string();

        let found = Self::ensure_dependencies();
        if !found {
            eprintln!("Aborting transcription: could not find dependencies");
        }

        println!("Transcribing: {}", content_request.title);
        let transcription_output = Command::new("assemblyai")
            .arg("transcribe")
            .arg(format!("\"{}\"", content_request.file_path))
            .current_dir(download_dir_path)
            .output();

        match transcription_output {
            Ok(output) => {
                if output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    eprintln!("Transcription command failed with output: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                eprintln!("Failed to run transcribe command: {}", e);
            }
        }

        Ok(())
    }
}
