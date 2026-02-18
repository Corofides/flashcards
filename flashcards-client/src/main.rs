use yew::prelude::*;
use flashcards_data::{CardDifficulty, ReviewCardPayload, CreateCardPayload, Card, CardState, CardSide};
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

    let delete_card = {
        let dispatcher = reducer.dispatcher();
        let cards = cards.clone();
        let card_index = card_index.clone();

        move |_| {
            let dispatcher = dispatcher.clone();
            let cards = cards.clone();
            let card_index = card_index.clone();

            wasm_bindgen_futures::spawn_local(async move {

                let card = cards.get(*card_index).unwrap();
                let card = card.get_card();

                /*let delete_payload = DeleteCardPayload {
                    id: card.get_id(),
                };*/

                let delete_card_path = format!("http://localhost:3000/cards/{}", card.get_id());

                let response = Request::delete(&delete_card_path)
                    .send()
                    .await;

                match response {
                    Ok(response) if response.ok() => {
                        log!("Card was successfully removed!");
                        dispatcher.dispatch(FlashCardAction::RemoveCard(card.clone()));
                        card_index.set((*card_index).saturating_sub(1));
                    },
                    _ => {
                        log!("Error: Could not remove card");
                    }
                }

            });
        }
    };

    let update_card = {
        let dispatcher = reducer.dispatcher();
        let cards = cards.clone();
        let card_index = card_index.clone();

        move |card: Card| {
            let dispatcher = dispatcher.clone();
            let cards = cards.clone();
            let card_index = card_index.clone();

            wasm_bindgen_futures::spawn_local(async move {

                let current_card = cards.get(*card_index).unwrap();
                let current_card = current_card.get_card();

                let card_payload = CreateCardPayload {
                    front: card.get_front().to_string(),
                    back: card.get_back().to_string(),
                };

                let update_url = format!("http://localhost:3000/cards/{}", current_card.get_id());

                let response = Request::put(&update_url)
                    .json(&card_payload)
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(response) if response.ok() => {
                        let updated_card: Card = response.json().await.unwrap();
                        dispatcher.dispatch(FlashCardAction::UpdateCard(updated_card));
                    },
                    _ => {
                        log!("Error occurred updating the card");
                    }
                }
            });
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

    let review_card = {
        let dispatcher = reducer.dispatcher();
        let cards = cards.clone();
        let card_index = card_index.clone();

        move |difficulty: CardDifficulty| {

            let dispatcher = reducer.dispatcher();
            let cards = cards.clone();
            let card_index = card_index.clone();


            move |_| {

                let cards = cards.clone();
                let card_index = card_index.clone();
                let dispatcher = dispatcher.clone();
                let difficulty = difficulty.clone();

                wasm_bindgen_futures::spawn_local(async move {

                    let card = cards.get(*card_index).unwrap();
                    let review_payload = ReviewCardPayload {
                        difficulty, // : CardDifficulty::Medium
                    };

                    let url = format!("http://localhost:3000/cards/{}/review", card.get_card().get_id());

                    let response = Request::post(&url)
                        .json(&review_payload)
                        .unwrap()
                        .send()
                        .await;

                    match response {
                        Ok(response) if response.ok() => {
                            let reviewed_card: Card = response.json().await.unwrap();
                            log!(format!("Reviewed Card: {:?}", &reviewed_card));
                            dispatcher.dispatch(FlashCardAction::UpdateCard(reviewed_card));
                        },
                        _ => {},
                    }

                });
                log!("Reviewing the card!");
            }
        }
    };

    if cards.is_empty() {
        return Ok(html! {
            <div>
                <div>
                    <AddNewCardForm on_update={update_card} on_add={add_card} />
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
                <button onclick={delete_card}>{ "Delete Card" }</button>
            </div>
            <div>
                <button onclick={review_card(CardDifficulty::Easy)}>{ "Easy" }</button>
                <button onclick={review_card(CardDifficulty::Medium)}>{ "Medium" }</button>
                <button onclick={review_card(CardDifficulty::Hard)}>{ "Hard" }</button>
            </div>
            <div>
                <AddNewCardForm on_update={update_card} on_add={add_card} />
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
