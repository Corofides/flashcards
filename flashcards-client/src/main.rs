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

#[derive(Properties, PartialEq)]
pub struct ManageModeProperties {
    cards: Vec<CardState>,
    delete_card: Callback<CardState>,
}

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

fn make_prev_card_callback(card_index: UseStateHandle<usize>) -> Callback<yew::MouseEvent> {
    let card_index = card_index.clone();

    Callback::from(move |_| {
        let mut value = *card_index;

        if value > 0 {
            value = value.saturating_sub(1);
        }

        card_index.set(value);

    })
}

fn make_next_card_callback(card_index: UseStateHandle<usize>, max_size: usize) -> Callback<yew::MouseEvent> {
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

fn make_flip_card_emit_callback(
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

fn delete_card_emit_callback(
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

fn make_review_card_emit_factory(
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

#[component]
fn ManageMode(ManageModeProperties { delete_card, cards }: &ManageModeProperties) -> HtmlResult {

    let card_index = use_state(|| 0);
    let cards = cards.clone();

    let next_card = make_next_card_callback(card_index.clone(), cards.len() - 1);
    let prev_card = make_prev_card_callback(card_index.clone());
    let delete_card = delete_card_emit_callback(cards.clone(), delete_card.clone(), card_index.clone());

    let update_card = {
        Callback::from(move |_| {
        })
    };

    let add_card = {
        Callback::from(move |_| {
        })
    };

    let card = &cards[*card_index];
    
    Ok(html! {
        <div>
            <h1>{ "Manage Mode" }</h1>
            <CardDiv card={card.clone()} />
            <button onclick={next_card}>{ "Next Card" }</button>
            <button onclick={delete_card}>{ "Delete" }</button>
            <button onclick={prev_card}>{ "Previous Card" }</button>
            <h1>{ "Add Card" }</h1>
            <div>
                <AddNewCardForm on_update={update_card} on_add={add_card} />
            </div>

        </div>
    }) 
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

    let prev_card = make_prev_card_callback(card_index.clone());
    let next_card = make_next_card_callback(card_index.clone(), cards.len() - 1);
    let flip_card = make_flip_card_emit_callback(card_index.clone(), &cards, flip_card.clone());
    let review_card = make_review_card_emit_factory(card_index.clone(), cards.clone(), review_card.clone());
    
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

    let next_card = make_next_card_callback(card_index.clone(), total_cards - 1);
    let prev_card = make_prev_card_callback(card_index.clone());
    
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
            <StudyMode cards={(*cards).clone()} review_card={review_card} flip_card={flip_card} />
            <ManageMode cards={(*cards).clone()} delete_card={delete_card} />
            <div>
                <CardDiv card={card.clone()} />
                <button onclick={prev_card}>{ "Prev Card" }</button>
                <button onclick={next_card}>{ "Next Card" }</button>
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
