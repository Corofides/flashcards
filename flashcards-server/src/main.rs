use flashcards_data::{CreateCardPayload, Card};

use tower_http::cors::{CorsLayer};
use http::header::{HeaderValue};
use http::Method;
use axum::{
    extract::State,
    routing::{
        get,
        post,
    },
    Router,
    response::Json,
};

use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

use sqlx::{
    {Row, SqlitePool, Sqlite},
    sqlite::SqlitePoolOptions,
    migrate::MigrateDatabase,
};
//use sqlx::{Row, SqlitePool, Sqlite};

const DB_URL: &str = "sqlite://flashcards.db";

struct AppState {
    cards: Mutex<Vec<Card>>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating DB {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Created DB"),
            Err(error) => panic!("Error: {}", error),
        }
    } else {
        println!("DB already exists");
    }

    //let pool = SqlitePoolOptions::new()
    //    .max_connections(5)
    //    .connect("sqlite://flashcard_db.db").await?;
    //
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let row = sqlx::query(
            "SELECT 150 as value"
        )
        //.bind(150_i64)
        .fetch_one(&db).await.unwrap();

    let value = row.get::<i64, &str>("value");
    assert!(150_i64 == value, "Could not retrieve data from database!");

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
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any/*["priority"]*/); // I'd rather not do the ANY thing.

    let app = Router::new()
        .route("/health", get(get_health))
        .route("/cards", get(get_cards))
        .route("/cards", post(add_card))
        .with_state(shared_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())

}

async fn add_card(State(state): State<Arc<AppState>>, Json(payload): Json<CreateCardPayload>) -> Json<Value> {

    let cards = &mut state.cards.lock().unwrap();
    let cards_total = cards.len();

    let new_card = Card::new(
        cards_total,
        payload.front.clone(),
        payload.back.clone(),
    ); 

    cards.push(Card::new(
        cards_total,
        payload.front.clone(),
        payload.back.clone(),
    ));

    Json(json!(
        new_card
    ))

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
