use std::{env,fs::{self, remove_dir_all}, io::Result, path::PathBuf};

pub const RELATIVE_DOWNLOAD_DIR_PATH : &str = "cache/downloads"; 
pub const RELATIVE_TRANSCRIPTION_DIR_PATH : &str = "cache/transcriptions";
pub const RELATIVE_HIGHLIGHTS_DIR_PATH : &str = "highlights";

pub struct FileManagerService{

}

impl FileManagerService {

    pub fn create_dir(dir_path: String) -> Result<PathBuf> {
        let current_dir = env::current_dir()?;
        let dir_path = current_dir.join(dir_path);

        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }

        Ok(dir_path)
    }

    pub fn create_highlight_dir(dir_name: String) -> Result<PathBuf>{
        Self::create_dir(format!("{}/{}", RELATIVE_HIGHLIGHTS_DIR_PATH.to_string(), dir_name))
    }

    fn create_download_dir() -> Result<PathBuf> {
        Self::create_dir(RELATIVE_DOWNLOAD_DIR_PATH.to_string())
    }

    fn create_transcription_dir() -> Result<PathBuf> {
        Self::create_dir(RELATIVE_TRANSCRIPTION_DIR_PATH.to_string())
    }

    fn create_highlights_dir() -> Result<PathBuf> {
        Self::create_dir(RELATIVE_HIGHLIGHTS_DIR_PATH.to_string())
    }

    fn clear_cache() -> Result<()>{
        remove_dir_all("cache")
    }

    fn remove_directory(path: &str) -> Result<()> {
        fs::remove_dir_all(path)?;
        Ok(())
    }

    pub fn create_cache_dirs() -> Result<()> {
        Self::clear_cache()?;

        Self::create_download_dir()?;
        Self::create_transcription_dir()?;
        Self::create_highlights_dir()?;

        Ok(())
    }

    pub fn get_downloads_path() -> Result<PathBuf> {
        Self::get_path(RELATIVE_DOWNLOAD_DIR_PATH.to_string())
    }

    pub fn get_transcription_path() -> Result<PathBuf> {
        Self::get_path(RELATIVE_TRANSCRIPTION_DIR_PATH.to_string())
    }

    pub fn get_highlights_path() -> Result<PathBuf> {
        Self::get_path(RELATIVE_HIGHLIGHTS_DIR_PATH.to_string())
    }

    fn get_path(relative_path : String) -> Result<PathBuf> {
        let current_dir = env::current_dir()?;
        let dir_path = current_dir.join(relative_path);
        return Ok(dir_path);
    }
}
