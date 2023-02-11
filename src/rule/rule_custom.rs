use std::cmp::min;
use std::path::Component::Normal;
use std::path::Path;

use chrono::NaiveDateTime;
use humansize::format_size;
use humansize::BINARY;

use crate::help_function::split_file_name;
use crate::rule::rules::*;

pub fn rule_custom(data_to_change: &str, rule: &SingleRule, general_rule_number: u64, rule_number_in_folder: u64, file_data: Option<(u64, u64, u64, &str)>) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let string_to_parse = rule.rule_data.custom_text.clone();

    let creation_date: String;
    let modification_date: String;
    let size: String;
    let parent_folder;

    let mut new_string = String::new();

    // Random data to visualize typical usage in examples
    if let Some(f_data) = file_data {
        modification_date = NaiveDateTime::from_timestamp_opt(f_data.0 as i64, 0).unwrap().to_string().replace(':', "_");
        creation_date = NaiveDateTime::from_timestamp_opt(f_data.1 as i64, 0).unwrap().to_string().replace(':', "_");
        size = format_size(f_data.2, BINARY);
        if let Some(last_component) = Path::new(&f_data.3).components().last() {
            if let Normal(path) = last_component {
                parent_folder = path.to_str().unwrap_or("").to_string();
            } else {
                parent_folder = String::new();
                eprintln!("Failed to read latest component from {last_component:?}");
            }
        } else {
            parent_folder = String::new();
        }
    } else {
        creation_date = "2021-01-31 08_42_12".to_string();
        modification_date = "2015-11-15 14_24_55".to_string();
        size = "2 KB".to_string();
        parent_folder = "Parent Folder".to_string();
    }

    match rule.rule_type {
        RuleType::Custom => match rule.rule_place {
            RulePlace::None => {
                // let mut start_index: u32 = 0;
                let mut latest_end_index: usize = 0;
                loop {
                    if let Some(start) = string_to_parse[latest_end_index..].find("$(") {
                        if let Some(end) = string_to_parse[latest_end_index + 2 + start..].find(')') {
                            new_string.push_str(&string_to_parse[latest_end_index..latest_end_index + start]);

                            let typ = string_to_parse[latest_end_index + start + 2..end + start + latest_end_index + 2].split(':').collect::<Vec<&str>>();

                            if !typ.is_empty() {
                                let invalid_data = parse_string_rules(
                                    &typ,
                                    &mut new_string,
                                    &general_rule_number,
                                    &rule_number_in_folder,
                                    &name,
                                    &creation_date,
                                    &modification_date,
                                    &size,
                                    &parent_folder,
                                    data_to_change,
                                    &extension,
                                );

                                if invalid_data {
                                    new_string.push_str(&string_to_parse[latest_end_index + start..latest_end_index + 2 + start]);
                                    latest_end_index = start + latest_end_index + 2;
                                } else {
                                    latest_end_index = start + end + 1 + latest_end_index + 2;
                                }
                            }
                        } else {
                            new_string.push_str(&string_to_parse[latest_end_index + start..]);
                            break; // No more )
                        }
                    } else {
                        new_string.push_str(&string_to_parse[latest_end_index..]);
                        break; // There is no more things special things
                    }
                }
            }
            _ => panic!("Invalid Rule Place for Custom"),
        },
        _ => panic!("Invalid Rule Type for Custom"),
    }

    new_string
}

