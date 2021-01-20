use crate::help_function::split_file_name;
use crate::rules::*;
use std::path::Path;

pub fn rule_trim(data_to_change: &str, rule_type: &RuleType, rule_place: &RulePlace, rule_case_sensitivity: &RuleCaseSensitivity, rule_data: &RuleData) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let mut return_string = data_to_change.to_string();

    let data_to_change_lowercase = data_to_change.to_lowercase();
    let name_lowercase = name.to_lowercase();
    let extension_lowercase = extension.to_lowercase();
    let text_to_trim = rule_data.trim_text.clone();
    let text_to_trim_lowerspace = text_to_trim.to_lowercase();

    match rule_type {
        RuleType::Trim => match rule_place {
            RulePlace::FromExtensionStart => {
                if RuleCaseSensitivity::Sensitive == *rule_case_sensitivity && extension.starts_with(&text_to_trim) {
                    if text_to_trim.len() == extension.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", name, extension[text_to_trim.len()..extension.len()].to_string());
                    }
                } else if RuleCaseSensitivity::Insensitive == *rule_case_sensitivity && extension_lowercase.starts_with(&text_to_trim_lowerspace) {
                    if text_to_trim_lowerspace.len() == extension_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", name, extension[text_to_trim_lowerspace.len()..extension_lowercase.len()].to_string());
                    }
                }
            }
            RulePlace::FromNameStart => {
                if RuleCaseSensitivity::Sensitive == *rule_case_sensitivity && data_to_change.starts_with(&text_to_trim) {
                    if text_to_trim.len() == data_to_change.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[text_to_trim.len()..data_to_change.len()].to_string();
                    }
                } else if RuleCaseSensitivity::Insensitive == *rule_case_sensitivity && data_to_change_lowercase.starts_with(&text_to_trim_lowerspace) {
                    if text_to_trim_lowerspace.len() == data_to_change_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[text_to_trim_lowerspace.len()..data_to_change_lowercase.len()].to_string();
                    }
                }
            }
            RulePlace::FromNameEndReverse => {
                if RuleCaseSensitivity::Sensitive == *rule_case_sensitivity && name.ends_with(&text_to_trim) {
                    if text_to_trim.len() == name.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", name[0..(name.len() - text_to_trim.len())].to_string(), extension);
                    }
                } else if RuleCaseSensitivity::Insensitive == *rule_case_sensitivity && name_lowercase.ends_with(&text_to_trim_lowerspace) {
                    if text_to_trim_lowerspace.len() == name_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", name[0..(name_lowercase.len() - text_to_trim_lowerspace.len())].to_string(), extension);
                    }
                }
            }
            RulePlace::FromExtensionEndReverse => {
                if RuleCaseSensitivity::Sensitive == *rule_case_sensitivity && data_to_change.ends_with(&text_to_trim) {
                    if text_to_trim.len() == data_to_change.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[0..(data_to_change.len() - text_to_trim.len())].to_string();
                    }
                } else if RuleCaseSensitivity::Insensitive == *rule_case_sensitivity && data_to_change_lowercase.ends_with(&text_to_trim_lowerspace) {
                    if text_to_trim_lowerspace.len() == data_to_change_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[0..(data_to_change_lowercase.len() - text_to_trim_lowerspace.len())].to_string();
                    }
                }
            }
            _ => {
                panic!("Not implemented function");
            }
        },
        _ => {
            panic!("Not implemented function");
        }
    }

    return_string
}

#[cfg(test)]
mod test {
    use crate::rule_trim::rule_trim;
    use crate::rules::{RuleCaseSensitivity, RuleData, RulePlace, RuleType};

