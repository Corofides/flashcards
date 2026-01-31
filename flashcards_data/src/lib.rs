use serde::Serialize;

#[derive(Clone, Default, PartialEq, Serialize)]
pub struct Card {
    id: usize,
    front: String,
    back: String,
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
