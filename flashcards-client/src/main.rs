use yew::prelude::*;
use flashcards_data::{CreateCardPayload, Card, CardState, CardSide};
use crate::reducers::flashcards::FlashCardAction;

mod card_hooks;
mod reducers;
mod components;
use crate::card_hooks::{use_flash_cards};
use components::add_card_form::{AddNewCardForm};
use gloo_net::http::Request;
use gloo_console::log;

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

/*#[derive(Properties, PartialEq)]
pub struct AddCardProps {
    pub on_add: Callback<Card>,
}

#[component]
fn AddNewCardForm(props: &AddCardProps) -> HtmlResult {

    let (result, reducer) = use_new_card();
    
    let dispatcher = reducer.dispatcher();

    let on_front_input = {
        let dispatcher = dispatcher.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            dispatcher.dispatch(NewCardAction::SetFront(
                input.value()        
            ));
        })
    };

    let on_back_input = {

        let dispatcher = dispatcher.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            dispatcher.dispatch(NewCardAction::SetBack(input.value()));
        })
    };

    let add_card = {

        let on_add = props.on_add.clone();
        let card = result.clone();
        let dispatcher = dispatcher.clone();

        move |e: SubmitEvent| {
            let card = (*card).clone();
            on_add.emit(card);
            /*cards_dispatcher.dispatch(FlashCardAction::AddCard(Card::new(
                cards.len(), String::from(card.get_front()), String::from(card.get_back())
            )));*/
            dispatcher.dispatch(NewCardAction::ResetCard);
            e.prevent_default();
        }
    };

    Ok(html! {
        <form onsubmit={add_card}>
            <input value={result.get_front().to_string()} oninput={on_front_input} type="text" />
            <input value={result.get_back().to_string()} oninput={on_back_input} type="text" />
            <button >{"Add Card"}</button>
        </form>
    })
} */

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

    let add_card = {
        let dispatcher = reducer.dispatcher();
        move |card: Card| {

            let dispatcher = dispatcher.clone();
            //"http://localhost:3000/cards"
            wasm_bindgen_futures::spawn_local(async move {

                let card_payload = CreateCardPayload {
                    front: card.get_front().to_string(),
                    back: card.get_back().to_string(),
                };

                let response = Request::post("http://localhost:3000/cards")
                    .json(&card_payload)
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(response) if response.ok() => {
                        let saved_card: Card = response.json().await.unwrap();
                        dispatcher.dispatch(FlashCardAction::AddCard(saved_card));
                    },
                    _ => {
                        log!("Error: Could not add card.");
                    }
                }
            });

        }
    };

    if cards.is_empty() {
        return Ok(html! {
            <div>
                <div>
                    <AddNewCardForm on_add={add_card} />
                </div>
            </div>
        });
    }

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
                <AddNewCardForm on_add={add_card} />
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
