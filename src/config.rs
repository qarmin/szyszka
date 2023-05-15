use directories_next::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

pub const CUSTOM_TEXT_FILE_NAME: &str = "custom_text_names.txt";
pub const RULES_FILE_NAME: &str = "rules_settings.txt";

pub fn get_config_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Szyszka") {
        return Some(PathBuf::from(proj_dirs.config_dir()));
    }
    None
}

pub fn get_custom_text_config_file() -> Option<PathBuf> {
    if let Some(config_path) = get_config_path() {
        return Some(config_path.join(CUSTOM_TEXT_FILE_NAME));
    }
    None
}

pub fn get_rules_config_file() -> Option<PathBuf> {
    if let Some(config_path) = get_config_path() {
        return Some(config_path.join(RULES_FILE_NAME));
    }
    None
}

const BASIC_CUSTOM_COMMANDS: &str = r#"
FILE_$(N).$(EXT)
FILE_$(K).$(EXT)
"#;

pub fn load_custom_rules() -> Vec<String> {
    if let Some(custom_file) = get_custom_text_config_file() {
        create_custom_text_file_if_needed();

        match fs::read_to_string(custom_file) {
            Ok(content) => {
                return content
                    .lines()
                    .filter_map(|s| {
                        let t = s.trim().to_string();
                        if t.is_empty() {
                            None
                        } else {
                            Some(t)
                        }
                    })
                    .collect();
            }
            Err(e) => {
                eprintln!("Error while reading file with custom texts {e}");
            }
        }
    }
    vec![]
}

pub fn create_custom_text_file_if_needed() {
    if let Some(custom_file) = get_custom_text_config_file() {
        if !Path::new(&custom_file).is_file() {
            let _ = fs::create_dir_all(Path::new(&custom_file).parent().unwrap());
            if let Err(e) = fs::write(&custom_file, BASIC_CUSTOM_COMMANDS) {
                eprintln!("Failed to create file, reason {e}");
            }
        }
    }
}

pub fn create_rule_settings_if_needed() {
    if let Some(rules_config) = get_rules_config_file() {
        if !Path::new(&rules_config).is_file() {
            let _ = fs::create_dir_all(Path::new(&rules_config).parent().unwrap());

            // TODO Add default rules
            if let Err(e) = fs::write(&rules_config, "") {
                eprintln!("Failed to create file {rules_config:?}, reason {e}");
            }
        }
    }
}
