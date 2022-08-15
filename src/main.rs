use axum::{
    extract::Form,
    routing::{get, post},
    http::StatusCode,
    response::{Html, IntoResponse},
    Router,
};
use serde::{Deserialize};
use std::net::SocketAddr;

#[derive(Deserialize,Debug)]
struct Input {
    text: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/haiku", get(index))
        .route("/haiku", post(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<String> {
    Html(include_str!("./template.html").to_string())
}

async fn handler(Form(input): Form<Input>) -> String {
    let ok = input.text.as_str().split('\n').collect::<Vec<&str>>();

    match ok.iter().map(|&x| x.split_ascii_whitespace().count()).collect::<Vec<usize>>() {
        x if x == vec![5,7,5] =>
            format!("Your haiku is valid ({},{},{})", 5, 7, 5),
        x =>
            format!("Your haiku is invalid ({},{},{})", x[0], x[1], x[2]),
    }
}