    #[test]
    fn test_trim() {
        let mut rule_data: RuleData = RuleData::new();

        rule_data.trim_text = "Txt".to_string();
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &RuleCaseSensitivity::Insensitive, &rule_data), "Roman.");
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &RuleCaseSensitivity::Sensitive, &rule_data), "Roman.txt");
        assert_eq!(rule_trim("Roman.Txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &RuleCaseSensitivity::Insensitive, &rule_data), "Roman.");
        assert_eq!(rule_trim("Roman.Txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &RuleCaseSensitivity::Sensitive, &rule_data), "Roman.");
        assert_eq!(rule_trim("Roman.asb", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &RuleCaseSensitivity::Insensitive, &rule_data), "Roman.asb");
        assert_eq!(rule_trim("Roman.asb", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &RuleCaseSensitivity::Sensitive, &rule_data), "Roman.asb");
        assert_eq!(rule_trim("Txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &RuleCaseSensitivity::Sensitive, &rule_data), "");
        assert_eq!(rule_trim("txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &RuleCaseSensitivity::Insensitive, &rule_data), "");

        rule_data.trim_text = "R".to_string();
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &RuleCaseSensitivity::Insensitive, &rule_data), "oman.txt");
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &RuleCaseSensitivity::Sensitive, &rule_data), "oman.txt");
        assert_eq!(rule_trim("roman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &RuleCaseSensitivity::Insensitive, &rule_data), "oman.txt");
        assert_eq!(rule_trim("roman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &RuleCaseSensitivity::Sensitive, &rule_data), "roman.txt");
        assert_eq!(rule_trim("Koman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &RuleCaseSensitivity::Insensitive, &rule_data), "Koman.txt");
        assert_eq!(rule_trim("Koman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &RuleCaseSensitivity::Sensitive, &rule_data), "Koman.txt");
        assert_eq!(rule_trim("R", &RuleType::Trim, &RulePlace::FromNameStart, &RuleCaseSensitivity::Sensitive, &rule_data), "");
        assert_eq!(rule_trim("r", &RuleType::Trim, &RulePlace::FromNameStart, &RuleCaseSensitivity::Insensitive, &rule_data), "");

        rule_data.trim_text = "n".to_string();
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &RuleCaseSensitivity::Insensitive, &rule_data), "Roma.txt");
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &RuleCaseSensitivity::Sensitive, &rule_data), "Roma.txt");
        assert_eq!(rule_trim("RomaN.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &RuleCaseSensitivity::Insensitive, &rule_data), "Roma.txt");
        assert_eq!(rule_trim("RomaN.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &RuleCaseSensitivity::Sensitive, &rule_data), "RomaN.txt");
        assert_eq!(rule_trim("RomaZ.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &RuleCaseSensitivity::Insensitive, &rule_data), "RomaZ.txt");
        assert_eq!(rule_trim("RomaZ.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &RuleCaseSensitivity::Sensitive, &rule_data), "RomaZ.txt");
        assert_eq!(rule_trim("n", &RuleType::Trim, &RulePlace::FromNameEndReverse, &RuleCaseSensitivity::Sensitive, &rule_data), "");
        assert_eq!(rule_trim("N", &RuleType::Trim, &RulePlace::FromNameEndReverse, &RuleCaseSensitivity::Insensitive, &rule_data), "");

        rule_data.trim_text = "t".to_string();
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionStart, &RuleCaseSensitivity::Insensitive, &rule_data), "Roman.xt");
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionStart, &RuleCaseSensitivity::Sensitive, &rule_data), "Roman.xt");
        assert_eq!(rule_trim("Roman.Txt", &RuleType::Trim, &RulePlace::FromExtensionStart, &RuleCaseSensitivity::Insensitive, &rule_data), "Roman.xt");
        assert_eq!(rule_trim("Roman.Txt", &RuleType::Trim, &RulePlace::FromExtensionStart, &RuleCaseSensitivity::Sensitive, &rule_data), "Roman.Txt");
        assert_eq!(rule_trim("Roman.Zxt", &RuleType::Trim, &RulePlace::FromExtensionStart, &RuleCaseSensitivity::Insensitive, &rule_data), "Roman.Zxt");
        assert_eq!(rule_trim("Roman.Zxt", &RuleType::Trim, &RulePlace::FromExtensionStart, &RuleCaseSensitivity::Sensitive, &rule_data), "Roman.Zxt");
        assert_eq!(rule_trim("t", &RuleType::Trim, &RulePlace::FromExtensionStart, &RuleCaseSensitivity::Sensitive, &rule_data), "t");
        assert_eq!(rule_trim("T", &RuleType::Trim, &RulePlace::FromExtensionStart, &RuleCaseSensitivity::Insensitive, &rule_data), "T");
    }
}
