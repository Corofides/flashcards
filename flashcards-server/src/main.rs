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
use async_std::task;

use sqlx::{
    {Sqlite, Pool},
    sqlite::SqlitePoolOptions,
    migrate::MigrateDatabase,
};
//use sqlx::{Row, SqlitePool, Sqlite};

const DB_URL: &str = "sqlite://flashcards.db";

struct AppState {
    database: Mutex<Database>,
}

#[derive(Debug, Default)]
pub struct Database {
    pool: Option<Pool<Sqlite>>,
}

impl Database {

    pub fn delete_card(&self, card_id: u32) {
        task::block_on(async {
            if let Some(pool) = self.pool.clone() {
                let result = sqlx::query("DELETE FROM flashcards WHERE id = ?")
                    .bind(card_id)
                    .execute(&pool)
                    .await;

                println!("Result {:?}", result);
            }
        });
    }

    pub fn add_card(&self, card: &Card) {
        task::block_on(async {
            if let Some(pool) = self.pool.clone() {
                let result = sqlx::query("INSERT INTO flashcards (id, front_of_card, back_of_card) VALUES (?, ?, ?)")
                    .bind(card.get_id())
                    .bind(card.get_front())
                    .bind(card.get_back())
                    .execute(&pool)
                    .await;

                println!("Result {:?}", result);
            }
        });
    }

    pub fn get_cards(&self) -> Vec<Card> {
        task::block_on(async {
            if let Some(pool) = self.pool.clone() {

                let cards = sqlx::query_as::<_, Card>(
                        "SELECT id, front_of_card as front, back_of_card as back FROM flashcards"
                    )
                    .fetch_all(&pool).await.unwrap();

                return cards;

            }

            vec![]
        })
    }

    async fn migrate_db(&mut self) {
        if let Some(pool) = self.pool.clone() {

            let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

            println!("Crate Dir {}", crate_dir);
            let migrations = std::path::Path::new(&crate_dir).join("migrations");

            println!("Migrations Dir: {:?}", migrations);

            let migration_result = sqlx::migrate::Migrator::new(migrations)
                .await
                .unwrap()
                .run(&pool)
                .await;
               
            match migration_result {
                Ok(_) => println!("Migration success!"),
                Err(error) => panic!("Migration Error: {}", error),
            }

        }

    }

    pub fn new() -> Self {
        task::block_on(async {
            if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
                println!("Creating DB {}", DB_URL);
                match Sqlite::create_database(DB_URL).await {
                    Ok(_) => println!("Created DB"),
                    Err(error) => panic!("Error: {}", error),
                }
            } else {
                println!("DB already exists");
            }

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(DB_URL).await;


            if let Ok(pool) = pool {

                let mut new_db = Self {
                    pool: Some(pool)
                };

                Self::migrate_db(&mut new_db).await;

                new_db

                

            } else {
                panic!("could not create db");
            }
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let database = Database::new();
        
    let cards = database.get_cards();

    println!("{cards:?}");
    
    let shared_state = Arc::new(AppState {
        database: Mutex::new(database),
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

    let database = state.database.lock().unwrap();
    let cards = &mut database.get_cards();
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

async fn get_cards(State(state): State<Arc<AppState>>) -> Json<Value> {
    //let cards = state.cards.lock().unwrap();
    let database = state.database.lock().unwrap();
    let cards = database.get_cards();

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
