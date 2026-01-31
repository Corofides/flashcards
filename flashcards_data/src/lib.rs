use serde::Serialize;

#[derive(Clone, Default, PartialEq, Serialize)]
pub struct Card {
    id: usize,
    front: String,
    back: String,
}

#[derive(Default, PartialEq, Clone)]
pub enum CardSide {
    #[default]
    Front,
    Back,
}

#[derive(Clone, Default, PartialEq)]
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
    pub fn new(id: usize, front: String, back: String) -> Self {
        Card {
            id,
            front,
            back,
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_front(&self) -> &str {
        &self.front
    }
    pub fn get_back(&self) -> &str {
        &self.back
    }
}
