use flashcards_data::{CardDifficulty, ReviewCardPayload, CreateCardPayload, Card};

use chrono::{Utc, Days};
use tower_http::cors::{CorsLayer};
use http::header::{HeaderValue};
use http::Method;
use axum::{
    extract::{State, Path},
    routing::{
        get,
        post,
        delete,
        put,
    },
    Router,
    response::Json,
};
/* use axum_macros::{
    debug_handler,
};*/

use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

mod database;
use crate::database::{Database, GetCardFilters};

const DB_URL: &str = "sqlite://flashcards.db";

struct AppState {
    database: Mutex<Database>,
}



#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let database = Database::new();
        
    let shared_state = Arc::new(AppState {
        database: Mutex::new(database),
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::PUT, Method::DELETE, Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any/*["priority"]*/); // I'd rather not do the ANY thing.

    let app = Router::new()
        .route("/health", get(get_health))
        .route("/cards", get(get_cards))
        .route("/cards/due", get(get_cards_due))
        .route("/cards", post(add_card))
        .route("/cards/{card_id}/review", post(review_card))
        .route("/cards/{card_id}", delete(remove_card))
        .route("/cards/{card_id}", put(update_card))
        .with_state(shared_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())

}

async fn remove_card(State(state): State<Arc<AppState>>, Path(card_id): Path<u32>/*, Json(payload): Json<DeleteCardPayload>*/) -> Json<Value> {

    let database = state.database.lock().unwrap();

    println!("CardId: {}", card_id);

    database.remove_card(card_id);

    Json(json!(
        true
    ))

}

async fn update_card(State(state): State<Arc<AppState>>, Path(card_id): Path<u32>, Json(payload): Json<CreateCardPayload>) -> Json<Value> {
    let database = state.database.lock().unwrap();

    let updated_card = Card::new(
        card_id,
        payload.front.clone(),
        payload.back.clone(),
    );

    database.update_card(&updated_card);

    Json(json!(
        updated_card
    ))
}

async fn add_card(State(state): State<Arc<AppState>>, Json(payload): Json<CreateCardPayload>) -> Json<Value> {

    let database = state.database.lock().unwrap();
    let cards = &mut database.get_cards(GetCardFilters::default());
    let cards_total = cards.len();

    let new_card = Card::new(
        cards_total as u32,
        payload.front.clone(),
        payload.back.clone(),
    ); 

    database.add_card(&new_card);

    cards.push(Card::new(
        cards_total as u32,
        payload.front.clone(),
        payload.back.clone(),
    ));

    Json(json!(
        new_card
    ))

}

async fn review_card(
        State(state): State<Arc<AppState>>, 
        Path(card_id): Path<u32>,
        Json(payload): Json<ReviewCardPayload>,
    ) -> Json<Value> {

    let database = state.database.lock().unwrap();
    let card = &mut database.get_card(card_id);
    let difficulty = payload.difficulty.clone();

    if let Some(card) = card {
        match difficulty {
            CardDifficulty::Easy => {
                let mut ease_factor = card.ease_factor().clone();
                card.set_interval(card.interval() * card.ease_factor());

                ease_factor = f32::min(5.0, ease_factor + 0.5);
                card.set_ease_factor(ease_factor);
            },
            CardDifficulty::Medium => {
                let mut ease_factor = card.ease_factor().clone();
                ease_factor = f32::min(5.0, ease_factor + 0.5);
                card.set_ease_factor(ease_factor);
            },
            CardDifficulty::Hard => {
                card.set_interval(1.0);

                let mut ease_factor = card.ease_factor().clone();
                ease_factor = f32::max(1.0, ease_factor - 0.5);
                card.set_ease_factor(ease_factor);
            },
        }

        let mut dt = Utc::now();
        let days_to_add = Days::new(card.interval().clone() as u64);
        dt = dt.checked_add_days(days_to_add).unwrap();

        card.set_next_review(&format!("{dt}"));

        return Json(json!(
            card
        ));

    };

    Json(json!(
        None::<Card>
    ))

}

// Function to serve route /cards/due 
async fn get_cards_due(State(state): State<Arc<AppState>>) -> Json<Value> {
    let database = state.database.lock().unwrap();

    let dt = Utc::now();

    let filters = GetCardFilters::default()
        .add_from(dt);

    println!("{}", dt.timestamp());

    let cards = database.get_cards(filters);

    Json(json!(
        cards
    ))
}

async fn get_cards(State(state): State<Arc<AppState>>) -> Json<Value> {
    //let cards = state.cards.lock().unwrap();
    let database = state.database.lock().unwrap();
    let cards = database.get_cards(GetCardFilters::default());

    Json(json!(
        cards
    ))
}

async fn get_health() -> String {
    String::from("200 OK")
}
