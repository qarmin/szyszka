use std::cell::RefCell;
use std::rc::Rc;

use crate::files::{ItemStruct, ResultEntries};
use crate::rule::rules::Rules;

#[derive(Default)]
pub struct AppState {
    pub files: Vec<ItemStruct>,
    pub file_selected: Vec<bool>,
    pub rules: Rules,
    pub rule_selected: Vec<bool>,
    pub result_entries: ResultEntries,
    pub edit_index: Option<usize>,
    pub active_timer: Option<slint::Timer>,
}

pub type SharedState = Rc<RefCell<AppState>>;

pub fn new_shared() -> SharedState {
    Rc::new(RefCell::new(AppState::default()))
}
