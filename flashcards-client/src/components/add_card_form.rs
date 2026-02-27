use yew::prelude::*;
use flashcards_data::{Card};
use crate::reducers::newcard::NewCardAction;
use web_sys::HtmlInputElement;
use crate::card_hooks::{use_new_card};
use crate::ActionButton;

// Properties Struct.
//
// This allows us to pass various properties to a child component. In this case I'm passing the
// callback on_add to allow us to add a card to the cards list.
#[derive(Properties, PartialEq)]
pub struct AddCardProps {
    pub on_add: Callback<Card>,
    pub on_update: Callback<Card>,
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

    let update_card = {
        let on_update = props.on_update.clone();
        let card = result.clone();
        let dispatcher = dispatcher.clone();

        Callback::from(move |e: SubmitEvent| {
            let card = (*card).clone();

            on_update.emit(card);
            dispatcher.dispatch(NewCardAction::ResetCard);

            e.prevent_default();
        })
    };

    let add_card = {

        let on_add = props.on_add.clone();
        let card = result.clone();
        let dispatcher = dispatcher.clone();

        Callback::from(move |e: MouseEvent| {
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
        })
    };

    /* let on_submit = {

        let update_card = update_card.clone();
        let add_card = add_card.clone();

        Callback::from(move |e: SubmitEvent| {

            let value = e.submitter(); //.value();
            
            if let Some(value) = value {
                if let Some(attribute) = value.get_attribute("value") {
                    if attribute == "add_card" {
                        add_card.emit(e);
                    } else if attribute == "update_card" {
                        update_card.emit(e);
                    }
                }
            }

        })

    };*/

    Ok(html! {

        <div class="card card--manage">
            <div class={"card-content"}>
                <h2>{ "Add a new card" }</h2>
                <input value={result.front().to_string()} oninput={on_front_input} type="text" />
                <input value={result.back().to_string()} oninput={on_back_input} type="text" />
                <div class={"card-actions"}>
                    <ActionButton aria_label="Add Card" onclick={add_card} icon="+" />
                </div>
                //<button value="update_card">{"Update Card"}</button>
            </div>
        </div>
    })
}
