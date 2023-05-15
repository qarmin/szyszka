use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::Builder;

use crate::gui_data_things::class_dialog_rules::GuiDialogRules;
use crate::gui_data_things::gui_data_results::GuiResults;
use crate::gui_data_things::gui_data_rules_bottom_panel::GuiRulesBottomPanel;
use crate::gui_data_things::gui_data_settings::GuiSettings;
use crate::gui_data_things::gui_data_upper_buttons::GuiUpperButtons;
use crate::gui_data_things::gui_popover_select::GuiPopoverSelect;
use crate::help_function::ResultEntries;
use crate::rule::rules::Rules;

#[derive(Clone)]
pub struct GuiData {
    // Glade builder
    // pub glade_src: String,
    // pub builder: Builder,

    // Window
    pub window_main: gtk4::Window,

    // Subcategories
    pub upper_buttons: GuiUpperButtons,
    pub results: GuiResults,
    pub rules_bottom_panel: GuiRulesBottomPanel,
    pub popover_select: GuiPopoverSelect,
    pub settings: GuiSettings,

    pub window_rules: GuiDialogRules,

    pub rules: Rc<RefCell<Rules>>,

    pub shared_result_entries: Rc<RefCell<ResultEntries>>,
}

impl GuiData {
    pub fn new_with_application(application: &gtk4::Application) -> Self {
        //// Loading glade file content and build with it help UI
        let window_main_src = include_str!("../../ui/window_main.ui").to_string();
        let builder_window_main = Builder::from_string(window_main_src.as_str());

        let settings_src = include_str!("../../ui/settings.ui").to_string();
        let builder_settings = Builder::from_string(settings_src.as_str());

        let popover_src = include_str!("../../ui/popover.ui").to_string();
        let builder_popover = Builder::from_string(popover_src.as_str());

        let rule_chooser_src = include_str!("../../ui/rule_chooser.ui").to_string();
        let builder_rule_chooser = Builder::from_string(rule_chooser_src.as_str());

        //// Windows
        let window_main: gtk4::Window = builder_window_main.object("window_main").unwrap();
        window_main.set_application(Some(application));

        window_main.show();
        window_main.set_title(Some("Szyszka"));

        let upper_buttons = GuiUpperButtons::create_from_builder(&builder_window_main);
        let results = GuiResults::create_from_builder(&builder_window_main);
        let rules_bottom_panel = GuiRulesBottomPanel::create_from_builder(&builder_window_main);
        let popover_select = GuiPopoverSelect::create_from_builder(&builder_popover);
        let settings = GuiSettings::create_from_builder(&builder_settings, &window_main);

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
    pub fn update_language(&self) {
        self.upper_buttons.update_language();
        self.results.update_language();
        self.rules_bottom_panel.update_language();
        // self.popover_select.update_language();
        // self.settings.update_language();
        // self.window_rules.update_language();

        // self.check_button_music_title.set_label(&fls!("music_title_checkbox"));
        // self.check_button_music_artist.set_label(&fls!("music_artist_checkbox"));
        // self.check_button_music_album_title.set_label(&fls!("music_album_title_checkbox"));
        // self.check_button_music_album_artist.set_label(&fls!("music_album_artist_checkbox"));
        // self.check_button_music_year.set_label(&fls!("music_year_checkbox"));
        // self.check_button_music_approximate_comparison.set_label(&fls!("music_comparison_checkbox"));
        //
        // self.check_button_music_approximate_comparison
        //     .set_tooltip_text(Some(&fls!("music_comparison_checkbox_tooltip")));
    }
}
