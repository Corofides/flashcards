use yew::prelude::*;
use flashcards_data::{Card};
use crate::reducers::newcard::NewCardAction;
use web_sys::HtmlInputElement;
use crate::card_hooks::{use_new_card};

// Properties Struct.
//
// This allows us to pass various properties to a child component. In this case I'm passing the
// callback on_add to allow us to add a card to the cards list.
#[derive(Properties, PartialEq)]
pub struct AddCardProps {
    pub on_add: Callback<Card>,
}

// Component Function
//
// AddNewCardFrom adds a component to yew. We pass in some props, in this case
// AddCardProps. We then return a HtmlResult which is of the form Ok(Html), Err(_)
#[component]
pub fn AddNewCardForm(props: &AddCardProps) -> HtmlResult {

    // Get the data from our hook. We are using a reducer to manage this.
    let (result, reducer) = use_new_card();
   
    // Get the dispatcher, this allows us to dispatch events to our reducer which then modifies
    // the state that we are using.
    let dispatcher = reducer.dispatcher();

    // This is a bit weird and want to explain. So we need a Callback of the type
    //
    // move | e: InputEvent |
    //
    // however this needs to act upon the dispatcher. Due to the way Rust works we need
    // to provide a new dispatcher variable to the Callback otherwise move would mean that
    // the ownership of our existing dispatcher would be taken by the Callback and we'd
    // no longer be able to use it.
    //
    // So we create a block and assign the block to on_front_input, this block then
    // returns our callback. (Note: the lack of semi-colon.
    //
    // Before this we shadow the pre-existing dispatcher with a new variable called
    // dipatcher which we set to a clone of the existing dispatcher. This is then moved
    // into the callback so we can use it.
    //
    // When the block ends this variable becomes out of scope, so the shadowing ends and
    // our initial dispatcher variable, which still has the original dispatcher item
    // is free for us to continue to use.
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

            // On add is a callback we have provided to this component.
            // on_add.emit(card) just means run the callback and pass the
            // parameter card.
            on_add.emit(card);
            dispatcher.dispatch(NewCardAction::ResetCard);

            // e is a wrapper for an event in JS, like in js we need to prevent
            // the default action from occurring otherwise on the form submission
            // the page would reload which we don't want.
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
