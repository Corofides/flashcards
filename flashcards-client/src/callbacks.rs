use yew::{ UseStateHandle, MouseEvent, Callback };
use flashcards_data::{Card, CardDifficulty, CardState};

pub struct Callbacks;

impl Callbacks {
    pub fn make_next_card_callback(card_index: UseStateHandle<usize>, max_size: usize) -> Callback<yew::MouseEvent> {
        let card_index = card_index.clone();
        let max_size = max_size.clone();

        Callback::from(move |_| {
            let mut value = *card_index;

            if value < max_size {
                value = value.saturating_add(1);
            }

            card_index.set(value);

        })
    }
    pub fn make_prev_card_callback(card_index: UseStateHandle<usize>) -> Callback<yew::MouseEvent> {
        let card_index = card_index.clone();

        Callback::from(move |_| {
            let mut value = *card_index;

            if value > 0 {
                value = value.saturating_sub(1);
            }

            card_index.set(value);

        })
    }
    pub fn make_flip_card_emit_callback(
            card_index: UseStateHandle<usize>,
            cards: &Vec<CardState>,
            flip_card: Callback<CardState> 
        ) -> Callback<yew::MouseEvent> {

        let card_index = card_index.clone();
        let cards = cards.clone();
        let flip_card = flip_card.clone();

        Callback::from(move |_| {
            let card = cards[*card_index].clone();
            flip_card.emit(card);
        })

    }

    pub fn delete_card_emit_callback(
            cards: Vec<CardState>,
            delete_card: Callback<CardState>,
            card_index: UseStateHandle<usize>,
        ) -> Callback<yew::MouseEvent> {

        let cards = cards.clone();
        let delete_card = delete_card.clone();
        let card_index = card_index.clone();

        Callback::from(move |_| {
            let card = cards[*card_index].clone();
            delete_card.emit(card);
        })
            
    }
    pub fn make_review_card_emit_factory(
            card_index: UseStateHandle<usize>,
            cards: Vec<CardState>,
            review_card: Callback<(CardState, CardDifficulty)>
        ) -> Box<dyn Fn(CardDifficulty) -> Callback<yew::MouseEvent>> {

        let review_card = review_card.clone();
        let cards = cards.clone();
        let card_index = card_index.clone();

        Box::new(move |difficulty: CardDifficulty| {

            let review_card = review_card.clone();
            let card = cards[*card_index].clone();
            let difficulty = difficulty.clone();

            Callback::from(move |_e: MouseEvent | {
                review_card.emit((card.clone(), difficulty.clone()));
            })
        })

    }
    pub fn make_add_card_emit_callback(
            add_card: Callback<Card>,
        ) -> Callback<Card> {

        let add_card = add_card.clone();

        Callback::from(move |card: Card| {
            let card = card.clone();
            add_card.emit(card);
        })
    }
}
