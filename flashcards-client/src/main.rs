use yew::prelude::*;
use flashcards_data::{Card, CardState, CardSide};
use implicit_clone::ImplicitClone;
use implicit_clone::unsync::{IArray};

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

    let cards = use_state(|| vec![
        CardState::new(Card::new(1, String::from("Ballet Flats"), String::from("\
            Simple slip-on shoes with very thin soles and no heel\
        "))),
        CardState::new(Card::new(2, String::from("Pumps (Court Shoes)"), String::from("\
            The quintessential heeled shoe. They are closed toe and usually have a seamless, low cut front"
        ))),
        CardState::new(Card::new(3, String::from("Loafers"), String::from("\
            A more structured, masculine-inspired slip-on shoe."
        ))),
    ]);

    let card_index = use_state(|| 0);
    //let current_card = use_state(|| cards[*card_index].clone());
    let total_cards = cards.len();

    let card = cards[*card_index].clone();

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
            /*current_card.flip_card();
            current_card.set(*current_card);*/
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


    html! {
        <div>
            <CardDiv card={card} />
            <button onclick={prev_card}>{ "Prev Card" }</button>
            <button onclick={flip_card}>{ "Turn Card" }</button>
            <button onclick={next_card}>{ "Next Card" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
