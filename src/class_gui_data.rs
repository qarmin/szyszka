use crate::class_dialog_rules::DialogRules;
use crate::class_results::Results;
use crate::class_rules_bottom_panel::RulesBottomPanel;
use crate::class_status::Status;
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
    pub upper_buttons: UpperButtons,
    pub results: Results,
    pub status: Status,
    pub rules_bottom_panel: RulesBottomPanel,

    pub dialog_rules: DialogRules,

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

        let upper_buttons: UpperButtons = UpperButtons::create_from_builder(&builder);
        let results: Results = Results::create_from_builder(&builder);
        let status: Status = Status::create_from_builder(&builder);
        let rules_bottom_panel: RulesBottomPanel = RulesBottomPanel::create_from_builder(&builder);

        let dialog_rules: DialogRules = DialogRules::create_from_builder(&builder);

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
