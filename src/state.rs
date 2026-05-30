use std::cell::RefCell;
use std::path::PathBuf;
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
    pub pending_folders: Vec<PathBuf>,
    /// Formatted error lines from the last rename, kept in full so the results
    /// dialog can page through them and copy them all at once.
    pub failed_renames: Vec<String>,
}

pub type SharedState = Rc<RefCell<AppState>>;

pub fn new_shared() -> SharedState {
    Rc::new(RefCell::new(AppState::default()))
}
