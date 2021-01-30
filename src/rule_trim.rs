use crate::help_function::split_file_name;
use crate::rules::*;
use std::path::Path;

pub fn rule_trim(data_to_change: &str, rule_type: &RuleType, rule_place: &RulePlace, rule_data: &RuleData) -> String {
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
                if rule_data.case_sensitive && extension.starts_with(&text_to_trim) {
                    if text_to_trim.len() == extension.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", name, extension[text_to_trim.len()..extension.len()].to_string());
                    }
                } else if !rule_data.case_sensitive && extension_lowercase.starts_with(&text_to_trim_lowerspace) {
                    if text_to_trim_lowerspace.len() == extension_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", name, extension[text_to_trim_lowerspace.len()..extension_lowercase.len()].to_string());
                    }
                }
            }
            RulePlace::FromNameStart => {
                if rule_data.case_sensitive && data_to_change.starts_with(&text_to_trim) {
                    if text_to_trim.len() == data_to_change.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[text_to_trim.len()..data_to_change.len()].to_string();
                    }
                } else if !rule_data.case_sensitive && data_to_change_lowercase.starts_with(&text_to_trim_lowerspace) {
                    if text_to_trim_lowerspace.len() == data_to_change_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[text_to_trim_lowerspace.len()..data_to_change_lowercase.len()].to_string();
                    }
                }
            }
            RulePlace::FromNameEndReverse => {
                if rule_data.case_sensitive && name.ends_with(&text_to_trim) {
                    if text_to_trim.len() == name.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", name[0..(name.len() - text_to_trim.len())].to_string(), extension);
                    }
                } else if !rule_data.case_sensitive && name_lowercase.ends_with(&text_to_trim_lowerspace) {
                    if text_to_trim_lowerspace.len() == name_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", name[0..(name_lowercase.len() - text_to_trim_lowerspace.len())].to_string(), extension);
                    }
                }
            }
            RulePlace::FromExtensionEndReverse => {
                if rule_data.case_sensitive && data_to_change.ends_with(&text_to_trim) {
                    if text_to_trim.len() == data_to_change.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[0..(data_to_change.len() - text_to_trim.len())].to_string();
                    }
                } else if !rule_data.case_sensitive && data_to_change_lowercase.ends_with(&text_to_trim_lowerspace) {
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
    use crate::rules::{RuleData, RulePlace, RuleType};

    #[test]
    fn test_trim() {
        let mut rule_data: RuleData = RuleData::new();

        rule_data.trim_text = "Txt".to_string();
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data), "Roman.");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data), "Roman.txt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.Txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data), "Roman.");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.Txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data), "Roman.");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.asb", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data), "Roman.asb");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.asb", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data), "Roman.asb");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data), "");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data), "");

        rule_data.trim_text = "R".to_string();
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &rule_data), "oman.txt");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &rule_data), "oman.txt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("roman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &rule_data), "oman.txt");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("roman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &rule_data), "roman.txt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Koman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &rule_data), "Koman.txt");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Koman.txt", &RuleType::Trim, &RulePlace::FromNameStart, &rule_data), "Koman.txt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("r", &RuleType::Trim, &RulePlace::FromNameStart, &rule_data), "");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("R", &RuleType::Trim, &RulePlace::FromNameStart, &rule_data), "");

        rule_data.trim_text = "n".to_string();
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &rule_data), "Roma.txt");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &rule_data), "Roma.txt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("RomaN.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &rule_data), "Roma.txt");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("RomaN.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &rule_data), "RomaN.txt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("RomaZ.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &rule_data), "RomaZ.txt");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("RomaZ.txt", &RuleType::Trim, &RulePlace::FromNameEndReverse, &rule_data), "RomaZ.txt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("N", &RuleType::Trim, &RulePlace::FromNameEndReverse, &rule_data), "");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("n", &RuleType::Trim, &RulePlace::FromNameEndReverse, &rule_data), "");

        rule_data.trim_text = "t".to_string();
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionStart, &rule_data), "Roman.xt");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionStart, &rule_data), "Roman.xt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.Txt", &RuleType::Trim, &RulePlace::FromExtensionStart, &rule_data), "Roman.xt");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.Txt", &RuleType::Trim, &RulePlace::FromExtensionStart, &rule_data), "Roman.Txt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.Zxt", &RuleType::Trim, &RulePlace::FromExtensionStart, &rule_data), "Roman.Zxt");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.Zxt", &RuleType::Trim, &RulePlace::FromExtensionStart, &rule_data), "Roman.Zxt");
        rule_data.case_sensitive = false;
        assert_eq!(rule_trim("T", &RuleType::Trim, &RulePlace::FromExtensionStart, &rule_data), "T");
        rule_data.case_sensitive = true;
        assert_eq!(rule_trim("t", &RuleType::Trim, &RulePlace::FromExtensionStart, &rule_data), "t");
    }
}
