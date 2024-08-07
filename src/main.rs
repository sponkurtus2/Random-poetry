mod authors;

use crate::authors::get_authors;
use actix_web::Responder;
use actix_web::web::route;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Deserialize, Serialize)]
struct Poem {
    title: String,
    author: String,
    lines: Vec<String>,
}

async fn index() -> Json<Value> {
    // Hacer la llamada usando Reqwest
    let response = reqwest::get("https://poetrydb.org/author,title/Shakespeare;Sonnet")
        .await
        .expect("Failed to fetch api");

    let poems: Vec<Poem> = response.json().await.expect("Failed to get poems");

    // Construir una respuesta JSON
    let json_data = json!({
        "poems": poems,
    });

    Json(json_data)

}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/authors", get(get_authors));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
