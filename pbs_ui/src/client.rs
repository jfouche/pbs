use pbs_srv::{Item, ItemBuy, ItemMake};

pub static BASE_API_URL: &str = "http://localhost:3030";

/// GET /search?pattern=<pattern>
pub async fn search_items(pattern: &str) -> Result<Vec<Item>, reqwest::Error> {
    let url = format!("{BASE_API_URL}/search?pattern={pattern}");
    let items = reqwest::get(&url).await?.json::<Vec<Item>>().await?;
    Ok(items)
}

/// `POST /item/make { name: string }`
pub async fn item_make(name: &str) -> Result<Item, reqwest::Error> {
    let url = format!("{BASE_API_URL}/item/make");
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

/// `POST /item/buy { pn: string, name: string }`
pub async fn item_buy(pn: &str, name: &str) -> Result<Item, reqwest::Error> {
    let url = format!("{BASE_API_URL}/item/buy");
    let new_item = ItemBuy {
        pn: pn.to_string(),
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
