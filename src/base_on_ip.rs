
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
        println!("Error: {}", response.status());
        return None;
    }
    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
   //check if the response has a city field
    if let Some(_) = json.get("city") {
        return Some(json.to_string());
    }
    return None;
}