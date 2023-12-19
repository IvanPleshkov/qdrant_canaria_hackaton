use crate::action::Action;

pub struct ActionsCollection {
    pub actions: Vec<Action>,
}

impl ActionsCollection {
    pub fn new() -> Self {
        Self { actions: vec![] }
    }
}
