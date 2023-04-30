mod model;
mod spider;

use crate::spider::core::SearchCore;
use axum::{extract::Query, response::Json, routing::get, Router};
use serde::Deserialize;
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(search));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

async fn search(Query(params): Query<SearchQuery>) -> Json<Value> {
    let song_name = params.q;
    let core = SearchCore::new();
    let result = tokio::task::block_in_place(|| core.search(&song_name));
    Json(json!({
        "code": 0,
        "msg": "success",
        "data": result,
    }))
}
