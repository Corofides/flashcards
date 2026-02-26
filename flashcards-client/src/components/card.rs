use yew::prelude::*;
use yew::{Html, component, html, Properties, Callback};
use flashcards_data::{Card, CardSide, CardState};
use crate::FlashCardMode;
use crate::components::actionbutton::ActionButton;
use chrono::DateTime;
use web_sys::HtmlInputElement;

type MouseCallback = Callback<yew::MouseEvent>;

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

fn render_for_manage(card: &CardState, card_for_edit: UseStateHandle<Card>, save_card: MouseCallback, edit_card: MouseCallback, manage_mode: ManageMode) -> Html {

    let card = card.card();
    let format = "%Y-%m-%d %H:%M:%S%.9f %Z";
    //let format = "%Y-%m-%d %H:%M:%S%.9f %Z";

    let dt = DateTime::parse_from_str(card.next_review(), format);
    
    let _review_date = match dt {
        Ok(dt) => {
            format!("{}", dt.format("%d-%m %H:%M"))
        },
        Err(_err) => {
            "Unknown".to_string()
        }
    };

    let on_back_input = {
        let card_for_edit = card_for_edit.clone();

        Callback::from(move |e: InputEvent| {

            let input: HtmlInputElement = e.target_unchecked_into();
            let card_for_edit = card_for_edit.clone();

            let front: String = (*card_for_edit.front()).to_string();

            let mut new_card = Card::new(
                *card_for_edit.id(),
                front,
                input.value(),
            );

            new_card.set_interval(*card_for_edit.interval());
            new_card.set_ease_factor(*card_for_edit.ease_factor());
            new_card.set_next_review(&*card_for_edit.next_review());

            card_for_edit.set(new_card);

        })
    };

    let on_front_input = {
        let card_for_edit = card_for_edit.clone();

        Callback::from(move |e: InputEvent| {

            let input: HtmlInputElement = e.target_unchecked_into();
            let card_for_edit = card_for_edit.clone();

            let back: String = (*card_for_edit.back()).to_string();

            let mut new_card = Card::new(
                *card_for_edit.id(),
                input.value(),
                back,
            );

            new_card.set_interval(*card_for_edit.interval());
            new_card.set_ease_factor(*card_for_edit.ease_factor());
            new_card.set_next_review(&*card_for_edit.next_review());


            card_for_edit.set(new_card);
            
        })
    };

    if manage_mode == ManageMode::Edit {
        return html! {
            <div class={"card card--manage"}>
                <div class="card-content">
                    <h2>{ format!("Card: {}", card.id()) }</h2>
                    <input value={card_for_edit.front().to_string()} oninput={on_front_input} type="text" />
                    <input value={card_for_edit.back().to_string()} oninput={on_back_input} type="text" />
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
    let card_for_edit = use_state(|| Card::new(0, String::new(), String::new()));

    let edit_card = {
        let manage_mode = manage_mode.clone();
        let card_for_edit = card_for_edit.clone();
        let card = card.clone();

        Callback::from(move |_| {
            manage_mode.set(ManageMode::Edit);
            let new_card = card.card().clone();
            card_for_edit.set(new_card);
        })
    };

    let save_card = {
        let manage_mode = manage_mode.clone();
        let card_for_edit = card_for_edit.clone();

        Callback::from(move |_| {
            log::info!("Card: {:?}", card_for_edit);
            manage_mode.set(ManageMode::View);
        })
    };


    match mode {
        FlashCardMode::Manage => {
            render_for_manage(&card, card_for_edit, save_card, edit_card, *manage_mode)
        },
        FlashCardMode::Study => {
            let flip = flip.clone().unwrap();
            render_for_study(&card, flip)
        }
    }
    
}
