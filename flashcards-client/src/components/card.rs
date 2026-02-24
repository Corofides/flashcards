use yew::{Html, component, html, Properties};
use flashcards_data::{CardSide, CardState};

#[derive(Properties, PartialEq)]
pub struct CardProperties {
    pub card: CardState,
}

#[component]
pub fn CardDiv(CardProperties { card }: &CardProperties) -> Html {

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
