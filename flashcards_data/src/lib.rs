use sqlx::FromRow;
use serde::{Serialize,Deserialize};

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
    ease_factor: f32,
    interval: f32,
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
    pub fn get_side(&self) -> &CardSide {
        &self.side
    }
    pub fn get_card(&self) -> &Card {
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
            ease_factor: 2.5_f32,
            interval: 0.0,
        }
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_front(&self) -> &str {
        &self.front
    }
    pub fn get_back(&self) -> &str {
        &self.back
    }
    pub fn next_review(&self) -> &str {
        &self.next_review
    }
    pub fn ease_factor(&self) -> &f32 {
        &self.ease_factor
    }
    pub fn interval(&self) -> &f32 {
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
    pub fn set_ease_factor(&mut self, ease_factor: f32) {
        self.ease_factor = ease_factor;
    }
    pub fn set_interval(&mut self, interval: f32) {
        self.interval = interval;
    }
}
