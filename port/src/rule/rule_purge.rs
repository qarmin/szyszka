use std::path::Path;

use crate::rule::rules::{split_file_name, RulePlace, RuleType, SingleRule};

pub fn rule_purge(data_to_change: &str, rule: &SingleRule) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let return_string;

    match rule.rule_type {
        RuleType::Purge => match rule.rule_place {
            RulePlace::Name => {
                return_string = extension;
            }
            RulePlace::ExtensionAndName => {
                return_string = String::new();
            }
            RulePlace::Extension => {
                return_string = name;
            }
            _ => panic!("Not implemented function"),
        },
        _ => panic!("Invalid Rule Type for purge rule"),
    }

    return_string
}
