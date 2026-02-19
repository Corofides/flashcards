use sqlx::FromRow;
use serde::{Serialize,Deserialize};
use chrono::{Utc, DateTime};


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
}

impl Card {
    pub fn new(id: u32, front: String, back: String) -> Self {
        Card {
            id,
            front,
            back,
            next_review: String::new(),
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
        let card_review_date: DateTime<Utc> = self.next_review().parse().expect("Valid date");

        card_review_date < current_date
    }
}
