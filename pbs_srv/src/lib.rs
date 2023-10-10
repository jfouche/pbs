use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post, Router},
    Json,
};
pub use pbs_core::{Item, Store};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};

pub async fn serve(port: u16) {
    println!("pbs_srv::serve({port})");
    let store_state = Arc::new(Store::open("store.db3").unwrap());

    let app = Router::new()
        .with_state(store_state)
        .route("/", get(root))
        // .route("/search", get(search))
        ;

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Product Breakdown Software!"
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct NewItem {
    name: String,
    pn: String,
}

// async fn create_item(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<NewItem>,
// ) -> (StatusCode, Json<User>) {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.name,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

#[derive(Serialize, Deserialize)]
struct ItemChild {
    parent_id: usize,
    child_id: usize,
    quantity: usize,
}

fn add_child(
    State(mut store): State<Arc<Store>>,
    Json(item_child): Json<ItemChild>,
) -> Result<(), String> {
    let store = Arc::get_mut(&mut store).unwrap();
    store
        .add_child_by_id(
            item_child.parent_id,
            item_child.child_id,
            item_child.quantity,
        )
        .map_err(|e| format!("{e:?}"))
}

#[derive(Deserialize)]
struct Pattern {
    pattern: String,
}

#[derive(Serialize)]
struct SearchResult(Vec<Item>);

async fn search(State(store): State<Arc<Store>>, query: Query<Pattern>) -> Json<Vec<Item>> {
    match store.search_items(&query.pattern) {
        Ok(items) => Json(items),
        Err(e) => Json(vec![]),
    }
}
