use std::rc::Rc;
use yew::Reducible;
use flashcards_data::Card;

pub struct NewCardState {
    pub card: Rc<Card>,
}

impl NewCardState {
    pub fn new() -> Self {
        Self {
            card: Rc::new(Card::new(0, String::new(), String::new())),
        }
    }
}

pub enum NewCardAction {
    SetFront(String),
    SetBack(String),
    ResetCard,
}

impl Reducible for NewCardState {
    type Action = NewCardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            NewCardAction::SetFront(new_front) => {
                
                let card: Card = (*self.card).clone();

                NewCardState {
                    card: Rc::new(card),
                }.into()
            }
            NewCardAction::SetBack(new_back) => {

                let card: Card = (*self.card).clone();

                NewCardState {
                    card: Rc::new(card),
                }.into()
            }
            NewCardAction::ResetCard => {
                NewCardState {
                    card: Rc::new(Card::new(0, String::new(), String::new())),
                }.into()
            }
        }
    }
}
