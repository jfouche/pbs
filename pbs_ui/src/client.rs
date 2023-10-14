use pbs_srv::{Item, NewItem};

pub static BASE_API_URL: &str = "http://localhost:3030";

pub async fn search_items(pattern: &str) -> Result<Vec<Item>, reqwest::Error> {
    let url = format!("{BASE_API_URL}/search?pattern={pattern}");
    let items = reqwest::get(&url).await?.json::<Vec<Item>>().await?;
    Ok(items)
}

pub async fn new_item(name: &str) -> Result<Item, reqwest::Error> {
    let url = format!("{BASE_API_URL}/item");
    let new_item = NewItem {
        name: name.to_string(),
    };
    let body = serde_json::to_string(&new_item).unwrap();
    let client = reqwest::Client::new();
    let item = client
        .post(&url)
        .body(body)
        .send()
        .await?
        .json::<Item>()
        .await?;
    Ok(item)
}
