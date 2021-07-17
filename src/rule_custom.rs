use crate::help_function::split_file_name;
use crate::rules::*;
use chrono::NaiveDateTime;
use humansize::{file_size_opts as options, FileSize};
use std::cmp::min;
use std::path::Path;

pub fn rule_custom(data_to_change: &str, rule: &SingleRule, rule_number: u64, file_data: Option<(u64, u64, u64)>) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let mut return_string = rule.rule_data.custom_text.clone();

    let creation_date: String;
    let modification_date: String;
    let size: String;

    // Random data to visualize typical
    if let Some(f_data) = file_data {
        modification_date = NaiveDateTime::from_timestamp(f_data.0 as i64, 0).to_string();
        creation_date = NaiveDateTime::from_timestamp(f_data.1 as i64, 0).to_string();
        size = f_data.2.file_size(options::BINARY).unwrap();
    } else {
        creation_date = "2021-01-31 08:42:12".to_string();
        modification_date = "2015-11-15 14:24:55".to_string();
        size = "2 KB".to_string();
    }

    match rule.rule_type {
        RuleType::Custom => match rule.rule_place {
            RulePlace::None => {
                return_string = return_string.replace("$(NAME)", name.as_str());
                return_string = return_string.replace("$(EXT)", extension.as_str());
                return_string = return_string.replace("$(SIZE)", size.as_str());
                return_string = return_string.replace("$(CREAT)", creation_date.as_str());
                return_string = return_string.replace("$(MODIF)", modification_date.as_str());
                return_string = return_string.replace("$(CURR)", data_to_change);

                // Part with numbers - TODO move this to different function
                if return_string.contains("$(N:") {
                    let mut index = 0;
                    while let Some(mut start_index) = return_string[index..].find("$(N:") {
                        start_index += index + 4;
                        if let Some(mut first_double) = return_string[start_index..].find(':') {
                            first_double += start_index;
                            if let Some(mut second_double) = return_string[first_double + 1..].find(':') {
                                second_double += first_double + 1;
                                if let Some(mut end_index) = return_string[second_double..].find(')') {
                                    end_index += second_double;

                                    let str_start_number = return_string[start_index..first_double].to_string();
                                    let str_step_number = return_string[first_double + 1..second_double].to_string();
                                    let str_fill_zeros = return_string[second_double + 1..end_index].to_string();

                                    if let Ok(start_number) = str_start_number.parse::<i64>() {
                                        if let Ok(step_number) = str_step_number.parse::<i64>() {
                                            if let Ok(fill_zeros) = str_fill_zeros.parse::<u64>() {
                                                // TODO think about putting it to docs or explaining it somewhere that bigger values will crash entire app, so value must be clamped
                                                let fill_zeros = min(fill_zeros, 50);

                                                let mut number;
                                                if step_number.checked_mul(rule_number as i64).is_none() {
                                                    number = 0;
                                                } else {
                                                    number = step_number * rule_number as i64;
                                                }
                                                // TODO Better handle negative numbers

                                                if number.checked_add(start_number).is_none() {
                                                    number = 0;
                                                } else {
                                                    number += start_number;
                                                }

                                                let mut text_to_replace = number.to_string();

                                                let mut zeros: String = "".to_string();
                                                if text_to_replace.len() < fill_zeros as usize {
                                                    for _i in 0..(fill_zeros - text_to_replace.len() as u64) {
                                                        zeros.push('0');
                                                    }
                                                    text_to_replace = zeros + text_to_replace.as_str();
                                                }

                                                return_string = format!("{}{}{}", &return_string[..start_index - 4], text_to_replace, &return_string[end_index + 1..]);

                                                index = start_index - 4 + text_to_replace.len();
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // Some things was not parsed successfully, so we need to start 4 positions later(after first "$(N:")
                        index = start_index;
                    }
                }
            }
            _ => panic!("Invalid Rule Place for Custom"),
        },
        _ => panic!("Invalid Rule Type for Custom"),
    }

    return_string
}

#[cfg(test)]
mod test {
    use crate::rule_custom::rule_custom;
    use crate::rules::{RulePlace, RuleType, SingleRule};
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_custom() {
        let mut rule: SingleRule = SingleRule::new();
        rule.rule_type = RuleType::Custom;
        rule.rule_place = RulePlace::None;

        let mut file_handler = OpenOptions::new().truncate(true).write(true).create(true).open(Path::new("wombat.txt")).unwrap();
        write!(file_handler, "50").unwrap();
        file_handler.flush().unwrap();

        rule.rule_data.custom_text = "$(CURR)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "wombat.txt");

        rule.rule_data.custom_text = "$(NAME)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "wombat");

        rule.rule_data.custom_text = "$(EXT)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "txt");

        rule.rule_data.custom_text = "$(N:)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "$(N:)");
        rule.rule_data.custom_text = "$(N:20:22:)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "$(N:20:22:)");
        rule.rule_data.custom_text = "$(20::22)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "$(20::22)");
        rule.rule_data.custom_text = "$(N:::22)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "$(N:::22)");
        rule.rule_data.custom_text = "$(:::)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "$(:::)");
        rule.rule_data.custom_text = "$(N:20:22:4)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "0020");
        assert_eq!(rule_custom("wombat.txt", &rule, 1, None), "0042");
        assert_eq!(rule_custom("wombat.txt", &rule, 2, None), "0064");
        rule.rule_data.custom_text = "$(N:1:10:3)$(N:2:10:4)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "0010002");
        assert_eq!(rule_custom("wombat.txt", &rule, 1, None), "0110012");
        rule.rule_data.custom_text = "$(N:0:2:5)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "00000");
        assert_eq!(rule_custom("wombat.txt", &rule, 1, None), "00002");
        assert_eq!(rule_custom("wombat.txt", &rule, 2, None), "00004");
        rule.rule_data.custom_text = "$(N:10:5:1)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "10");
        assert_eq!(rule_custom("wombat.txt", &rule, 1, None), "15");
        assert_eq!(rule_custom("wombat.txt", &rule, 2, None), "20");
        rule.rule_data.custom_text = "$(EXT)$(())$(($(N:10:5:1)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "txt$(())$((10");

        rule.rule_data.custom_text = "$(SIZE)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, None), "2 KB");

        rule.rule_data.custom_text = "$(MODIF)".to_string();
        let text = rule_custom("wombat.txt", &rule, 0, None);
        assert!(text.contains('-') && text.contains("20"));

        rule.rule_data.custom_text = "$(CREAT)".to_string();
        let text = rule_custom("wombat.txt", &rule, 0, None);
        assert!(text.contains('-') && text.contains("20"));

        fs::remove_file("wombat.txt").unwrap();
    }
}
