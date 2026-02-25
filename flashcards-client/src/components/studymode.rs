use yew::{
    html,
    HtmlResult,
    use_state,
    component,
    Properties,
    Callback,
};
use crate::{
    components::actionbutton::ActionButton,
    FlashCardMode,
    Callbacks,
    CardDiv,
};
use flashcards_data::{
    CardState,
    CardDifficulty,
};

#[derive(Properties, PartialEq)]
pub struct StudyModeProperties {
    pub flip_card:  Callback<CardState>,
    pub review_card: Callback<(CardState, CardDifficulty)>,
    pub cards: Vec<CardState>,
}

#[component]
pub fn StudyMode(StudyModeProperties { review_card, flip_card, cards }: &StudyModeProperties) -> HtmlResult {

    log::info!("Cards: {:?}", cards);
    let card_index = use_state(|| 0);

    let cards: Vec<CardState> = cards.iter()
        .filter(|card| {
            let card = card.card();
            card.needs_review()
        })
        .cloned()
        .collect();

    let total_cards = cards.len();

    if total_cards == 0 {
        return Ok(html! {
            <div>{ "You have no cards to review at this time." }</div>
        });
    }

    let prev_card = Callbacks::make_prev_card_callback(card_index.clone());
    let next_card = Callbacks::make_next_card_callback(card_index.clone(), cards.len() - 1);
    let flip_card = Callbacks::make_flip_card_emit_callback(card_index.clone(), &cards, flip_card.clone());
    let review_card = Callbacks::make_review_card_emit_factory(card_index.clone(), cards.clone(), review_card.clone());
   
    let has_previous = *card_index > 0;
    let has_next = *card_index < cards.len() - 1;

    let card = &cards[*card_index];

    if card.is_front() {
        return Ok(html! {
            <div>
                <CardDiv flip={flip_card} mode={FlashCardMode::Study} card={card.clone()} />
                <div class="button-container">
                    <ActionButton enabled={has_previous} aria_label="Previous" onclick={prev_card} icon="\u{2B05}" />
                    <ActionButton enabled={has_next} aria_label="Next" onclick={next_card} icon="\u{27A1}" />
                </div>
            </div>
        })
    }

    Ok(html! {
        <div>
            <CardDiv flip={flip_card} mode={FlashCardMode::Study} card={card.clone()} />
            <div class="button-container">
                <ActionButton enabled={true} aria_label="Easy" onclick={review_card(CardDifficulty::Easy)} icon="\u{2730}" />
                <ActionButton enabled={true} aria_label="Medium" onclick={review_card(CardDifficulty::Medium)} icon="\u{272E}" />
                <ActionButton enabled={true} aria_label="Hard" onclick={review_card(CardDifficulty::Hard)} icon="\u{272A}" />
            </div>
        </div>
    })
    
}
