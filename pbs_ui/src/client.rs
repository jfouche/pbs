use pbs_srv::{Item, ItemMake};

pub static BASE_API_URL: &str = "http://localhost:3030";

pub async fn search_items(pattern: &str) -> Result<Vec<Item>, reqwest::Error> {
    let url = format!("{BASE_API_URL}/search?pattern={pattern}");
    let items = reqwest::get(&url).await?.json::<Vec<Item>>().await?;
    Ok(items)
}

pub async fn new_item(name: &str) -> Result<Item, reqwest::Error> {
    let url = format!("{BASE_API_URL}/item");
    let new_item = ItemMake {
        name: name.to_string(),
    };
    let client = reqwest::Client::new();
    let item = client
        .post(&url)
        .json(&new_item)
        .send()
        .await?
        .json::<Item>()
        .await?;
    Ok(item)
}
