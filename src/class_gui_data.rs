use crate::class_dialog_rules::GuiDialogRules;
use crate::class_popover_select::GuiPopoverSelect;
use crate::class_results::GuiResults;
use crate::class_rules_bottom_panel::GuiRulesBottomPanel;
use crate::class_settings::GuiSettings;
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
    pub popover_select: GuiPopoverSelect,
    pub settings: GuiSettings,

    pub window_rules: GuiDialogRules,

    pub rules: Rc<RefCell<Rules>>,

    //
    pub shared_result_entries: Rc<RefCell<ResultEntries>>,
}

impl GuiData {
    pub fn new() -> Self {
        //// Loading glade file content and build with it help UI
        let window_main_src = include_str!("../ui/window_main.ui").to_string();
        let builder_window_main = Builder::from_string(window_main_src.as_str());

        let settings_src = include_str!("../ui/settings.ui").to_string();
        let builder_settings = Builder::from_string(settings_src.as_str());

        let popover_src = include_str!("../ui/popover.ui").to_string();
        let builder_popover = Builder::from_string(popover_src.as_str());

        let rule_chooser_src = include_str!("../ui/rule_chooser.ui").to_string();
        let builder_rule_chooser = Builder::from_string(rule_chooser_src.as_str());

        //// Windows
        let window_main: gtk::Window = builder_window_main.object("window_main").unwrap();
        window_main.set_position(WindowPosition::Center);
        window_main.show_all();
        window_main.set_title("Szyszka");

        let upper_buttons = GuiUpperButtons::create_from_builder(&builder_window_main);
        let results = GuiResults::create_from_builder(&builder_window_main);
        let rules_bottom_panel = GuiRulesBottomPanel::create_from_builder(&builder_window_main);
        let popover_select = GuiPopoverSelect::create_from_builder(&builder_popover);
        let settings = GuiSettings::create_from_builder(&builder_settings);

        let window_rules = GuiDialogRules::create_from_builder(&builder_rule_chooser);

        let rules = Rc::new(RefCell::new(Rules::new()));

        let shared_result_entries = Rc::new(RefCell::new(ResultEntries { files: Default::default() }));

        Self {
            window_main,
            upper_buttons,
            results,
            rules_bottom_panel,
            popover_select,
            settings,
            window_rules,
            rules,
            shared_result_entries,
        }
    }
}
