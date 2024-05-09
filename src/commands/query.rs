use std::path::PathBuf;
use crate::utils;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub async fn run(pattern: &str) {
    let futures = utils::PROVIDERS.iter().map(|provider| async move {
        let dir = utils::get_sni_per_dir().expect("Failed to get sni-per directory");
        let file_path = PathBuf::from(format!(
            "{}/{}_ipv4_merged_sni.txt",
            dir.clone().display(),
            provider
        ));
        let file = File::open(&file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            if line.contains(pattern) {
                println!("{}", line);
            }
        }
        io::Result::Ok(())
    });

    let results = futures::future::join_all(futures).await;
    for result in results {
        if let Err(e) = result {
            eprintln!("Error processing file: {}", e);
        }
    }
}
