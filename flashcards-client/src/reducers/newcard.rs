use std::rc::Rc;
use yew::Reducible;

pub struct NewCardState {
    pub front: Rc<String>,
    pub back: Rc<String>,
}

impl NewCardState {
    pub fn new() -> Self {
        Self {
            front: Rc::new(String::new()),
            back: Rc::new(String::new()),
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
                let mut front: String = (*self.front).clone();
                let back: String = (*self.back).clone();

                front = String::from(new_front);

               NewCardState {
                    front: Rc::new(front),
                    back: Rc::new(back),
                }.into()
            }
            NewCardAction::SetBack(new_back) => {
                let front: String = (*self.front).clone();
                let mut back: String = (*self.back).clone();

                back = String::from(new_back);

                NewCardState {
                    front: Rc::new(front),
                    back: Rc::new(back),
                }.into()
            }
            NewCardAction::ResetCard => {
                NewCardState {
                    front: Rc::new(String::new()),
                    back: Rc::new(String::new()),
                }.into()
            }
        }
    }
}