#[allow(clippy::too_many_arguments)]
pub fn parse_string_rules(
    typ: &[&str],
    new_string: &mut String,
    general_rule_number: &u64,
    rule_number_in_folder: &u64,
    name: &str,
    creation_date: &str,
    modification_date: &str,
    size: &str,
    parent_folder: &str,
    data_to_change: &str,
    extension: &str,
) -> bool {
    let mut invalid_data = true;
    'mat: {
        match typ[0] {
            "CURR" => {
                if typ.len() == 1 {
                    new_string.push_str(data_to_change);
                    invalid_data = false;
                }
            }
            "NAME" => {
                if typ.len() == 1 {
                    new_string.push_str(name);
                    invalid_data = false;
                }
            }
            "EXT" => {
                if typ.len() == 1 {
                    new_string.push_str(extension);
                    invalid_data = false;
                }
            }
            "SIZE" => {
                if typ.len() == 1 {
                    new_string.push_str(size);
                    invalid_data = false;
                }
            }
            "CREAT" => {
                if typ.len() == 1 {
                    new_string.push_str(creation_date);
                    invalid_data = false;
                }
            }
            "MODIF" => {
                if typ.len() == 1 {
                    new_string.push_str(modification_date);
                    invalid_data = false;
                }
            }
            "PARENT" => {
                if typ.len() == 1 {
                    new_string.push_str(parent_folder);
                    invalid_data = false;
                }
            }
            "N" | "K" => {
                invalid_data = true;
                if (1..=4).contains(&typ.len()) {
                    let start_str = typ.get(1);
                    let start_number = if let Some(start) = start_str {
                        match start.parse::<i64>() {
                            Ok(t) => t,
                            Err(_) => break 'mat,
                        }
                    } else {
                        0
                    };

                    let step_str = typ.get(2);
                    let step_number = if let Some(step) = step_str {
                        match step.parse::<i64>() {
                            Ok(t) => t,
                            Err(_) => break 'mat,
                        }
                    } else {
                        1
                    };

                    let fill_str = typ.get(3);
                    let fill_zeros = if let Some(zero) = fill_str {
                        match zero.parse::<i64>() {
                            Ok(t) => t,
                            Err(_) => break 'mat,
                        }
                    } else {
                        1
                    };

                    // TODO think about putting it to docs or explaining it somewhere that bigger values will crash entire app, so value must be clamped
                    let fill_zeros = min(fill_zeros, 50);

                    let used_number = (if typ[0] == "N" { *general_rule_number } else { *rule_number_in_folder }) as i64;

                    let mut number;
                    if step_number.checked_mul(used_number).is_none() {
                        number = 0;
                    } else {
                        number = step_number * used_number;
                    }

                    if number.checked_add(start_number).is_none() {
                        number = 0;
                    } else {
                        number += start_number;
                    }

                    let mut text_to_replace = number.to_string();

                    let mut zeros: String = String::new();
                    if text_to_replace.len() < fill_zeros as usize {
                        for _i in 0..(fill_zeros - text_to_replace.len() as i64) {
                            zeros.push('0');
                        }
                        text_to_replace = zeros + text_to_replace.as_str();
                    }

                    new_string.push_str(&text_to_replace);

                    invalid_data = false;
                }
            }
            _ => {
                // Just invalid rule
            }
        }
    }
    invalid_data
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::Path;

    use crate::rule::rule_custom::rule_custom;
    use crate::rule::rules::{RulePlace, RuleType, SingleRule};

    #[test]
    fn test_custom() {
        let mut rule: SingleRule = SingleRule::new();
        rule.rule_type = RuleType::Custom;
        rule.rule_place = RulePlace::None;

        let mut file_handler = OpenOptions::new().truncate(true).write(true).create(true).open(Path::new("wombat.txt")).unwrap();
        write!(file_handler, "50").unwrap();
        file_handler.flush().unwrap();

        rule.rule_data.custom_text = "  )  $(CURR)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "  )  wombat.txt");
        rule.rule_data.custom_text = "$($(CURR)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "$(wombat.txt");
        rule.rule_data.custom_text = "$(CURR)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "wombat.txt");
        rule.rule_data.custom_text = "$(CURR )".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "$(CURR )");
        rule.rule_data.custom_text = "$(CURR)$(CURR)$(CURR)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "wombat.txtwombat.txtwombat.txt");
        rule.rule_data.custom_text = "Roman $(CURR) Roman".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "Roman wombat.txt Roman");

        rule.rule_data.custom_text = "$(NAME)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "wombat");

        rule.rule_data.custom_text = "$(EXT)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "txt");

        rule.rule_data.custom_text = "$(K:)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 1000, None), "$(K:)");
        rule.rule_data.custom_text = "$(K:0)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 1000, None), "1000");
        rule.rule_data.custom_text = "$(K)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 1, None), "1");
        rule.rule_data.custom_text = "$(K)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 1111111110, 0, None), "0");

        rule.rule_data.custom_text = "$(N)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 111110, None), "0");
        rule.rule_data.custom_text = "$(N)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 1, 0, None), "1");
        rule.rule_data.custom_text = "$(N:)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "$(N:)");
        rule.rule_data.custom_text = "$(N:20:22:)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "$(N:20:22:)");
        rule.rule_data.custom_text = "$(20::22)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "$(20::22)");
        rule.rule_data.custom_text = "$(N:::22)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "$(N:::22)");
        rule.rule_data.custom_text = "$(:::)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "$(:::)");
        rule.rule_data.custom_text = "$(N:20)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "20");
        assert_eq!(rule_custom("wombat.txt", &rule, 1, 0, None), "21");
        rule.rule_data.custom_text = "$(N:20:2)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 1, 0, None), "22");
        rule.rule_data.custom_text = "$(N:20:22:4)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "0020");
        assert_eq!(rule_custom("wombat.txt", &rule, 1, 0, None), "0042");
        assert_eq!(rule_custom("wombat.txt", &rule, 2, 0, None), "0064");
        rule.rule_data.custom_text = "$(N:1:10:3)$(N:2:10:4)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "0010002");
        assert_eq!(rule_custom("wombat.txt", &rule, 1, 0, None), "0110012");
        rule.rule_data.custom_text = "$(N:0:2:5)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "00000");
        assert_eq!(rule_custom("wombat.txt", &rule, 1, 0, None), "00002");
        assert_eq!(rule_custom("wombat.txt", &rule, 2, 0, None), "00004");
        rule.rule_data.custom_text = "$(N:10:5:1)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "10");
        assert_eq!(rule_custom("wombat.txt", &rule, 1, 0, None), "15");
        assert_eq!(rule_custom("wombat.txt", &rule, 2, 0, None), "20");
        rule.rule_data.custom_text = "$(EXT)$(())$(($(N:10:5:1)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "txt$(())$((10");

        rule.rule_data.custom_text = "$(SIZE)".to_string();
        assert_eq!(rule_custom("wombat.txt", &rule, 0, 0, None), "2 KB");

        rule.rule_data.custom_text = "$(MODIF)".to_string();
        let text = rule_custom("wombat.txt", &rule, 0, 0, None);
        assert!(text.contains('-') && text.contains("20"));

        rule.rule_data.custom_text = "$(CREAT)".to_string();
        let text = rule_custom("wombat.txt", &rule, 0, 0, None);
        assert!(text.contains('-') && text.contains("20"));

        rule.rule_data.custom_text = "$(PARENT)".to_string();
        let text = rule_custom("Absymal.txt", &rule, 0, 0, None);
        assert_eq!(text, "Parent Folder");

        fs::remove_file("wombat.txt").unwrap();
    }
}
