use crate::help_function::split_file_name;
use crate::rule::rules::*;
use std::path::Path;

pub fn rule_trim(data_to_change: &str, rule: &SingleRule) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let mut return_string = data_to_change.to_string();

    let data_to_change_lowercase = data_to_change.to_lowercase();
    let name_lowercase = name.to_lowercase();
    let extension_lowercase = extension.to_lowercase();
    let text_to_trim = rule.rule_data.trim_text.clone();
    let text_to_trim_lowercase = text_to_trim.to_lowercase();

    match rule.rule_type {
        RuleType::Trim => match rule.rule_place {
            RulePlace::FromExtensionStart => {
                if rule.rule_data.case_sensitive && extension.starts_with(&text_to_trim) {
                    if text_to_trim.len() == extension.len() {
                        return_string = format!("{}.", name);
                    } else {
                        return_string = format!("{}.{}", name, &extension[text_to_trim.len()..extension.len()]);
                    }
                } else if !rule.rule_data.case_sensitive && extension_lowercase.starts_with(&text_to_trim_lowercase) {
                    if text_to_trim_lowercase.len() == extension_lowercase.len() {
                        return_string = format!("{}.", name);
                    } else {
                        return_string = format!("{}.{}", name, &extension[text_to_trim_lowercase.len()..extension_lowercase.len()]);
                    }
                }
            }
            RulePlace::FromNameStart => {
                if rule.rule_data.case_sensitive && data_to_change.starts_with(&text_to_trim) {
                    if text_to_trim.len() == data_to_change.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[text_to_trim.len()..data_to_change.len()].to_string();
                    }
                } else if !rule.rule_data.case_sensitive && data_to_change_lowercase.starts_with(&text_to_trim_lowercase) {
                    if text_to_trim_lowercase.len() == data_to_change_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[text_to_trim_lowercase.len()..data_to_change_lowercase.len()].to_string();
                    }
                }
            }
            RulePlace::FromNameEndReverse => {
                if rule.rule_data.case_sensitive && name.ends_with(&text_to_trim) {
                    if text_to_trim.len() == name.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", &name[0..(name.len() - text_to_trim.len())], extension);
                    }
                } else if !rule.rule_data.case_sensitive && name_lowercase.ends_with(&text_to_trim_lowercase) {
                    if text_to_trim_lowercase.len() == name_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = format!("{}.{}", &name[0..(name_lowercase.len() - text_to_trim_lowercase.len())], extension);
                    }
                }
            }
            RulePlace::FromExtensionEndReverse => {
                if rule.rule_data.case_sensitive && data_to_change.ends_with(&text_to_trim) {
                    if text_to_trim.len() == data_to_change.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[0..(data_to_change.len() - text_to_trim.len())].to_string();
                    }
                } else if !rule.rule_data.case_sensitive && data_to_change_lowercase.ends_with(&text_to_trim_lowercase) {
                    if text_to_trim_lowercase.len() == data_to_change_lowercase.len() {
                        return_string = "".to_string();
                    } else {
                        return_string = data_to_change[0..(data_to_change_lowercase.len() - text_to_trim_lowercase.len())].to_string();
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
    use crate::rule::rule_trim::rule_trim;
    use crate::rule::rules::{RulePlace, RuleType, SingleRule};

    #[test]
    fn test_trim() {
        let mut rule = SingleRule::new();
        rule.rule_type = RuleType::Trim;

        rule.rule_place = RulePlace::FromExtensionEndReverse;
        rule.rule_data.trim_text = "Txt".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.txt", &rule), "Roman.");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.txt", &rule), "Roman.txt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.Txt", &rule), "Roman.");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.Txt", &rule), "Roman.");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.asb", &rule), "Roman.asb");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.asb", &rule), "Roman.asb");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("txt", &rule), "");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Txt", &rule), "");

        rule.rule_place = RulePlace::FromNameStart;
        rule.rule_data.trim_text = "R".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.txt", &rule), "oman.txt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.txt", &rule), "oman.txt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("roman.txt", &rule), "oman.txt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("roman.txt", &rule), "roman.txt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Koman.txt", &rule), "Koman.txt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Koman.txt", &rule), "Koman.txt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("r", &rule), "");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("R", &rule), "");

        rule.rule_place = RulePlace::FromNameEndReverse;
        rule.rule_data.trim_text = "n".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.txt", &rule), "Roma.txt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.txt", &rule), "Roma.txt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("RomaN.txt", &rule), "Roma.txt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("RomaN.txt", &rule), "RomaN.txt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("RomaZ.txt", &rule), "RomaZ.txt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("RomaZ.txt", &rule), "RomaZ.txt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("N", &rule), "");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("n", &rule), "");

        rule.rule_place = RulePlace::FromExtensionStart;
        rule.rule_data.trim_text = "t".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.txt", &rule), "Roman.xt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.txt", &rule), "Roman.xt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.Txt", &rule), "Roman.xt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.Txt", &rule), "Roman.Txt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.Zxt", &rule), "Roman.Zxt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.Zxt", &rule), "Roman.Zxt");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("T", &rule), "T");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("t", &rule), "t");
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_trim("Roman.t", &rule), "Roman.");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.t", &rule), "Roman.");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_trim("Roman.T", &rule), "Roman.T");
    }
}
