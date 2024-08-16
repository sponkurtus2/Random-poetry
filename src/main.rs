mod authors;
mod poem;

use std::fs;
use std::sync::Arc;
use crate::authors::get_authors;
use crate::poem::{get_all_poems, get_one_random_poem};
use axum::{routing::get, Router, Extension};
use axum::response::{IntoResponse, Html};
use lazy_static::lazy_static;
use tera::{Context, Tera};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let tera = Arc::new(TEMPLATES.clone());

    let app = Router::new()
        .route("/", get(root))
        .route("/about", get(about_page))
        .route("/poems", get(get_all_poems))
        .route("/random", get(get_one_random_poem))
        .route("/authors", get(get_authors))
        .layer(Extension(tera));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


// Route for the Home page
async fn root(Extension(tera): Extension<Arc<Tera>>) -> Html<String> {
    let mut context = Context::new();
    let rendered = tera.render("index.html", &context).expect("Failed to render template");
    Html(rendered)
}

// Route for the about page
async fn about_page() -> impl IntoResponse {
    match fs::read_to_string("../About.html") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error loading about page: {}", e))
    }
}