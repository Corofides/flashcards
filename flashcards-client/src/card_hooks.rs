use yew::{use_state, UseStateHandle, hook};
use yew::suspense::{Suspension, SuspensionResult};
use std::rc::Rc;
use flashcards_data::{Card, CardState};
use gloo_net::http::Request;


#[hook]
pub fn use_flash_cards() -> (SuspensionResult<Rc<Vec<CardState>>>, UseStateHandle<Option<Rc<Vec<CardState>>>>) {

    let result_handle = use_state(|| None);
    let suspension_handle = use_state(|| None);

    let result = if let Some(cards) = (*result_handle).clone() {
        Ok(cards)
    } else if let Some(suspension) = (*suspension_handle).clone() {
        Err(suspension)
    } else {

        let (suspension, comp_handle) = Suspension::new();
        suspension_handle.set(Some(suspension.clone()));

        let result_handle = result_handle.clone();

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

            result_handle.set(Some(Rc::new(fetched_cards)));
            comp_handle.resume();

        });

        Err(suspension)

    };

    (result, result_handle)
}
