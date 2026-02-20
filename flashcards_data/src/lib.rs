use sqlx::FromRow;
use serde::{Serialize,Deserialize};
use chrono::{Utc, DateTime};
use chrono::format::ParseError;


#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum CardDifficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReviewCardPayload {
    pub difficulty: CardDifficulty,
}

#[derive(FromRow, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Card {
    id: u32,
    front: String,
    back: String,
    ease_factor: u8,
    interval: u8,
    next_review: String,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub enum CardSide {
    #[default]
    Front,
    Back,
}

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct CreateCardPayload {
    pub front: String,
    pub back: String,
}

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
 pub struct DeleteCardPayload {
    pub id: u32,
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct CardState {
    card: Card,
    side: CardSide,

}

impl CardState {
    pub fn new(card: Card) -> Self {
        Self {
            card,
            side: CardSide::default(),
        }
    }
    pub fn flip_card(&mut self) {
        self.side = match self.side {
            CardSide::Front => CardSide::Back,
            CardSide::Back => CardSide::Front,
        }
    }
    pub fn side(&self) -> &CardSide {
        &self.side
    }
    pub fn card(&self) -> &Card {
        &self.card
    }
    pub fn is_front(&self) -> bool {
        self.side == CardSide::Front
    }
    pub fn is_back(&self) -> bool {
        !&self.is_front()
    }
}

impl Card {
    pub fn new(id: u32, front: String, back: String) -> Self {

        let dt = Utc::now();
        let date_time_string = dt.format("%Y-%m-%d %H:%M:%S%.9f %Z").to_string();
        println!("{}", date_time_string);

        Card {
            id,
            front,
            back,
            next_review: date_time_string, //Utc::now().to_string(),
//String::new(),
            ease_factor: 3,
            interval: 1,
        }
    }
    pub fn id(&self) -> &u32 {
        &self.id
    }
    pub fn front(&self) -> &str {
        &self.front
    }
    pub fn back(&self) -> &str {
        &self.back
    }
    pub fn next_review(&self) -> &str {
        &self.next_review
    }
    pub fn ease_factor(&self) -> &u8 {
        &self.ease_factor
    }
    pub fn interval(&self) -> &u8 {
        &self.interval
    }
    pub fn set_front(&mut self, front: &str) {
        self.front = String::from(front);
    }
    pub fn set_back(&mut self, back: &str) {
        self.back = String::from(back);
    }
    pub fn set_next_review(&mut self, next_review: &str) {
        self.next_review = String::from(next_review);
    }
    pub fn set_ease_factor(&mut self, ease_factor: u8) {
        self.ease_factor = ease_factor;
    }
    pub fn set_interval(&mut self, interval: u8) {
        self.interval = interval;
    }
    pub fn needs_review(&self) -> bool {
        let current_date = Utc::now();
        let card_review_date: Result<DateTime<Utc>, ParseError> = self.next_review().parse(); //.expect("Valid date");

        if let Ok(card_review_date) = card_review_date {
            println!("Card Review Date Success: {}", self.id());
            return card_review_date < current_date;
        } else {
            println!("Could not parse {}", self.id());
        }
            

        return false;
        //card_review_date < current_date
    }
}
