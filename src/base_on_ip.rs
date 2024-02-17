use std::fmt::Result;
use reqwest::{self, Url};
use config::Config;


pub async fn get_location(ip: &str) -> Option<String> {
    let mut settings = Config::default();
    let config_path = "config.toml";
     // Merge with the configuration file
     if let Err(e) = settings.merge(config::File::new(&config_path, config::FileFormat::Toml)) {
        eprintln!("Error loading configuration: {}", e);
        return None;
    }
    let token: String = settings.get("ipinfo_token").expect("Missing 'secret_key' in config");

    let url = format!("https://ipinfo.io/{}?token={}", ip, token); 
    let response = reqwest::get(&url).await.unwrap();
    // check response status
    if !response.status().is_success() {
        return None;
    }
    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    
    return Some(json.to_string());
}