use yew::{use_reducer, use_state, UseReducerHandle, hook};
use yew::suspense::{Suspension, SuspensionResult};
use std::rc::Rc;
use flashcards_data::{Card, CardState};
use gloo_net::http::Request;
use crate::reducers::{
    flashcards::{FlashCardAction, FlashCardsState},
    newcard::{NewCardState},
};

#[hook]
pub fn use_new_card() -> (Rc<Card>, UseReducerHandle<NewCardState>) {

    let reducer = use_reducer(|| NewCardState::new());

    (reducer.card.clone(), reducer)

}

#[hook]
pub fn use_flash_cards() -> (SuspensionResult<Rc<Vec<CardState>>>, UseReducerHandle<FlashCardsState>) {

    let reducer = use_reducer(|| FlashCardsState::new());

    let suspension_handle = use_state(|| None);

    if reducer.cards.is_empty() {
        if let Some(suspension) = (*suspension_handle).clone() {
            return (Err(suspension), reducer);
        }

        let (suspension, comp_handle) = Suspension::new();
        suspension_handle.set(Some(suspension.clone()));

        let dispatcher = reducer.dispatcher();
        
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_cards: Vec<Card> = Request::get("http://localhost:3000/cards")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
                
            let fetched_cards: Vec<CardState> = fetched_cards.iter()
                .map(move |card| CardState::new(card.clone()))
                .collect();

            dispatcher.dispatch(FlashCardAction::SetData(fetched_cards));
            comp_handle.resume();

        });

        return (Err(suspension), reducer);

    }

    (Ok(reducer.cards.clone()), reducer)

}
