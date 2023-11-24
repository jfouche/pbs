use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{delete, get, post, Router},
    Json,
};
pub use pbs_core::*;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tracing::{info, warn};

pub enum Error {
    StoreError,
    StateError,
}

// type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::StoreError => (StatusCode::INTERNAL_SERVER_ERROR, "STORE ERROR"),
            Error::StateError => (StatusCode::INTERNAL_SERVER_ERROR, "STATE ERROR"),
        }
        .into_response()
    }
}

#[derive(Clone)]
struct AppState {
    store: Arc<Store>,
}

pub async fn serve(port: u16) -> std::result::Result<(), pbs_core::Error> {
    info!("serve({port})");

    let store_state = AppState {
        store: Arc::new(Store::open("store.db3")?),
    };

    let app = Router::new()
        .route("/item/make", post(item_make))
        .route("/item/buy", post(item_buy))
        .route("/item/:id", get(get_item))
        .route("/item/:id/children", get(get_item_children))
        .route("/item/:id_parent/child", post(add_child))
        .route("/item/:id_parent/child/:id_child", delete(delete_child))
        .route("/list", get(list))
        .route("/search", get(search))
        .fallback(fallback)
        .with_state(store_state);

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    warn!("fallback({uri})");
    (StatusCode::NOT_FOUND, String::new())
}

/// `/list`
async fn list(State(state): State<AppState>) -> impl IntoResponse {
    info!("list()");
    state
        .store
        .items()
        .map_err(|_e| Error::StoreError)
        .map(Json)
}

#[derive(Deserialize)]
struct Pattern {
    pattern: String,
}

#[derive(Serialize)]
struct SearchResult(Vec<Item>);

/// `/search`
async fn search(State(state): State<AppState>, Query(query): Query<Pattern>) -> impl IntoResponse {
    info!("search({})", query.pattern);
    state
        .store
        .search_items(&query.pattern)
        .map_err(|_e| Error::StoreError)
        .map(Json)
}

/// `/item/:id`
async fn get_item(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    info!("get_item({id})");
    state
        .store
        .item(id)
        .map_err(|_e| Error::StoreError)
        .map(Json)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemMake {
    pub name: String,
}

/// `/item/make`
async fn item_make(
    State(state): State<AppState>,
    Json(new_item): Json<ItemMake>,
) -> impl IntoResponse {
    info!("item_make({new_item:?})");
    state
        .store
        .make_item(&new_item.name)
        .map_err(|_e| Error::StoreError)
        .map(Json)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemBuy {
    pub pn: String,
    pub name: String,
}

/// `/item/buy`
async fn item_buy(
    State(state): State<AppState>,
    Json(new_item): Json<ItemBuy>,
) -> impl IntoResponse {
    info!("item_buy({new_item:?})");
    state
        .store
        .buy_item(&new_item.pn, &new_item.name)
        .map_err(|_e| Error::StoreError)
        .map(Json)
}

/// `/item/:id/children`
async fn get_item_children(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    info!("get_item_children({id})");
    state
        .store
        .children(id)
        .map_err(|_e| Error::StoreError)
        .map(Json)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddChild {
    pub id_child: i64,
    pub quantity: usize,
}

/// `/item/:id_parent/child`
async fn add_child(
    State(state): State<AppState>,
    Path(id_parent): Path<i64>,
    Json(child): Json<AddChild>,
) -> impl IntoResponse {
    info!("add_child({id_parent}, {child:?}");
    state
        .store
        .add_child(id_parent, child.id_child, child.quantity)
        .map_err(|_e| Error::StoreError)
}

/// `/item/:id_parent/child/:id_child`
async fn delete_child(
    State(state): State<AppState>,
    Path((id_parent, id_child)): Path<(i64, i64)>,
) -> impl IntoResponse {
    info!("delete_child({id_parent}, {id_parent})");
    state
        .store
        .remove_child(id_parent, id_child)
        .map_err(|_e| Error::StoreError)
}
