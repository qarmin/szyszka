use crate::help_function::split_file_name;
use crate::rules::*;
use std::path::Path;

pub fn rule_change_size_letters(data_to_change: &str, rule_type: &RuleType, rule_place: &RulePlace, rule_data: &RuleData) -> String {
    let (mut name, mut extension) = split_file_name(Path::new(data_to_change));

    match rule_type {
        RuleType::CaseSize => {
            if !rule_data.to_lowercase {
                match rule_place {
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
                match rule_place {
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
        format!("{}.{}", name, extension)
    } else {
        name
    }
}

#[cfg(test)]
mod test {
    use crate::rule_change_size_letters::rule_change_size_letters;
    use crate::rules::{RuleData, RulePlace, RuleType};

    #[test]
    fn test_size_letters() {
        let mut rule_data = RuleData::new();
        rule_data.to_lowercase = true;
        assert_eq!(rule_change_size_letters("Roman.Txt", &RuleType::CaseSize, &RulePlace::Name, &rule_data), "roman.Txt");
        assert_eq!(rule_change_size_letters("Roman.Txt", &RuleType::CaseSize, &RulePlace::Extension, &rule_data), "Roman.txt");
        assert_eq!(rule_change_size_letters("Roman.Txt", &RuleType::CaseSize, &RulePlace::ExtensionAndName, &rule_data), "roman.txt");

        rule_data.to_lowercase = false;
        assert_eq!(rule_change_size_letters("Roman.Txt", &RuleType::CaseSize, &RulePlace::Name, &rule_data), "ROMAN.Txt");
        assert_eq!(rule_change_size_letters("Roman.Txt", &RuleType::CaseSize, &RulePlace::Extension, &rule_data), "Roman.TXT");
        assert_eq!(rule_change_size_letters("Roman.Txt", &RuleType::CaseSize, &RulePlace::ExtensionAndName, &rule_data), "ROMAN.TXT");
    }
}
