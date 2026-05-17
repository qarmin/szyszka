use std::cmp::min;
use std::path::Component::Normal;
use std::path::Path;

use chrono::DateTime;
use humansize::{format_size, BINARY};

use crate::rule::rules::{split_file_name, RulePlace, RuleType, SingleRule};

pub fn rule_custom(data_to_change: &str, rule: &SingleRule, general_rule_number: u64, rule_number_in_folder: u64, file_data: Option<(u64, u64, u64, &str)>) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let string_to_parse = rule.rule_data.custom_text.clone();

    let creation_date: String;
    let modification_date: String;
    let size: String;
    let parent_folder;

    let mut new_string = String::new();

    if let Some(f_data) = file_data {
        modification_date = DateTime::from_timestamp(f_data.0 as i64, 0)
            .expect("Failed to create DateTime(should never happens)")
            .to_string()
            .replace(':', "_");
        creation_date = DateTime::from_timestamp(f_data.1 as i64, 0)
            .expect("Failed to create DateTime(should never happens)")
            .to_string()
            .replace(':', "_");
        size = format_size(f_data.2, BINARY);
        if let Some(last_component) = Path::new(&f_data.3).components().next_back() {
            if let Normal(path) = last_component {
                parent_folder = path.to_str().unwrap_or("").to_string();
            } else {
                parent_folder = String::new();
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
                let mut latest_end_index: usize = 0;
                loop {
                    if let Some(start) = string_to_parse[latest_end_index..].find("$(") {
                        if let Some(end) = string_to_parse[latest_end_index + 2 + start..].find(')') {
                            new_string.push_str(&string_to_parse[latest_end_index..latest_end_index + start]);

                            let typ = string_to_parse[latest_end_index + start + 2..end + start + latest_end_index + 2]
                                .split(':')
                                .collect::<Vec<&str>>();

                            if !typ.is_empty() {
                                let invalid_data = parse_string_rules(
                                    &typ,
                                    &mut new_string,
                                    general_rule_number,
                                    rule_number_in_folder,
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
                            break;
                        }
                    } else {
                        new_string.push_str(&string_to_parse[latest_end_index..]);
                        break;
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
    general_rule_number: u64,
    rule_number_in_folder: u64,
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

                    let fill_zeros = min(fill_zeros, 50);

                    let used_number = (if typ[0] == "N" { general_rule_number } else { rule_number_in_folder }) as i64;

                    let mut number = if step_number.checked_mul(used_number).is_none() { 0 } else { step_number * used_number };

                    if number.checked_add(start_number).is_none() {
                        number = 0;
                    } else {
                        number += start_number;
                    }

                    let mut text_to_replace = number.to_string();

                    if text_to_replace.len() < fill_zeros as usize {
                        let zeros: String = "0".repeat((fill_zeros - text_to_replace.len() as i64) as usize);
                        text_to_replace = zeros + text_to_replace.as_str();
                    }

                    new_string.push_str(&text_to_replace);

                    invalid_data = false;
                }
            }
            _ => {}
        }
    }
    invalid_data
}
