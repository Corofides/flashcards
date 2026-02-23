use yew::prelude::*;
use flashcards_data::{CardDifficulty, ReviewCardPayload, CreateCardPayload, Card, CardState, CardSide};
use crate::reducers::flashcards::FlashCardAction;

mod card_hooks;
mod reducers;
mod components;
use crate::card_hooks::{use_flash_cards};
use components::{
    add_card_form::{AddNewCardForm},
    managemode::{ManageMode},
    studymode::{StudyMode},
};
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

fn make_add_card_emit_callback(
        add_card: Callback<Card>,
    ) -> Callback<Card> {

    let add_card = add_card.clone();

    Callback::from(move |card: Card| {
        let card = card.clone();
        add_card.emit(card);
    })
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

    if *current_mode == FlashCardMode::Study {
        return Ok(html! {
            <div>
                <button onclick={change_mode}>{ "Manage Mode" }</button>
                <StudyMode cards={(*cards).clone()} review_card={review_card} flip_card={flip_card} />
            </div>
        });
    }

    Ok(html! {
        <div>
            <button onclick={change_mode}>{ "Study Mode" }</button>
            <ManageMode cards={(*cards).clone()} add_card={add_card} delete_card={delete_card} />
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
