use crate::class_dialog_rules::GUIDialogRules;
use crate::class_results::GUIResults;
use crate::class_rules_bottom_panel::GUIRulesBottomPanel;
use crate::class_status::GUIStatus;
use crate::class_upper_buttons::*;
use crate::file_entry::ResultEntries;
use crate::rules::Rules;
use gtk::prelude::*;
use gtk::Builder;
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
    pub upper_buttons: GUIUpperButtons,
    pub results: GUIResults,
    pub status: GUIStatus,
    pub rules_bottom_panel: GUIRulesBottomPanel,

    pub dialog_rules: GUIDialogRules,

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
        let window_main: gtk::Window = builder.get_object("window_main").unwrap();
        window_main.show_all();
        window_main.set_title("Szyszka");

        let upper_buttons: GUIUpperButtons = GUIUpperButtons::create_from_builder(&builder);
        let results: GUIResults = GUIResults::create_from_builder(&builder);
        let status: GUIStatus = GUIStatus::create_from_builder(&builder);
        let rules_bottom_panel: GUIRulesBottomPanel = GUIRulesBottomPanel::create_from_builder(&builder);

        let dialog_rules: GUIDialogRules = GUIDialogRules::create_from_builder(&builder);

        let rules = Rc::new(RefCell::new(Rules::new()));

        let shared_result_entries = Rc::new(RefCell::new(ResultEntries { entries: vec![] }));

        Self {
            // glade_src,
            // builder,
            upper_buttons,
            results,
            status,
            rules_bottom_panel,
            dialog_rules,
            window_main,
            rules,
            shared_result_entries,
        }
    }
}
