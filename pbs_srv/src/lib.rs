use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, Router},
    Json,
};
pub use pbs_core::{Item, Store};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use tracing::info;

pub enum Error {
    StoreError,
    StateError,
}

type Result<T> = std::result::Result<T, Error>;

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
    store: Arc<RwLock<Store>>,
}

impl AppState {
    fn store(&self) -> Result<RwLockReadGuard<Store>> {
        self.store.read().map_err(|_| Error::StateError)
    }

    fn mut_store(&self) -> Result<RwLockWriteGuard<Store>> {
        self.store.write().map_err(|_| Error::StateError)
    }
}

pub async fn serve(port: u16) -> std::result::Result<(), pbs_core::Error> {
    info!("pbs_srv::serve({port})");

    let store_state = AppState {
        store: Arc::new(RwLock::new(Store::open("store.db3")?)),
    };

    let app = Router::new()
        // .route("/", get(root))
        .route("/item/:id", get(get_item))
        .route("/search", get(search))
        .route("/item", post(new_item))
        .with_state(store_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Deserialize)]
struct Pattern {
    pattern: String,
}

#[derive(Serialize)]
struct SearchResult(Vec<Item>);

async fn search(State(state): State<AppState>, Query(query): Query<Pattern>) -> impl IntoResponse {
    info!("search({})", query.pattern);
    match state.store.read().unwrap().search_items(&query.pattern) {
        Ok(items) => Json(items),
        Err(_e) => Json(vec![]),
    }
}

async fn get_item(State(state): State<AppState>, Path(id): Path<usize>) -> Result<Json<Item>> {
    info!("get_item({id})");
    let item = state.store()?.item(id).map_err(|_e| Error::StoreError)?;
    Ok(Json(item))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewItem {
    pub name: String,
}

async fn new_item(
    State(state): State<AppState>,
    Json(new_item): Json<NewItem>,
) -> Result<Json<Item>> {
    info!("pbs_srv::new_item({new_item:?})");
    let item = state
        .mut_store()?
        .create_item(&new_item.name)
        .map_err(|_e| Error::StoreError)?;
    Ok(Json(item))
}
