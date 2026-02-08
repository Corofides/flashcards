use std::rc::Rc;
use crate::{Card, CardState};
use yew::prelude::*;

pub struct FlashCardsState {
    pub cards: Rc<Vec<CardState>>,
}

impl FlashCardsState {
    pub fn new() -> Self {
        Self {
            cards: Rc::new(Vec::new()),
        }
    }
}

pub enum FlashCardAction {
    SetData(Vec<CardState>),
    FlipCard(usize),
    AddCard(Card),
}

impl Reducible for FlashCardsState {
    type Action = FlashCardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            FlashCardAction::AddCard(card) => {
                let mut new_cards: Vec<CardState> = (*self.cards).clone();

                let new_card_state = CardState::new(card);

                FlashCardsState {
                    cards: Rc::new(new_cards),
                }.into()
            }
            FlashCardAction::SetData(cards) => {
                FlashCardsState {
                    cards: Rc::new(cards),
                }.into()
            },
            FlashCardAction::FlipCard(index) => {
                let mut new_cards: Vec<CardState> = (*self.cards).clone();

                if let Some(card) = new_cards.get_mut(index) {
                    (*card).flip_card();
                }

                FlashCardsState {
                    cards: Rc::new(new_cards),
                }.into()
            }
        }
    }
}
