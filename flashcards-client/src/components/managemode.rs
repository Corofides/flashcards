use yew::{Properties, HtmlResult, use_state, Callback, component, html};
use crate::{
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
}

#[component]
pub fn ManageMode(ManageModeProperties { add_card, delete_card, cards }: &ManageModeProperties) -> HtmlResult {

    let card_index = use_state(|| 0);
    let cards = cards.clone();

    let next_card = Callbacks::make_next_card_callback(card_index.clone(), cards.len() - 1);
    let prev_card = Callbacks::make_prev_card_callback(card_index.clone());
    let delete_card = Callbacks::delete_card_emit_callback(cards.clone(), delete_card.clone(), card_index.clone());
    let add_card = Callbacks::make_add_card_emit_callback(add_card.clone());

    let update_card = {
        Callback::from(move |_| {
        })
    };

    let card = &cards[*card_index];
    
    Ok(html! {
        <div>
            <h1>{ "Manage Mode" }</h1>
            <CardDiv mode={FlashCardMode::Manage} card={card.clone()} />
            <button onclick={next_card}>{ "Next Card" }</button>
            <button onclick={delete_card}>{ "Delete" }</button>
            <button onclick={prev_card}>{ "Previous Card" }</button>
            <h1>{ "Add Card" }</h1>
            <div>
                <AddNewCardForm on_update={update_card} on_add={add_card} />
            </div>

        </div>
    }) 
}

