use std::rc::Rc;
use crate::{Card, CardState};
use yew::prelude::*;
use gloo_console::log;
use wasm_bindgen::JsValue;

pub struct FlashCardsState {
    pub cards: Rc<Vec<CardState>>,
    pub has_pulled: bool,
}

impl FlashCardsState {
    pub fn new() -> Self {
        Self {
            cards: Rc::new(Vec::new()),
            has_pulled: false,
        }
    }
}

pub enum FlashCardAction {
    SetData(Vec<CardState>),
    FlipCard(usize),
    AddCard(Card),
    UpdateCard(Card),
    RemoveCard(Card),
}

impl Reducible for FlashCardsState {
    type Action = FlashCardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            FlashCardAction::UpdateCard(card) => {
                let mut new_cards: Vec<CardState> = (*self.cards).clone();

                let card_position = new_cards.iter()
                    .position(|current_card| {
                        current_card.get_card().get_id() == card.get_id()
                    });

                if let Some(card_position) = card_position {

                    let new_card_state = CardState::new(card);

                    new_cards[card_position] = new_card_state;

                    // let current_card = new_cards.get_mut(card_position).unwrap();

                    // current_card.get_card().set_front(card.get_front());
                    // current_card.get_card().set_back(card.get_back());
                }

                FlashCardsState {
                    cards: Rc::new(new_cards),
                    has_pulled: true,
                }.into()

            },
            FlashCardAction::AddCard(card) => {
                let mut new_cards: Vec<CardState> = (*self.cards).clone();

                let new_card_state = CardState::new(card);

                log!(JsValue::from(new_card_state.get_card().get_id()));
                log!(JsValue::from(new_card_state.get_card().get_front()));

                new_cards.push(new_card_state);

                FlashCardsState {
                    cards: Rc::new(new_cards),
                    has_pulled: true
                }.into()
            }
            FlashCardAction::RemoveCard(card) => {
                let mut new_cards: Vec<CardState> = (*self.cards).clone();

                let card_position = new_cards.iter()
                    .position(|current_card| {
                        current_card.get_card().get_id() == card.get_id()
                    });

                if let Some(card_position) = card_position {
                    new_cards.remove(card_position);
                };

                FlashCardsState {
                    cards: Rc::new(new_cards),
                    has_pulled: true,
                }.into()
            }
            FlashCardAction::SetData(cards) => {
                FlashCardsState {
                    cards: Rc::new(cards),
                    has_pulled: true,
                }.into()
            },
            FlashCardAction::FlipCard(index) => {
                let mut new_cards: Vec<CardState> = (*self.cards).clone();

                if let Some(card) = new_cards.get_mut(index) {
                    (*card).flip_card();
                }

                FlashCardsState {
                    cards: Rc::new(new_cards),
                    has_pulled: true,
                }.into()
            }
        }
    }
}
