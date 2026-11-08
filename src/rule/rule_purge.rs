use std::path::Path;

use crate::rule::rules::{split_file_name, RulePlace, RuleType, SingleRule};

pub fn rule_purge(data_to_change: &str, rule: &SingleRule) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let return_string;

    match rule.rule_type {
        RuleType::Purge => match rule.rule_place {
            RulePlace::Name => {
                return_string = if extension.is_empty() {
                    if data_to_change.ends_with('.') {
                        ".".to_string()
                    } else {
                        String::new()
                    }
                } else {
                    format!(".{extension}")
                };
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

#[cfg(test)]
mod test {
    use crate::rule::rule_purge::rule_purge;
    use crate::rule::rules::{RulePlace, RuleType, SingleRule};

    #[test]
    fn test_purge() {
        let mut rule = SingleRule::new();

        rule.rule_type = RuleType::Purge;

        rule.rule_place = RulePlace::Name;
        assert_eq!(rule_purge("Roman.txt", &rule), ".txt");
        assert_eq!(rule_purge("Roman", &rule), "");
        rule.rule_place = RulePlace::Extension;
        assert_eq!(rule_purge("Roman.txt", &rule), "Roman");
        rule.rule_place = RulePlace::ExtensionAndName;
        assert_eq!(rule_purge("Roman.txt", &rule), "");
    }
}
