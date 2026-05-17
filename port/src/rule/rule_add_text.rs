use std::path::Path;

use crate::rule::rules::{split_file_name, RulePlace, RuleType, SingleRule};

pub fn rule_add_text(data_to_change: &str, rule: &SingleRule) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let return_string;
    let add_text_text = rule.rule_data.add_text_text.clone();

    match rule.rule_type {
        RuleType::AddText => match rule.rule_place {
            RulePlace::BeforeName => {
                if extension.is_empty() {
                    return_string = format!("{add_text_text}{name}");
                } else {
                    return_string = format!("{add_text_text}{name}.{extension}");
                }
            }
            RulePlace::AfterName => {
                if extension.is_empty() {
                    return_string = format!("{name}{add_text_text}");
                } else {
                    return_string = format!("{name}{add_text_text}.{extension}");
                }
            }
            _ => panic!("Invalid Rule Place for AddText"),
        },
        _ => panic!("Invalid Rule Type for AddText"),
    }

    return_string
}
