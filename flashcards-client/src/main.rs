use yew::prelude::*;
use flashcards_data::{Card, CardState, CardSide};
use implicit_clone::ImplicitClone;
use implicit_clone::unsync::{IArray};
use gloo_net::http::Request;

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
fn App() -> Html {

    let cards = use_state(|| {
        let vec: Vec<CardState> = Vec::new();
        vec
    });
    {
        let cards = cards.clone();
        use_effect_with((), move |_| {
            let cards = cards.clone();
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

                cards.set(fetched_cards);
            });
        });
    }
    
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
        let card_index = card_index.clone();
        //let current_card = current_card.clone();
        move |_| {

            let mut new_cards = (*cards).clone();

            if let Some(card) = new_cards.get_mut(*card_index) {
                card.flip_card();
            }

            cards.set(new_cards);
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

    let card = cards.get(*card_index).clone();

    if let Some(card) = card {
        html! {
            <div>
                <CardDiv card={card.clone()} />
                <button onclick={prev_card}>{ "Prev Card" }</button>
                <button onclick={flip_card}>{ "Turn Card" }</button>
                <button onclick={next_card}>{ "Next Card" }</button>
            </div>
        }
    } else {
        html! {
            <div>Loading...</div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
