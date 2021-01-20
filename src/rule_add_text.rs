use crate::help_function::split_file_name;
use crate::rules::*;
use std::path::Path;

pub fn rule_add_text(data_to_change: &str, rule_type: &RuleType, rule_place: &RulePlace, rule_data: &RuleData) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let return_string;
    let add_text_text = rule_data.add_text_text.clone();

    match rule_type {
        RuleType::AddText => match rule_place {
            RulePlace::BeforeName => {
                if extension.is_empty() {
                    return_string = format!("{}{}", add_text_text, name);
                } else {
                    return_string = format!("{}{}.{}", add_text_text, name, extension);
                }
            }
            RulePlace::AfterName => {
                if extension.is_empty() {
                    return_string = format!("{}{}", name, add_text_text);
                } else {
                    return_string = format!("{}{}.{}", name, add_text_text, extension);
                }
            }
            _ => panic!("Invalid Rule Place for AddText"),
        },
        _ => panic!("Invalid Rule Type for AddText"),
    }

    return_string
}

#[cfg(test)]
mod test {
    use crate::rule_add_text::rule_add_text;
    use crate::rules::{RuleData, RulePlace, RuleType};

    #[test]
    fn test_add_text() {
        let mut rule_data: RuleData;

        rule_data = RuleData::new();
        rule_data.add_text_text = "Qwark".to_string();
        assert_eq!(rule_add_text("wombat.txt", &RuleType::AddText, &RulePlace::BeforeName, &rule_data), "Qwarkwombat.txt");

        rule_data = RuleData::new();
        rule_data.add_text_text = "Qwark".to_string();
        assert_eq!(rule_add_text("wombat.txt", &RuleType::AddText, &RulePlace::AfterName, &rule_data), "wombatQwark.txt");
    }
}
