use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, Router},
    Json,
};
pub use pbs_core::{Item, Store};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};

enum Error {
    StoreError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "MY ERROR").into_response()
    }
}

#[derive(Clone)]
struct AppState {
    store: Arc<Store>,
}

pub async fn serve(port: u16) -> Result<(), pbs_core::Error> {
    println!("pbs_srv::serve({port})");

    let store_state = AppState {
        store: Arc::new(Store::open("store.db3")?),
    };

    let app = Router::new()
        // .route("/", get(root))
        .route("/item/:id", get(get_item))
        .route("/search", get(search))
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

#[derive(Serialize, Deserialize)]
struct Pattern {
    pattern: String,
}

#[derive(Serialize, Deserialize)]
struct SearchResult(Vec<Item>);

async fn search(State(store): State<AppState>, Query(query): Query<Pattern>) -> impl IntoResponse {
    match store.store.search_items(&query.pattern) {
        Ok(items) => Json(items),
        Err(_e) => Json(vec![]),
    }
}

async fn get_item(
    State(store): State<AppState>,
    Path(id): Path<usize>,
) -> Result<Json<Item>, Error> {
    let store = store.store;
    let item = store.get_item_by_id(id).map_err(|_e| Error::StoreError)?;
    Ok(Json(item))
}
