use crate::class_dialog_rules::GuiDialogRules;
use crate::class_results::GuiResults;
use crate::class_rules_bottom_panel::GuiRulesBottomPanel;
use crate::class_upper_buttons::*;
use crate::file_entry::ResultEntries;
use crate::rules::Rules;
use gtk::prelude::*;
use gtk::{Builder, WindowPosition};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct GuiData {
    // Glade builder
    // pub glade_src: String,
    // pub builder: Builder,

    // Window
    pub window_main: gtk::Window,

    // Subcategories
    pub upper_buttons: GuiUpperButtons,
    pub results: GuiResults,
    pub rules_bottom_panel: GuiRulesBottomPanel,

    pub window_rules: GuiDialogRules,

    pub rules: Rc<RefCell<Rules>>,

    //
    pub shared_result_entries: Rc<RefCell<ResultEntries>>,
}

impl GuiData {
    pub fn new() -> Self {
        //// Loading glade file content and build with it help UI
        let glade_src = include_str!("../szyszka.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        //// Windows
        let window_main: gtk::Window = builder.object("window_main").unwrap();
        window_main.set_position(WindowPosition::Center);
        window_main.show_all();
        window_main.set_title("Szyszka");

        let upper_buttons: GuiUpperButtons = GuiUpperButtons::create_from_builder(&builder);
        let results: GuiResults = GuiResults::create_from_builder(&builder);
        let rules_bottom_panel: GuiRulesBottomPanel = GuiRulesBottomPanel::create_from_builder(&builder);

        let window_rules: GuiDialogRules = GuiDialogRules::create_from_builder(&builder);

        let rules = Rc::new(RefCell::new(Rules::new()));

        let shared_result_entries = Rc::new(RefCell::new(ResultEntries { files: Default::default() }));

        Self {
            window_main,
            upper_buttons,
            results,
            rules_bottom_panel,
            window_rules,
            rules,
            shared_result_entries,
        }
    }
}
