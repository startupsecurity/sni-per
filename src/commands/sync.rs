use crate::utils;
use std::path::Path;

pub async fn run() {
    println!("Syncing...");
    utils::create_tmp_dir().expect("Failed to create tmp directory");

    let dir = utils::get_sni_per_dir().expect("Failed to get sni-per directory");

    for provider in utils::PROVIDERS {
        let url = format!("https://kaeferjaeger.gay/sni-ip-ranges/{}/ipv4_merged_sni.txt", provider);
        let file_path_str = format!("{}/{}_ipv4_merged_sni.txt", dir.display(), provider);
        let file_path = Path::new(&file_path_str);
        utils::download_file(&url, file_path).await.expect("Error downloading file");
    }
}
