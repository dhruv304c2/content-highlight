use std::process::Command;


pub struct VidDownloadService{

}

impl VidDownloadService{
    pub fn download (url: String) {
        let output = Command::new("yt-dlp")
            .arg(url.clone())
            .output()
            .expect("failed to invoke download command, please ensure you, have yt-dlp installed");
        if output.status.success() {
            println!("yt-dlp operation wase successful: returned {}" , String::from_utf8_lossy(&output.stdout));
        }
        else{
            println!("yt-dlp operation failed returned: {}" , String::from_utf8_lossy(&output.stdout));
        }

        let makingdirs = Command::new("mkdir")
            .arg("-p")
            .arg("Downloads")
            .output()
            .expect("Failed to create downloads directory");
        
        let changingdirs = Command::new("mv")
            .arg("*.webm")
            .arg("Downloads/")
            .output()
            .expect("Files failed to move");
        if changingdirs.status.success() {
            print!("Moved to Downloads folder")
        }
    }
}
