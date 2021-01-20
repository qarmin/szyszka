use crate::help_function::split_file_name;
use crate::rules::*;
use std::path::Path;

pub fn rule_purge(data_to_change: &str, rule_type: &RuleType, rule_place: &RulePlace) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let return_string;

    match rule_type {
        RuleType::Purge => match rule_place {
            RulePlace::Name => {
                return_string = extension;
            }
            RulePlace::ExtensionAndName => {
                return_string = "".to_string();
            }
            RulePlace::Extension => {
                return_string = name;
            }
            _ => {
                panic!("Not implemented function");
            }
        },
        _ => panic!("Invalid Rule Type for purge rule"),
    }

    return_string
}

#[cfg(test)]
mod test {
    use crate::rule_purge::rule_purge;
    use crate::rules::{RulePlace, RuleType};

    #[test]
    fn test_purge() {
        assert_eq!(rule_purge("Roman.txt", &RuleType::Purge, &RulePlace::Name), "txt");
        assert_eq!(rule_purge("Roman.txt", &RuleType::Purge, &RulePlace::Extension), "Roman");
        assert_eq!(rule_purge("Roman.txt", &RuleType::Purge, &RulePlace::ExtensionAndName), "");
    }
}
