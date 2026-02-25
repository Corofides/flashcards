use yew::{Html, component, html, Properties, Callback};
use flashcards_data::{CardSide, CardState};
use crate::FlashCardMode;
use crate::components::actionbutton::ActionButton;
use chrono::DateTime;

#[derive(Properties, PartialEq)]
pub struct CardProperties {
    pub card: CardState,
    pub mode: FlashCardMode,
    #[prop_or(None)]
    pub flip: Option<Callback<yew::MouseEvent>>,
}

fn render_for_study(card: &CardState, flip: Callback<yew::MouseEvent>) -> Html {

    let mut can_flip = false;
    let (title, content) = match card.side() {
        CardSide::Front => {
            can_flip = true;
            ("Front", card.card().front())
        },
        CardSide::Back => ("Back", card.card().back()),
    };

    html! {
        <div class={"card card--study"}>
            <div class={"card-content"}>
                <h2 class="title">{title}</h2>
                <p class="description">
                    {content}
                </p>
                //<button class="action-btn">{ "Turn Card" }</button>
                <div class={"card-actions"}>
                    <ActionButton enabled={can_flip} aria_label="Turn Card" onclick={flip} icon="\u{21BB}" />
                </div>

            </div>
        </div>
    }
}

fn render_for_manage(card: &CardState) -> Html {

    let card = card.card();
    let format = "%Y-%m-%d %H:%M:%S%.9f %Z";
    //let format = "%Y-%m-%d %H:%M:%S%.9f %Z";

    let dt = DateTime::parse_from_str(card.next_review(), format);

    let review_date = match dt {
        Ok(dt) => {
            format!("{}", dt.format("%d-%m %H:%M"))
        },
        Err(_err) => {
            "Unknown".to_string()
        }
    };

    html! {
        <div class={"card card--manage"} >
            <h1>{ "Card" }</h1>
            <div>{ format!("Next Review: {}", card.next_review()) }</div>
            <div>{ format!("Next Review: {}", review_date) }</div>
            <div>{ format!("Front of Card: {}", card.front()) }</div>
            <div>{ format!("Back of Card: {}", card.back()) }</div>
            <div>{ format!("Ease Factor: {}", card.ease_factor()) }</div>
            <div>{ format!("{:?}", card) }</div>
        </div>
    }
}

#[component]
pub fn CardDiv(CardProperties { mode, card, flip }: &CardProperties) -> Html {

    match mode {
        FlashCardMode::Manage => {
            render_for_manage(&card)
        },
        FlashCardMode::Study => {
            let flip = flip.clone().unwrap();
            render_for_study(&card, flip)
        }
    }
    
}
