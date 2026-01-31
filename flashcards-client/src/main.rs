use yew::prelude::*;
use flashcards_data::Card;

#[derive(Properties, PartialEq)]
pub struct Props {
    name: String,
}

#[derive(Properties, PartialEq)]
pub struct CardProperties {
    card: Card,
}

#[component]
fn CardDiv(CardProperties { card }: &CardProperties) -> Html {

    let title = card.get_front();
    let back = card.get_back();

    html! {
        <>
            <h1>{title}</h1>
            <p>{back}</p>
        </>
    }
}

#[component]
fn App() -> Html {

    let cards = use_state(|| vec![
        Card::new(1, String::from("Ballet Flats"), String::from("\
            Simple slip-on shoes with very thin soles and no heel\
        ")),
        Card::new(2, String::from("Pumps (Court Shoes)"), String::from("\
            The quintessential heeled shoe. They are closed toe and usually have a seamless, low cut front"
        )),
        Card::new(3, String::from("Loafers"), String::from("\
            A more structured, masculine-inspired slip-on shoe."
        )),
    ]);

    let card_index = use_state(|| 0);
    let total_cards = cards.len();

    let next_card = {
        let card_index = card_index.clone();
        move |_| {
            let value = (*card_index + 1) % total_cards;
            card_index.set(value);
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

    let card = cards[*card_index].clone();

    html! {
        <div>
            <CardDiv card={card} />
            <button onclick={prev_card}>{ "Prev Card" }</button>
            <button onclick={next_card}>{ "Next Card" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
