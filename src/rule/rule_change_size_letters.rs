use std::path::Path;

use crate::help_function::split_file_name;
use crate::rule::rules::*;

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
                    _ => {
                        panic!("Not implemented function");
                    }
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
                    _ => {
                        panic!("Not implemented function");
                    }
                }
            }
        }
        _ => {
            panic!("Invalid Rule type");
        }
    }

    // Handle also situation when e.g. file is "file." where there is not extension, but
    if !extension.is_empty() || data_to_change.contains('.') {
        format!("{name}.{extension}")
    } else {
        name
    }
}

#[cfg(test)]
mod test {
    use crate::rule::rule_change_size_letters::rule_change_size_letters;
    use crate::rule::rules::{RulePlace, RuleType, SingleRule};

    #[test]
    fn test_size_letters() {
        let mut rule = SingleRule::new();
        rule.rule_type = RuleType::CaseSize;

        rule.rule_data.to_lowercase = true;

        rule.rule_place = RulePlace::Name;
        assert_eq!(rule_change_size_letters("Roman.Txt", &rule), "roman.Txt");
        rule.rule_place = RulePlace::Extension;
        assert_eq!(rule_change_size_letters("Roman.Txt", &rule), "Roman.txt");
        rule.rule_place = RulePlace::ExtensionAndName;
        assert_eq!(rule_change_size_letters("Roman.Txt", &rule), "roman.txt");

        rule.rule_data.to_lowercase = false;

        rule.rule_place = RulePlace::Name;
        assert_eq!(rule_change_size_letters("Roman.Txt", &rule), "ROMAN.Txt");
        rule.rule_place = RulePlace::Extension;
        assert_eq!(rule_change_size_letters("Roman.Txt", &rule), "Roman.TXT");
        rule.rule_place = RulePlace::ExtensionAndName;
        assert_eq!(rule_change_size_letters("Roman.Txt", &rule), "ROMAN.TXT");
    }
}
