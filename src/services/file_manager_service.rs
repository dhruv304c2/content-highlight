use std::{env, io::Result, path::PathBuf, process::Command};

pub const RELATIVE_DOWNLOAD_DIR_PATH : &str = "cache/downloads"; 
pub const RELATIVE_TRANSCRIPTION_DIR_PATH : &str = "cache/transcriptions";

pub struct FileManagerService{

}

impl FileManagerService {
    fn create_dir(dir_path: String) -> Result<()> {
        let current_dir = env::current_dir()?;
        let dir_path = current_dir.join(dir_path);

        let _make_dir_output = Command::new("mkdir")
            .arg("-p")
            .arg(dir_path.clone())
            .output()
            .expect("failed to create downloads directory");

        Ok(())
    }

    fn create_download_dir() {
        let _ = Self::create_dir(RELATIVE_DOWNLOAD_DIR_PATH.to_string());
    }

    fn create_transcription_dir() {
        let _ = Self::create_dir(RELATIVE_TRANSCRIPTION_DIR_PATH.to_string());
    }

    pub fn create_cache_dirs() {
        Self::create_download_dir();
        Self::create_transcription_dir();
    }

    pub fn get_downloads_path() -> Result<PathBuf> {
        Self::get_path(RELATIVE_DOWNLOAD_DIR_PATH.to_string())
    }

    pub fn get_transcription_path() -> Result<PathBuf> {
        Self::get_path(RELATIVE_TRANSCRIPTION_DIR_PATH.to_string())
    }

    fn get_path(relative_path : String) -> Result<PathBuf> {
        let current_dir = env::current_dir()?;
        let dir_path = current_dir.join(relative_path);
        return Ok(dir_path);
    }
}
