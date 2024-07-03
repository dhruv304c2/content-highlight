use std::{env, io, process::Command};


pub struct VidDownloadService{

}

impl VidDownloadService{
    pub fn download (url: String) -> io::Result<()>{
        let current_dir = env::current_dir()?;
        let download_dir_path = format!("{}/downloads", current_dir.to_str().unwrap()).to_string();

        println!("downloads path set to {}", download_dir_path.to_string());

        let _make_dir_output = Command::new("mkdir")
            .arg("-p")
            .arg(download_dir_path.clone())
            .output()
            .expect("failed to create downloads directory");
 
        let download_output = Command::new("yt-dlp")
            .arg(url.clone())
            .current_dir(&download_dir_path.clone())
            .output()
            .expect("failed to invoke download command, please ensure you, have yt-dlp installed");

        if download_output.status.success() {
            println!("yt-dlp operation was successful: returned {}" , String::from_utf8_lossy(&download_output.stdout));
        }
        else{
            println!("yt-dlp operation failed returned: {}" , String::from_utf8_lossy(&download_output.stdout));
        }

        Ok(())
    }
}
