use yew::prelude::*;
use flashcards_data::{Card};
use crate::reducers::newcard::NewCardAction;
use web_sys::HtmlInputElement;
use crate::card_hooks::{use_new_card};


#[derive(Properties, PartialEq)]
pub struct AddCardProps {
    pub on_add: Callback<Card>,
}

#[component]
pub fn AddNewCardForm(props: &AddCardProps) -> HtmlResult {

    let (result, reducer) = use_new_card();
    
    let dispatcher = reducer.dispatcher();

    let on_front_input = {
        let dispatcher = dispatcher.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            dispatcher.dispatch(NewCardAction::SetFront(
                input.value()        
            ));
        })
    };

    let on_back_input = {

        let dispatcher = dispatcher.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            dispatcher.dispatch(NewCardAction::SetBack(input.value()));
        })
    };

    let add_card = {

        let on_add = props.on_add.clone();
        let card = result.clone();
        let dispatcher = dispatcher.clone();

        move |e: SubmitEvent| {
            let card = (*card).clone();
            on_add.emit(card);
            dispatcher.dispatch(NewCardAction::ResetCard);
            e.prevent_default();
        }
    };

    Ok(html! {
        <form onsubmit={add_card}>
            <input value={result.get_front().to_string()} oninput={on_front_input} type="text" />
            <input value={result.get_back().to_string()} oninput={on_back_input} type="text" />
            <button >{"Add Card"}</button>
        </form>
    })
}
