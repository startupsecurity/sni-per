use std::fs;
use std::path::Path;
use std::io::Write;
use futures::StreamExt;
use std::path::PathBuf;

pub static PROVIDERS: [&str; 5] = ["amazon", "digitalocean", "google", "microsoft", "oracle"];

pub fn get_sni_per_dir() -> std::io::Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found"))?;
    let sni_per_dir = home_dir.join(".sni-per");
    Ok(sni_per_dir)
}

pub fn create_tmp_dir() -> std::io::Result<()> {
    let sni_per_dir = get_sni_per_dir()?;
    fs::create_dir_all(&sni_per_dir)?;
    Ok(())
}

pub async fn download_file(url: &str, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting download from URL: {}", url);
    let response = reqwest::get(url).await?;
    
    let total_size = response.content_length().unwrap_or_else(|| {
        println!("Content length not provided.");
        0
    }); // Get content length before consuming response

    let mut stream = response.bytes_stream(); // Now consume response
    
    let mut file = fs::File::create(file_path)?;
    println!("Creating file: {}", file_path.display());

    
    let mut downloaded: u64 = 0;
    let mut last_output = String::new();
    
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        
        if total_size > 0 {
            let percentage = (downloaded as f64 / total_size as f64) * 100.0;
            let output = format!("Downloaded: {:.2}%", percentage);
            print!("\r{}", output);
            last_output = output;
        } else {
            let output = format!("Downloaded {} bytes so far", downloaded);
            print!("\r{}", output);
            last_output = output;
        }
    }
    
    println!("\r{}", last_output); 
    Ok(())
}