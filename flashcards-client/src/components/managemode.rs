use yew::{Properties, HtmlResult, use_state, Callback, component, html};
use crate::{
    ActionButton,
    FlashCardMode,
    Callbacks,
    CardDiv, 
    AddNewCardForm,
    Card,
    CardState,
};

#[derive(Properties, PartialEq)]
pub struct ManageModeProperties {
    pub cards: Vec<CardState>,
    pub delete_card: Callback<CardState>,
    pub add_card: Callback<Card>,
    pub update_card: Callback<Card>
}

#[derive(Clone, PartialEq)]
pub enum ManageModeType {
    AddCard,
    ViewCards,
}

#[component]
pub fn ManageMode(ManageModeProperties { add_card, delete_card, update_card, cards }: &ManageModeProperties) -> HtmlResult {

    let card_index = use_state(|| 0);
    let cards = cards.clone();
    let manage_mode_type = use_state(|| ManageModeType::ViewCards);

    let next_card = Callbacks::make_next_card_callback(card_index.clone(), cards.len() - 1);
    let prev_card = Callbacks::make_prev_card_callback(card_index.clone());
    let delete_card = Callbacks::delete_card_emit_callback(cards.clone(), delete_card.clone(), card_index.clone());
    //let add_card = Callbacks::make_add_card_emit_callback(add_card.clone());

    let update_card = update_card.clone();
    let has_previous = *card_index > 0;
    let has_next = *card_index < cards.len() - 1;

    let switch = {

        let manage_mode_type = manage_mode_type.clone();

        Callback::from(move |_| {
            let type_to_set = match *manage_mode_type {
                ManageModeType::AddCard => ManageModeType::ViewCards,
                ManageModeType::ViewCards => ManageModeType::AddCard,
            };

            manage_mode_type.set(type_to_set);
        })
    };

    let add_card = {
        let add_card = add_card.clone();
        let manage_mode_type = manage_mode_type.clone();

        Callback::from(move |card: Card| {
            add_card.emit(card);
            manage_mode_type.set(ManageModeType::ViewCards);
        })
    };

    

    let card = &cards[*card_index];
    
    if *manage_mode_type == ManageModeType::AddCard {
        return Ok(html! {
            <div class="content">
                <AddNewCardForm on_update={update_card} on_add={add_card} />
                <div class="button-container">
                    <ActionButton aria_label="Exit" onclick={switch.clone()} icon="X" />
                </div>
            </div>
        });
    }

    Ok(html! {
        <div class="content">
            <CardDiv save_callback={update_card} mode={FlashCardMode::Manage} card={card.clone()} />
            <div class="button-container">
                <ActionButton enabled={has_previous} aria_label="Previous" onclick={prev_card} icon="\u{2B05}" />
                <ActionButton aria_label="Add" onclick={switch.clone()} icon="\u{1F5CE}" />
                <ActionButton aria_label="Delete" onclick={delete_card} icon="\u{1F5D1}" />
                <ActionButton enabled={has_next} aria_label="Next" onclick={next_card} icon="\u{27A1}" />
            </div>
            //<h1>{ "Add Card" }</h1>
            /*<div>
                <AddNewCardForm on_update={update_card} on_add={add_card} />
            </div>*/

        </div>
    }) 
}

