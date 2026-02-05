use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use flashcards_data::{Card, CardState, CardSide};
use implicit_clone::ImplicitClone;
use implicit_clone::unsync::{IArray};
use gloo_net::http::Request;
use std::{cell::RefCell, rc::Rc};

#[derive(Properties, PartialEq)]
pub struct Props {
    name: String,
}

#[derive(Properties, PartialEq)]
pub struct CardProperties {
    card: CardState,
}

#[hook]
fn use_flash_cards() -> (SuspensionResult<Rc<Vec<CardState>>>, UseStateHandle<Option<Rc<Vec<CardState>>>>) {

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


#[component]
fn CardDiv(CardProperties { card }: &CardProperties) -> Html {


    let (title, content) = match card.get_side() {
        CardSide::Front => ("Front", card.get_card().get_front()),
        CardSide::Back => ("Back", card.get_card().get_back()),
    };

    html! {
        <>
            <h1>{title}</h1>
            <p>{content}</p>
        </>
    }
}

#[component]
fn Content() -> HtmlResult {
    let (result, handle) = use_flash_cards();
    let cards = result?;
    
    let card_index = use_state(|| 0);
    let total_cards = cards.len();

    let next_card = {
        let card_index = card_index.clone();
        move |_| {
            let value = (*card_index + 1) % total_cards;
            card_index.set(value);
        }
    };

    let flip_card = {
        let cards = cards.clone();
        let handle = handle.clone();
        let card_index = card_index.clone();
        //let current_card = current_card.clone();
        move |_| {

            let mut cards = (*cards).clone();

            if let Some(card) = cards.get_mut(*card_index) {
                card.flip_card();
            }

            handle.set(Some(Rc::new(cards)));

        }
    };

    let prev_card = {
        let card_index = card_index.clone();
        move |_| {

            let value = if *card_index == 0 {
                total_cards.saturating_sub(1)
            } else {
                *card_index - 1
            };

            card_index.set(value);
        }
    };

    let card = cards.get(*card_index).unwrap(); //.clone();

    Ok(html! {
        <div>
            <CardDiv card={card.clone()} />
            <button onclick={prev_card}>{ "Prev Card" }</button>
            <button onclick={flip_card}>{ "Turn Card" }</button>
            <button onclick={next_card}>{ "Next Card" }</button>
        </div>
    })

}

#[component]
fn App() -> Html {

    let fallback = html! {<div>{ "Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
