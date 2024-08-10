use axum::{Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Deserialize, Serialize)]
struct Author {
    authors: Vec<String>,
}

pub async fn get_authors() -> Json<Value> {

    let response = reqwest::get("https://poetrydb.org/author")
        .await
        .expect("Failed to fetch authors");

    let author: Author = response.json()
        .await
        .expect("Failed to get author");

    let author_data: Value = json!({
        "author": author,
    });

    Json(author_data)

}