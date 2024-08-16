mod poem;

use crate::poem::{get_all_poems, get_one_random_poem};
use axum::response::{Html};
use axum::{routing::get, Extension, Router};
use lazy_static::lazy_static;
use std::sync::Arc;
use tera::{Context, Tera};

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
        .route("/about", get(about))
        .route("/poems", get(get_all_poems))
        .route("/random", get(get_one_random_poem))
        .layer(Extension(tera));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}


// Route for the Home page
async fn root(Extension(tera): Extension<Arc<Tera>>) -> Html<String> {
    let context: Context = Context::new();
    let rendered: String = tera
        .render("index.html", &context)
        .expect("Failed to render index.html");
    Html(rendered)
}

// Route for the rendered About page
async fn about(Extension(tera): Extension<Arc<Tera>>) -> Html<String> {
    let context: Context = Context::new();
    let rendered: String = tera
        .render("about.html", &context)
        .expect("Failed to render about.html");
    Html(rendered)

}

// Route for the about page
// This is the old route, and this way you can render a html page, but just the page (Not with tera)
// async fn about_page() -> impl IntoResponse {
    // match fs::read_to_string("templates/about.html") {
        // Ok(content) => Html(content),
        // Err(e) => Html(format!("Error loading about page: {}", e)),
    // }
// }


