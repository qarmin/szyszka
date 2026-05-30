use std::path::PathBuf;

use directories_next::ProjectDirs;
use file_rotate::compression::Compression;
use file_rotate::suffix::{AppendTimestamp, FileLimit};
use file_rotate::{ContentLimit, FileRotate};
use handsome_logger::{ColorChoice, CombinedLogger, ConfigBuilder, FormatText, SharedLogger, TermLogger, TerminalMode, TimeFormat, WriteLogger};
use log::{info, LevelFilter, Record};

const APP_NAME: &str = "szyszka";

pub fn get_cache_path() -> Option<PathBuf> {
    ProjectDirs::from("pl", "Qarmin", "Szyszka").map(|p| PathBuf::from(p.cache_dir()))
}

pub fn setup_logger() {
    log_panics::init();

    let term_config = ConfigBuilder::default()
        .set_level(LevelFilter::Info)
        .set_message_filtering(Some(filtering_messages))
        .build();
    let file_config = ConfigBuilder::default()
        .set_level(LevelFilter::Debug)
        .set_write_once(true)
        .set_message_filtering(Some(filtering_messages))
        .set_time_format(TimeFormat::DateTimeWithMicro, None)
        .set_format_text(FormatText::DefaultWithThreadFile.get(), None)
        .build();

    let combined_logger = (|| {
        let cache_path = get_cache_path()?;
        if let Err(e) = std::fs::create_dir_all(&cache_path) {
            #[expect(clippy::print_stderr)]
            {
                eprintln!("Cannot create cache directory {}: {e}", cache_path.display());
            }
            return None;
        }
        let log_path = cache_path.join(format!("{APP_NAME}.log"));

        let write_rotater = FileRotate::new(
            &log_path,
            AppendTimestamp::default(FileLimit::MaxFiles(3)),
            ContentLimit::BytesSurpassed(10 * 1024 * 1024),
            Compression::None,
            None,
        );

        let combined_logs: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new_from_config(term_config.clone()), WriteLogger::new(file_config, write_rotater)];

        CombinedLogger::init(combined_logs).ok().inspect(|()| {
            info!("Logging to file \"{}\" and terminal", log_path.display());
        })
    })();

    if combined_logger.is_none() {
        let _ = TermLogger::init(term_config, TerminalMode::Mixed, ColorChoice::Always);
        info!("Logging to terminal only, file logging is disabled");
    }
}

fn filtering_messages(record: &Record) -> bool {
    record
        .module_path()
        .is_none_or(|module_path| ["szyszka", "log_panics"].iter().any(|t| module_path.starts_with(t)))
}
