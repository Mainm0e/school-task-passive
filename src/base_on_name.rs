use scraper::{ElementRef, Html, Selector};

pub async fn get_people(name_str: &str) -> Option<String> {
    let result = radaris_search(name_str).await;
    result
}
#[allow(dead_code)]
async fn radaris_search(name_str: &str) -> Option<String> {
    let (first, last) = name_str.split_once(" ")?;

    let url = format!("https://radaris.com/p/{}/{}", first, last);
    let response = reqwest::get(&url).await.ok()?;
    let body = response.text().await.ok()?;
    let document = Html::parse_document(&body);
    let profiles_list_selector = Selector::parse(".profiles-list").unwrap();
    let cards_selector = Selector::parse(".card.teaser-card").unwrap();

    
    if let Some(profiles_list) = document.select(&profiles_list_selector).next() {
        let mut results = Vec::new();
        
        for card in profiles_list.select(&cards_selector) {
          if let Some(result) = search_elements(card).await {
              results.push(result);
          } 
        }
       return Some(results.join("\n"));
    } else {
        return Some(String::new());
    }
}

async fn search_elements(card:  ElementRef<'_>) -> Option<String> {

    let full_name = card.select(&Selector::parse(".card-title").unwrap()).next()?.text().collect::<String>();
    let address = card.select(&Selector::parse(".res-in .many-links-item").unwrap()).next()?.text().collect::<String>();
    let phone_number = card.select(&Selector::parse(".teaser-card-item .ph").unwrap()).next()?.text().collect::<String>();

    let name_parts: Vec<&str> = full_name.split_whitespace().collect();
    let first_name = name_parts[..name_parts.len() - 1].join(" ");
    let last_name = name_parts.last().copied().unwrap_or("");

    let result = format!("First name: {}\nLast name: {}\nAddress: {}\nNumber: {}\n\n", first_name, last_name, address, phone_number);
   return Some(result);

}