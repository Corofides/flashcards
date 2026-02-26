use yew::prelude::*;
use flashcards_data::{ CardDifficulty, ReviewCardPayload, CreateCardPayload, Card, CardState };
use crate::reducers::flashcards::FlashCardAction;
use crate::components::actionbutton::ActionButton;

mod card_hooks;
mod reducers;
mod components;
mod callbacks;

use callbacks::{Callbacks};
use crate::card_hooks::{use_flash_cards};
use components::{
    add_card_form::{AddNewCardForm},
    managemode::{ManageMode},
    studymode::{StudyMode},
    card::{CardDiv},
};
use gloo_net::http::Request;
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct Props {
    name: String,
}

#[derive(PartialEq)]
pub enum FlashCardMode {
    Study,
    Manage,
}

#[component]
fn Content() -> HtmlResult {
    let (result, reducer) = use_flash_cards();
    let cards = result?;

    let current_mode = use_state(|| FlashCardMode::Study);
    let card_index = use_state(|| 0);
   
    let change_mode = {

        let current_mode = current_mode.clone();

        Callback::from(move |_| {
            let current_mode = current_mode.clone();

            let next_mode = match *current_mode {
                FlashCardMode::Study => {
                    FlashCardMode::Manage
                },
                FlashCardMode::Manage => {
                    FlashCardMode::Study
                },
            };

            current_mode.set(next_mode);

        

        })
    };

    let flip_card = {
        let cards = cards.clone();
        let dispatcher = reducer.dispatcher();

        Callback::from(move |card: CardState| {

            let dispatcher = dispatcher.clone();
            let cards = cards.clone();

            let card_id = card.card().id();

            let card = cards.iter()
                .position(|card| {
                    card.card().id() == card_id
                });

            if let Some(position) = card {
                dispatcher.dispatch(FlashCardAction::FlipCard(position));
            }
        
        })

    };

        
    let delete_card = {

        let dispatcher = reducer.dispatcher();
        let card_index = card_index.clone();

        Callback::from(move |card: CardState| {
            let dispatcher = dispatcher.clone();
            let card = card.clone();
            let card_index = card_index.clone();

            wasm_bindgen_futures::spawn_local(async move {

                let card = card.clone();
                let card = card.card();
                
                let delete_card_path = format!("http://localhost:3000/cards/{}", card.id());

                let response = Request::delete(&delete_card_path)
                    .send()
                    .await;

                match response {
                    Ok(response) if response.ok() => {
                        log!("Card was successfully removed!");
                        let value: usize = *card_index;
                        dispatcher.dispatch(FlashCardAction::RemoveCard(card.clone()));
                        card_index.set(value.saturating_sub(1));
                    },
                    _ => {
                        log!("Error: Could not remove card");
                    }
                }

            });
        })
    };

    let update_card = {
        let dispatcher = reducer.dispatcher();
        let cards = cards.clone();
        //let card_index = card_index.clone();

        Callback::from(move |card: Card| {
            let dispatcher = dispatcher.clone();
            let cards = cards.clone();
            //let card_index = card_index.clone();

            wasm_bindgen_futures::spawn_local(async move {

                let card_index = cards.iter().position(|current_card: &CardState| {
                    let current_card = current_card.card();
                    card.id() == current_card.id()
                }).unwrap();

                let current_card = cards.get(card_index).unwrap();
                let current_card = current_card.card();

                let card_payload = CreateCardPayload {
                    front: card.front().to_string(),
                    back: card.back().to_string(),
                };

                let update_url = format!("http://localhost:3000/cards/{}", current_card.id());

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
        })
    };

    let add_card = {
        let dispatcher = reducer.dispatcher();
        Callback::from(move |card: Card| {

            let dispatcher = dispatcher.clone();
            //"http://localhost:3000/cards"
            wasm_bindgen_futures::spawn_local(async move {

                let card_payload = CreateCardPayload {
                    front: card.front().to_string(),
                    back: card.back().to_string(),
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
        })
    };

    let review_card = {
        let dispatcher = reducer.dispatcher();

        Callback::from(move |(card, difficulty): (CardState, CardDifficulty)| {
            let dispatcher = dispatcher.clone();

            wasm_bindgen_futures::spawn_local(async move {

                let review_payload = ReviewCardPayload {
                    difficulty,
                };

                let url = format!("http://localhost:3000/cards/{}/review", card.card().id());

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

        })

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

    if *current_mode == FlashCardMode::Study {
        return Ok(html! {
            <div class="main main--study">
                <header>
                    <ActionButton aria_label="Manage" onclick={change_mode} icon="\u{2699}" />
                </header>
                <div class="content">
                    <StudyMode cards={(*cards).clone()} review_card={review_card} flip_card={flip_card} />
                </div>
            </div>
        });
    }

    Ok(html! {
        <div class="main main--manage">
            <header>
                <ActionButton aria_label="Study" onclick={change_mode} icon="\u{1F441}" />
            </header>
            <ManageMode cards={(*cards).clone()} update_card={update_card} add_card={add_card} delete_card={delete_card} />
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
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
