use crate::gui_connection::common::{connect_examples_check_button, connect_examples_entry_name};
use crate::gui_data_things::gui_data::GuiData;
use gtk4::prelude::*;
use gtk4::{Entry, Label};
use regex::Regex;

pub fn connect_rule_window_replace_click(gui_data: &GuiData) {
    let check_button_replace_name = gui_data.window_rules.replace.check_button_replace_name.clone();
    let check_button_replace_extension = gui_data.window_rules.replace.check_button_replace_extension.clone();
    let check_button_replace_both = gui_data.window_rules.replace.check_button_replace_both.clone();

    let check_button_replace_case_insensitive = gui_data.window_rules.replace.check_button_replace_case_insensitive.clone();
    let check_button_replace_case_sensitive = gui_data.window_rules.replace.check_button_replace_case_sensitive.clone();

    let check_button_replace_regex = gui_data.window_rules.replace.check_button_replace_regex.clone();
    let check_button_replace_replace_all = gui_data.window_rules.replace.check_button_replace_replace_all.clone();
    let label_replace_captures = gui_data.window_rules.replace.label_replace_captures.clone();
    let label_replace_captured_captures = gui_data.window_rules.replace.label_replace_captured_captures.clone();

    let entry_replace_text_to_change = gui_data.window_rules.replace.entry_replace_text_to_change.clone();
    let entry_replace_text_to_find = gui_data.window_rules.replace.entry_replace_text_to_find.clone();

    let entry_example_before = gui_data.window_rules.entry_example_before.clone();

    connect_examples_check_button(&check_button_replace_name, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_extension, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_both, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_case_sensitive, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_case_insensitive, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_regex, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_replace_all, &gui_data.window_rules);

    connect_examples_entry_name(&entry_replace_text_to_find, &gui_data.window_rules);
    connect_examples_entry_name(&entry_replace_text_to_change, &gui_data.window_rules);

    check_button_replace_regex.connect_toggled(move |e| {
        let active = e.is_active();
        check_button_replace_case_sensitive.set_sensitive(!active);
        check_button_replace_case_insensitive.set_sensitive(!active);
        check_button_replace_name.set_sensitive(!active);
        check_button_replace_both.set_sensitive(!active);
        check_button_replace_extension.set_sensitive(!active);

        check_button_replace_replace_all.set_sensitive(active);
        label_replace_captures.set_visible(active);
        label_replace_captured_captures.set_visible(active);

        if active {
            recalculate_regex_captures(&label_replace_captured_captures, &entry_replace_text_to_find, &entry_example_before);
        }
    });

    let check_button_replace_regex = gui_data.window_rules.replace.check_button_replace_regex.clone();
    let label_replace_captured_captures = gui_data.window_rules.replace.label_replace_captured_captures.clone();
    let entry_example_before = gui_data.window_rules.entry_example_before.clone();
    let entry_replace_text_to_find = gui_data.window_rules.replace.entry_replace_text_to_find.clone();
    entry_replace_text_to_find.connect_changed(move |entry_replace_text_to_find| {
        if check_button_replace_regex.is_active() {
            recalculate_regex_captures(&label_replace_captured_captures, entry_replace_text_to_find, &entry_example_before);
        }
    });

    let check_button_replace_regex = gui_data.window_rules.replace.check_button_replace_regex.clone();
    let label_replace_captured_captures = gui_data.window_rules.replace.label_replace_captured_captures.clone();
    let entry_example_before = gui_data.window_rules.entry_example_before.clone();
    let entry_replace_text_to_find = gui_data.window_rules.replace.entry_replace_text_to_find.clone();
    entry_example_before.connect_changed(move |entry_example_before| {
        if check_button_replace_regex.is_active() {
            recalculate_regex_captures(&label_replace_captured_captures, &entry_replace_text_to_find, entry_example_before);
        }
    });
}

fn recalculate_regex_captures(label_replace_captured_captures: &Label, entry_replace_text_to_find: &Entry, entry_example_before: &Entry) {
    if entry_replace_text_to_find.text().len() == 0 {
        label_replace_captured_captures.set_label("No captures");
        return;
    }

    let Ok(regex) = Regex::new(entry_replace_text_to_find.text().as_str()) else {
        label_replace_captured_captures.set_label("INVALID REGEX");
        return;
    };

    let text_before = entry_example_before.text();
    let captures = regex.captures(&text_before);
    match captures {
        Some(captures) => {
            let mut items = Vec::new();
            items.push(format!("({} captures) - ", captures.len()));
            for (idx, capture) in captures.iter().enumerate() {
                // TODO check logic, why match can be None?
                let mat = match capture {
                    Some(capture) => capture.as_str().to_string(),
                    None => String::new(),
                };
                items.push(format!("{idx}: {mat}, "));
            }

            let mut txt = String::new();
            let mut cumulated_chars = 0;
            for i in items {
                if cumulated_chars > 80 {
                    txt.push('\n');
                    cumulated_chars = 0;
                }
                txt.push_str(i.as_str());
                cumulated_chars += i.len();
            }

            label_replace_captured_captures.set_label(&txt);
        }
        None => {
            label_replace_captured_captures.set_label("No captures");
        }
    };
}
