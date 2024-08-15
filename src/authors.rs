use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tera::{Context, Tera};

#[derive(Deserialize, Serialize)]
struct Author {
    authors: Vec<String>,
}

pub async fn get_authors() -> Json<Value> {
    let response = reqwest::get("https://poetrydb.org/author")
        .await
        .expect("Failed to fetch authors");

    let author: Author = response.json().await.expect("Failed to get author");

    let author_data: Value = json!({
        "author": author,
    });

    Json(author_data)
}


pub async fn get_rendered_authors(Extension(tera): Extension<Arc<Tera>> ) -> Result<impl IntoResponse, (StatusCode, String)> {

    let response = reqwest::get("https://poetrydb.org/author")
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let authors: Vec<Author> = response.json()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    
    let author = &authors[0];

    let mut context = Context::new();
    context.insert("author", author);

    let rendered = tera.render("authors.html", &context)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Html(rendered))
}
