use yew::prelude::*;
use flashcards_data::{CardDifficulty, ReviewCardPayload, CreateCardPayload, Card, CardState, CardSide};
use crate::reducers::flashcards::FlashCardAction;

mod card_hooks;
mod reducers;
mod components;
use crate::card_hooks::{use_flash_cards};
use components::add_card_form::{AddNewCardForm};
use gloo_net::http::Request;
//use log::log;
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct Props {
    name: String,
}

#[derive(Properties, PartialEq)]
pub struct CardProperties {
    card: CardState,
}

#[derive(Properties, PartialEq)]
pub struct StudyModeProperties {
    flip_card:  Callback<CardState>,
    review_card: Callback<(CardState, CardDifficulty)>,
    cards: Vec<CardState>,
}

/*#[derive(Properties, PartialEq)]
pub struct StudyModeControlProperties {
    card: CardState,
}*/

#[component]
fn CardDiv(CardProperties { card }: &CardProperties) -> Html {


    let (title, content) = match card.side() {
        CardSide::Front => ("Front", card.card().front()),
        CardSide::Back => ("Back", card.card().back()),
    };

    html! {
        <>
            <h1>{title}</h1>
            <p>{content}</p>
        </>
    }
}

#[component]
fn StudyMode(StudyModeProperties { review_card, flip_card, cards }: &StudyModeProperties) -> HtmlResult {

    log::info!("Cards: {:?}", cards);
    let card_index = use_state(|| 0);

    let cards: Vec<CardState> = cards.iter()
        .filter(|card| {
            let card = card.card();
            card.needs_review()
        })
        .cloned()
        .collect();

    let total_cards = cards.len();

    if total_cards == 0 {
        return Ok(html! {
            <div>{ "You have no cards to review at this time." }</div>
        });
    }

    let prev_card = {

        let card_index = card_index.clone();

        Callback::from(move |_| {
            let card_index = card_index.clone();
            let mut value = *card_index;

            if *card_index > 0 {
                value -= 1;
            }

            card_index.set(value);
        })
    };

    let next_card = {
        let card_index = card_index.clone();
        let cards = cards.clone();

        Callback::from(move |_| {
            let card_index = card_index.clone();
            let cards = cards.clone();
            let mut value = *card_index;

            if *card_index < cards.len() - 1 {
                value += 1;
            }

            card_index.set(value);
        })
    };

    let flip_card = {
        let card_index = card_index.clone();
        let cards = cards.clone();
        let flip_card = flip_card.clone();

        Callback::from(move |_| {
            let card = cards[*card_index].clone();
            flip_card.emit(card);
        })
    };

    let review_card = {

        let review_card = review_card.clone();
        let cards = cards.clone();
        let card_index = card_index.clone();

        move |difficulty: CardDifficulty| {

            let review_card = review_card.clone();
            let card = cards[*card_index].clone();
            let difficulty = difficulty.clone();

            Callback::from(move |_e: MouseEvent | {
                review_card.emit((card.clone(), difficulty.clone()));
            })
        }
        
    };
    
    let card = &cards[*card_index];

    if card.is_front() {
        return Ok(html! {
            <div>
                <CardDiv card={card.clone()} />
                <button onclick={prev_card}>{ "Prev Card" }</button>
                <button onclick={flip_card}>{ "Turn Card" }</button>
                <button onclick={next_card}>{ "Next Card" }</button>
            </div>
        })
    }

    Ok(html! {
        <div>
            <CardDiv card={card.clone()} />
            <button onclick={review_card(CardDifficulty::Easy)}>{ "Easy" }</button>
            <button onclick={review_card(CardDifficulty::Medium)}>{ "Medium" }</button>
            <button onclick={review_card(CardDifficulty::Hard)}>{ "Hard" }</button>
        </div>
    })
    
}

#[component]
fn Content() -> HtmlResult {
    let (result, reducer) = use_flash_cards();
    let cards = result?;

    let card_index = use_state(|| 0);
    let total_cards = cards.len();

    let next_card = {
        let card_index = card_index.clone();
        Callback::from(move |_| {
            let value = (*card_index + 1) % total_cards;
            card_index.set(value);
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

    let prev_card = {
        let card_index = card_index.clone();

        Callback::from(move |_| {

            let value = if *card_index == 0 {
                total_cards.saturating_sub(1)
            } else {
                *card_index - 1
            };

            card_index.set(value);
        })
    };

    let delete_card = {
        let dispatcher = reducer.dispatcher();
        let cards = cards.clone();
        let card_index = card_index.clone();

        Callback::from(move |_| {
            let dispatcher = dispatcher.clone();
            let cards = cards.clone();
            let card_index = card_index.clone();

            wasm_bindgen_futures::spawn_local(async move {

                let card = cards.get(*card_index).unwrap();
                let card = card.card();
                
                let delete_card_path = format!("http://localhost:3000/cards/{}", card.id());

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
        })
    };

    let update_card = {
        let dispatcher = reducer.dispatcher();
        let cards = cards.clone();
        let card_index = card_index.clone();

        Callback::from(move |card: Card| {
            let dispatcher = dispatcher.clone();
            let cards = cards.clone();
            let card_index = card_index.clone();

            wasm_bindgen_futures::spawn_local(async move {

                let current_card = cards.get(*card_index).unwrap();
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

    let card = cards.get(*card_index).unwrap(); //.clone();

    Ok(html! {
        <div>
            <div>
                <StudyMode cards={(*cards).clone()} review_card={review_card} flip_card={flip_card} />
            </div>
            <div>
                <CardDiv card={card.clone()} />
                <button onclick={prev_card}>{ "Prev Card" }</button>
                //<button onclick={flip_card}>{ "Turn Card" }</button>
                <button onclick={next_card}>{ "Next Card" }</button>
                <button onclick={delete_card}>{ "Delete Card" }</button>
            </div>
            <div>
                //<button onclick={review_card(CardDifficulty::Easy)}>{ "Easy" }</button>
                //<button onclick={review_card(CardDifficulty::Medium)}>{ "Medium" }</button>
                //<button onclick={review_card(CardDifficulty::Hard)}>{ "Hard" }</button>
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
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
