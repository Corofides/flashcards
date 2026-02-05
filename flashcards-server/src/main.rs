use flashcards_data::Card;

use tower_http::cors::{CorsLayer};
use http::header::HeaderValue;
use http::Method;
use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/health", get(get_health))
        .route("/cards", get(get_cards))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn get_cards() -> Json<Value> {
    let mut cards = vec![];

    cards.push(Card::new(0, String::from("Ballet Flats"), String::new()));
    cards.push(Card::new(1, String::from("Pumps"), String::new()));
    cards.push(Card::new(2, String::from("Loafers"), String::new()));

    Json(json!(
        cards
    ))
}

async fn get_health() -> String {
    String::from("200 OK")
    /* Json(json!(
        {"state": String::from("running")}
    )) */
}
