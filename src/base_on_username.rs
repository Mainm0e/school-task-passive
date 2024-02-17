use scraper::{Html, Selector};

pub async fn check_username(username: &str) -> Option<String>  {
    let results = vec![
        ("github", check_github(username).await),
        //("pornhub", check_pornhub(username).await),
        ("twitter", check_twitter(username).await),
        ("youtube", check_youtube(username).await),
        ("tiktok", check_tiktok(username).await),
        ("instagram", check_instagram(username).await),
        ("linkedin", check_linkedin(username).await),
        ("onlyfan", check_onlyfan(username).await)
    ];
    let mut result_string = String::new();

    for (platform, is_valid) in results {
        match is_valid {
            Some(true) => result_string.push_str(&format!("{}: yes\n", platform)),
            Some(false) => result_string.push_str(&format!("{}: no\n", platform)),
            None => result_string.push_str(&format!("Error checking {}: unknown\n", platform)),
        }
    }

    Some(result_string)
}


async fn check_github(username: &str) -> Option<bool> {
    let url = format!("https://github.com/{}", username);
    // Create custom headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(
        "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Mobile Safari/537.36"));

   // Make a GET request to the URL with custom headers
   let response = match reqwest::Client::builder().default_headers(headers).build().unwrap().get(&url).send().await {
    Ok(response) => response,
    Err(error) => {
        eprintln!("Error making request: {}", error);
        return None;
    }
    };

     // Check if the response status is a success (2xx)
     if response.status().is_success() {
        Some(true)
    } else {
        Some(false)
    }
}

async fn check_pornhub(username: &str) -> Option<bool> {
    let url = format!("https://pornhub.com/users/{}", username);
    let response = reqwest::get(&url).await.ok()?;
    println!("{:?}", response.status());
    if response.status().is_success() {
        Some(true);
    } else {
        Some(false);
    }
    Some(false)
}

// Implement similar functions for other platforms...

async fn check_twitter(username: &str) -> Option<bool> {
    // Your Twitter checking logic here
    // Example: Dummy logic, replace it with actual implementation
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Simulate asynchronous task
    Some(true)
}

async fn check_youtube(username: &str) -> Option<bool> {
    // Your YouTube checking logic here
    // Example: Dummy logic, replace it with actual implementation
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Simulate asynchronous task
    Some(true)
}

async fn check_tiktok(username: &str) -> Option<bool> {
    let url = format!("https://www.tiktok.com/@{}", username);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(
        "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Mobile Safari/537.36"
    ));

   // Make a GET request to the URL with custom headers
   let response = match reqwest::Client::builder().default_headers(headers).build().unwrap().get(&url).send().await {
    Ok(response) => response,
    Err(error) => {
        eprintln!("Error making request: {}", error);
        return None;
    }
};
    if response.status().is_success() {
        if response.url().as_str() == url {
            Some(true)
        } else {
            Some(false)
        }
    } else {
        Some(false)
    }
}

async fn check_instagram(username: &str) -> Option<bool> {
    let url = format!("https://www.picuki.com/profile/{}", username);
    // Create custom headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));

   // Make a GET request to the URL with custom headers
   let response = match reqwest::Client::builder().default_headers(headers).build().unwrap().get(&url).send().await {
    Ok(response) => response,
    Err(error) => {
        eprintln!("Error making request: {}", error);
        return None;
    }
};
    // Check if the response status is a success (2xx)
    if response.status().is_success() {
        let body = response.text().await.ok()?;
        let document = Html::parse_document(&body);
        // Use a CSS selector to find the specific <h1> element with the class "profile-name-top"
        let selector = Selector::parse("h1.profile-name-top").unwrap();
        if let Some(element) = document.select(&selector).next() {
            // Check if the text content of the <h1> element contains the username
            let check_word = format!("@{}", username);
            if element.text().any(|text| text.contains(&check_word)) {
                println!("Found username: {}", username);
                // The desired <h1> element is present with the correct username
                return Some(true);
            }
        }
        Some(false)
    } else {
        Some(false)
    }
}

async fn check_linkedin(username: &str) -> Option<bool> {
    // Your LinkedIn checking logic here
    // Example: Dummy logic, replace it with actual implementation
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Simulate asynchronous task
    Some(true)
}

async fn check_onlyfan(username: &str) -> Option<bool> {
    // Your OnlyFans checking logic here
    // Example: Dummy logic, replace it with actual implementation
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Simulate asynchronous task
    Some(true)
}