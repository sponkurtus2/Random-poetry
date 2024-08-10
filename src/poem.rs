use std::sync::Arc;
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tera::{Context, Tera};

#[derive(Deserialize, Serialize)]
struct Poem {
    title: String,
    author: String,
    lines: Vec<String>,
}

pub async fn get_all_poems() -> Json<Value> {
    let response = reqwest::get("https://poetrydb.org/author,title/Shakespeare;Sonnet")
        .await
        .expect("Failed to fetch api");

    let poems: Vec<Poem> = response.json().await.expect("Failed to get poems");

    let json_data = json!({
        "poems": poems,
    });

    Json(json_data)
}

pub async fn get_one_random_poem(
    Extension(tera): Extension<Arc<Tera>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let response = reqwest::get("https://poetrydb.org/random")
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let poems: Vec<Poem> = response.json().await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let poem = &poems[0]; // Solo un poema aleatorio

    let mut context = Context::new();
    context.insert("poem", poem);

    let rendered = tera.render("randomPoem.html", &context)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Html(rendered))
}