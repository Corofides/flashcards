
use flashcards_data::Card;
use async_std::task;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions,
    Sqlite,
    QueryBuilder,
    Pool,
};
use crate::DB_URL;
use chrono::{DateTime, Utc};

#[derive(Debug, Default)]
pub struct GetCardFilters {
    from: Option<DateTime<Utc>>
}

impl GetCardFilters {
    pub fn add_from(mut self, from: DateTime<Utc>) -> Self {
        self.from = Some(from);
        self
    }
}

#[derive(Debug, Default)]
pub struct Database {
    pool: Option<Pool<Sqlite>>,
}

impl Database {

    pub fn remove_card(&self, card_id: u32) {
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

    pub fn update_card(&self, card: &Card) {
        task::block_on(async {
            if let Some(pool) = self.pool.clone() {
                let result = sqlx::query("UPDATE flashcards SET front_of_card = ?, back_of_card = ? WHERE id = ?")
                    .bind(card.get_front())
                    .bind(card.get_back())
                    .bind(card.get_id())
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

    pub fn get_card(&self, id: u32) -> Option<Card> {
        task::block_on(async {
            if let Some(pool) = self.pool.clone() {
                let card = sqlx::query_as::<_, Card>(
                        "SELECT id, front_of_card as front, back_of_card as back FROM flashcards WHERE id = ?"
                    )
                    .bind(id)
                    .fetch_one(&pool).await.unwrap();

                return Some(card);

            }

            None

        })

    }

    pub fn get_cards(&self, filters: GetCardFilters) -> Vec<Card> {
        task::block_on(async {
            if let Some(pool) = self.pool.clone() {

                let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
                    SELECT id, front_of_card as front, back_of_card as back, next_review
                    FROM flashcards
                    WHERE 1=1 
                ");

                if let Some(from) = filters.from {
                    query_builder.push(" AND next_review < ");
                    query_builder.push_bind(format!("{}", from));
                }

                //println!("{:?}", query_builder);


                let query = query_builder.build_query_as::<Card>();
                let cards = query
                    .fetch_all(&pool)
                    .await
                    .unwrap();
                
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
