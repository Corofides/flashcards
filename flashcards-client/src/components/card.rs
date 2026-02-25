use yew::prelude::*;
use yew::{Html, component, html, Properties, Callback};
use flashcards_data::{CardSide, CardState};
use crate::FlashCardMode;
use crate::components::actionbutton::ActionButton;
use chrono::DateTime;

#[derive(Properties, PartialEq)]
pub struct CardProperties {
    pub card: CardState,
    pub mode: FlashCardMode,
    #[prop_or(false)]
    pub edit: bool,
    #[prop_or(None)]
    pub flip: Option<Callback<yew::MouseEvent>>,
    #[prop_or(None)]
    pub edit_callback: Option<Callback<yew::MouseEvent>>,
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

#[derive(Clone, Copy, PartialEq)]
pub enum ManageMode {
    View,
    Edit,
}

fn render_for_manage(card: &CardState, edit_card: Callback<yew::MouseEvent>, manage_mode: ManageMode) -> Html {

    let card = card.card();
    let format = "%Y-%m-%d %H:%M:%S%.9f %Z";
    //let format = "%Y-%m-%d %H:%M:%S%.9f %Z";

    let dt = DateTime::parse_from_str(card.next_review(), format);

    
    let save_card = {
        Callback::from(|_| {
        })
    };

    let _review_date = match dt {
        Ok(dt) => {
            format!("{}", dt.format("%d-%m %H:%M"))
        },
        Err(_err) => {
            "Unknown".to_string()
        }
    };

    if manage_mode == ManageMode::Edit {
        return html! {
            <div class={"card card--manage"}>
                <div class="card-content">
                    <h2>{ format!("Card: {}", card.id()) }</h2>
                    <div class="description">{ format!("Next Review: {}", card.next_review()) }</div>
                    //<div class="description">{ format!("Next Review: {}", review_date) }</div>
                    <div class="description">{ format!("Front of Card: {}", card.front()) }</div>
                    <div class="description">{ format!("Back of Card: {}", card.back()) }</div>
                    <div class="description">{ format!("Ease Factor: {}", card.ease_factor()) }</div>
                    <div class={"card-actions"}>
                        <ActionButton aria_label="Save Card" onclick={save_card} icon={"S"} />
                    </div>
                </div>
            </div>
        };
    }

    html! {
        <div class={"card card--manage"} >
            <div class="card-content">
                <h2>{ format!("Card: {}", card.id()) }</h2>
                <div class="description">{ format!("Next Review: {}", card.next_review()) }</div>
                //<div class="description">{ format!("Next Review: {}", review_date) }</div>
                <div class="description">{ format!("Front of Card: {}", card.front()) }</div>
                <div class="description">{ format!("Back of Card: {}", card.back()) }</div>
                <div class="description">{ format!("Ease Factor: {}", card.ease_factor()) }</div>
                <div class={"card-actions"}>
                    <ActionButton aria_label="Edit Card" onclick={edit_card} icon="\u{1F527}" />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CardDiv(CardProperties { mode, card, flip, edit, edit_callback }: &CardProperties) -> Html {

    let manage_mode = use_state(|| ManageMode::View);

    let edit_card = {
        let manage_mode = manage_mode.clone();

        Callback::from(move |_| {
            manage_mode.set(ManageMode::Edit);
        })
    };


    match mode {
        FlashCardMode::Manage => {
            render_for_manage(&card, edit_card, *manage_mode)
        },
        FlashCardMode::Study => {
            let flip = flip.clone().unwrap();
            render_for_study(&card, flip)
        }
    }
    
}
