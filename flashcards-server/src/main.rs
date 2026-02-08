use flashcards_data::Card;

use tower_http::cors::{CorsLayer};
use http::header::HeaderValue;
use http::Method;
use axum::{
    extract::State,
    routing::get,
    Router,
    response::Json,
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

struct AppState {
    cards: Mutex<Vec<Card>>,
}

#[tokio::main]
async fn main() {

    let cards = vec![
        Card::new(
            0, 
            String::from("Simple, slip on shoes with very thin soles and no heel"), 
            String::from("Ballet Flats")
        ),
        Card::new(
            1,
            String::from("The quintessential heeled shoe. They are closed toe and usually have a seamless, low cut front. They don't have straps or laces - you just slide your foot in."),
            String::from("Pumps (or Court Shoes)"),
        ),
        Card::new(
            2,
            String::from("A more structured, masculine inspired slip-on shoe. They often have a slightly thicker sole than a ballet flat, and a distinct tongue that covers more of the top of the foot"),
            String::from("Loafers"),
        ),
    ];

    let shared_state = Arc::new(AppState {
        cards: Mutex::new(cards),
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/health", get(get_health))
        .route("/cards", get(get_cards))
        .with_state(shared_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn get_cards(State(state): State<Arc<AppState>>) -> Json<Value> {
    let cards = state.cards.lock().unwrap();

    Json(json!(
        *cards
    ))
}

async fn get_health() -> String {
    String::from("200 OK")
    /* Json(json!(
        {"state": String::from("running")}
    )) */
}
