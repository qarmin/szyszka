use std::path::Path;

use crate::rule::rules::{split_file_name, RulePlace, RuleType, SingleRule};

pub fn rule_change_size_letters(data_to_change: &str, rule: &SingleRule) -> String {
    let (mut name, mut extension) = split_file_name(Path::new(data_to_change));

    match rule.rule_type {
        RuleType::CaseSize => {
            if !rule.rule_data.to_lowercase {
                match rule.rule_place {
                    RulePlace::Name => {
                        name = name.to_uppercase();
                    }
                    RulePlace::ExtensionAndName => {
                        name = name.to_uppercase();
                        extension = extension.to_uppercase();
                    }
                    RulePlace::Extension => {
                        extension = extension.to_uppercase();
                    }
                    _ => panic!("Not implemented function"),
                }
            } else {
                match rule.rule_place {
                    RulePlace::Name => {
                        name = name.to_lowercase();
                    }
                    RulePlace::ExtensionAndName => {
                        name = name.to_lowercase();
                        extension = extension.to_lowercase();
                    }
                    RulePlace::Extension => {
                        extension = extension.to_lowercase();
                    }
                    _ => panic!("Not implemented function"),
                }
            }
        }
        _ => panic!("Invalid Rule type"),
    }

    if !extension.is_empty() || data_to_change.contains('.') {
        format!("{name}.{extension}")
    } else {
        name
    }
}
