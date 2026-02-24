use yew::{Html, component, html, Properties};
use flashcards_data::{CardSide, CardState};
use crate::FlashCardMode;

#[derive(Properties, PartialEq)]
pub struct CardProperties {
    pub card: CardState,
    pub mode: FlashCardMode,
}

fn render_for_study(card: &CardState) -> Html {

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

fn render_for_manage(card: &CardState) -> Html {

    let card = card.card();

    html! {
        <>
            <h1>{ "Card" }</h1>
            <div>{ format!("Next Review: ") }</div>
            <div>{ format!("Front of Card: {}", card.front()) }</div>
            <div>{ format!("Back of Card: {}", card.back()) }</div>
            <div>{ format!("Ease Factor: {}", card.ease_factor()) }</div>
            <div>{ format!("{:?}", card) }</div>
        </>
    }
}

#[component]
pub fn CardDiv(CardProperties { mode, card }: &CardProperties) -> Html {

    match mode {
        FlashCardMode::Manage => {
            render_for_manage(&card)
        },
        FlashCardMode::Study => {
            render_for_study(&card)
        }
    }
    
}
