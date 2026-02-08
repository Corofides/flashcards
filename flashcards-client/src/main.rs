use yew::prelude::*;
use flashcards_data::{CardState, CardSide};
use std::{rc::Rc};
use crate::reducers::flashcards::FlashCardAction;
use gloo_console::log;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

mod card_hooks;
mod reducers;
use crate::card_hooks::use_flash_cards;

#[derive(Properties, PartialEq)]
pub struct Props {
    name: String,
}

#[derive(Properties, PartialEq)]
pub struct CardProperties {
    card: CardState,
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
fn AddNewCardForm() -> Html {

    let mut front = String::new();
    let mut back = String::new();

    let edit = move |input: HtmlInputElement| {
        let value = input.value();
        front = String::from(value);
        log!("{value}");
    }; 

    let onkeypress = move |e: KeyboardEvent| {
        log!("{:?}", e.key());
        //|| {edit(e.target_unchecked_into())};
        //front = format!("{}{}", front, e.key());
    };

    let add_card = {
        move |_| {
            log!("adding the card");
        }
    };

    html! {
        <form>
            <input value={front} onkeypress={onkeypress} type="text" />
            <input type="text" />
            <button onclick={add_card}>{"Add Card"}</button>
        </form>
    }
}

#[component]
fn Content() -> HtmlResult {
    let (result, reducer) = use_flash_cards();
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

        let card_index = card_index.clone();
        let dispatcher = reducer.dispatcher();

        move |_| {
            dispatcher.dispatch(FlashCardAction::FlipCard(*card_index));
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
            <div>
                <CardDiv card={card.clone()} />
                <button onclick={prev_card}>{ "Prev Card" }</button>
                <button onclick={flip_card}>{ "Turn Card" }</button>
                <button onclick={next_card}>{ "Next Card" }</button>
            </div>
            <div>
                <AddNewCardForm />
            </div>
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
