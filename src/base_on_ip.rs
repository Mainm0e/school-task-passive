use reqwest::{self, Url};
use std::str::Bytes;

#[derive(Debug, serde::Deserialize)]
struct IpapiResult {
    org: Option<String>,
    city: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
    error: Option<String>,
}

pub async fn get_location(ip: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url: String = format!("https://ipapi.co/{}/json", ip);
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            eprintln!("Rate limit exceeded. Try again later.");
            return Err("IPAPI IP query failed with rate limit exceeded".into());
        }
        return Err(format!("IPAPI IP query failed with status code: {}", response.status()).into());
    }

    // Extract the response body as bytes without consuming the entire response
    let response_body_bytes: Bytes = response.bytes().await?;
    let raw_response = String::from_utf8_lossy(&response_body_bytes);
    println!("Raw response: {}", raw_response);

    // Now use the original response for further processing
    let result: IpapiResult = response.json().await.expect("Failed to parse JSON");
    println!("Parsed response: {:?}", result);

    
    if let Some(error) = result.error {
        return handle_error(ip).await;
    } else {
        let isp = result.org.unwrap_or_default();
        let city = result.city.unwrap_or_default();
        let lat = result.latitude.unwrap_or_default();
        let long = result.longitude.unwrap_or_default();
        
        let result_str = format!("ISP: {}\nCity Lat/Long: {} ({}) / ({})\n", isp, city, lat, long);
        println!("{}", result_str);
        Ok(result_str)
    }
}

async fn handle_error(ip: &str) -> Result<String, Box<dyn std::error::Error>> {
    let whois_process = tokio::process::Command::new("whois")
        .arg(ip)
        .output()
        .await?;
    
    if !whois_process.status.success() {
        eprintln!("Whois command failed: {:?}", whois_process.stderr);
        return Ok(String::new());
    }
    
    let whois_str = String::from_utf8_lossy(&whois_process.stdout);
    
    let isp = get_regex_group(&whois_str, r"OrgName:\s+(.*)");
    let city = get_regex_group(&whois_str, r"City:\s(.*)");
    let address = get_regex_group(&whois_str, r"Address:\s+(.*)");
    let (lat, long) = get_coordinates(&address).await;
    
    println!("lat: {}", lat);
    println!("long: {}", long);
    
    let result_str = format!("ISP: {}\nCity Lat/Long: {} ({}) / ({})\n", isp, city, lat, long);
    println!("{}", result_str);
    
    Ok(result_str)
}

async fn get_coordinates(location: &str) -> (String, String) {
    let base_url = "https://nominatim.openstreetmap.org/search";

    let mut url = Url::parse(base_url).expect("Failed to parse base URL");
    url.query_pairs_mut()
        .append_pair("q", location)
        .append_pair("format", "json")
        .append_pair("limit", "1");

    let response = reqwest::get(url).await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let data: Vec<serde_json::Value> = response.json().await.expect("Failed to parse JSON");
                if let Some(result) = data.get(0) {
                    if let (Some(lat), Some(lon)) = (result.get("lat").and_then(|v| v.as_str()), result.get("lon").and_then(|v| v.as_str())) {
                        return (lat.to_string(), lon.to_string());
                    }
                }
                eprintln!("No coordinates found for the location: {}", location);
            } else {
                eprintln!("Request failed with status code: {}", response.status());
            }
        }
        Err(err) => {
            eprintln!("Error fetching coordinates: {:?}", err);
        }
    }

    (String::new(), String::new())
}

fn get_regex_group(input: &str, pattern: &str) -> String {
    if let Some(captures) = regex::Regex::new(pattern).unwrap().captures(input) {
        captures.get(1).map_or("", |m| m.as_str()).to_string()
    } else {
        String::new()
    }
}
