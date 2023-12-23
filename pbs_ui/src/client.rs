use pbs_srv::{AddChild, Children, Item, ItemBuy, ItemMake, Report};

pub static BASE_API_URL: &str = "http://localhost:3030";

/// GET /search?pattern={pattern}
pub async fn search_items(pattern: &str) -> Result<Vec<Item>, reqwest::Error> {
    let pattern = urlencoding::encode(pattern);
    let url = format!("{BASE_API_URL}/search?pattern={pattern}");
    let items = reqwest::get(&url).await?.json::<Vec<Item>>().await?;
    Ok(items)
}

/// `POST /item/make { name: string }`
pub async fn item_make(name: &str) -> Result<Item, reqwest::Error> {
    let new_item = ItemMake {
        name: name.to_string(),
    };
    let client = reqwest::Client::new();
    let item = client
        .post(new_item.url(BASE_API_URL))
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

/// `GET /item/:id`
pub async fn item(id: i64) -> Result<Item, reqwest::Error> {
    let url = format!("{BASE_API_URL}/item/{id}");
    let item = reqwest::get(&url).await?.json::<Item>().await?;
    Ok(item)
}

/// `GET /item/:id/children`
pub async fn children(id: i64) -> Result<Children, reqwest::Error> {
    let url = format!("{BASE_API_URL}/item/{id}/children");
    let items = reqwest::get(&url).await?.json::<Children>().await?;
    Ok(items)
}

/// `DELETE /item/:id_parent/child/:id_child`
pub async fn delete_child(id_parent: i64, id_child: i64) -> Result<bool, reqwest::Error> {
    let url = format!("{BASE_API_URL}/item/{id_parent}/child/{id_child}");
    reqwest::Client::builder()
        .build()?
        .delete(&url)
        .send()
        .await?;
    Ok(true)
}

/// `POST /item/:id_parent/child { id: i64, quantity: usize }`
pub async fn add_child(
    id_parent: i64,
    id_child: i64,
    quantity: usize,
) -> Result<(), reqwest::Error> {
    let url = format!("{BASE_API_URL}/item/{id_parent}/child");
    let add_child = AddChild { id_child, quantity };
    let client = reqwest::Client::new();
    let _ = client
        .post(&url)
        .json(&add_child)
        .send()
        .await?
        .bytes()
        .await?;
    Ok(())
}

/// `POST /item/:id_parent/release`
pub async fn release_item(id: i64) -> Result<Item, reqwest::Error> {
    let url = format!("{BASE_API_URL}/item/{id}/release");
    let client = reqwest::Client::new();
    let item = client.post(&url).send().await?.json::<Item>().await?;
    Ok(item)
}

/// `GET /item/:id/report`
pub async fn item_report(id: i64) -> Result<Report, reqwest::Error> {
    let url = format!("{BASE_API_URL}/item/{id}/report");
    let client = reqwest::Client::new();
    let report = client.get(&url).send().await?.json::<Report>().await?;
    Ok(report)
}
