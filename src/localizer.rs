use std::collections::HashMap;

use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
use i18n_embed::{DefaultLocalizer, LanguageLoader, Localizer};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader.load_fallback_language(&Localizations).expect("Error while loading fallback language");

    // Fluent wraps interpolated placeables in Unicode bidi-isolation marks (U+2068/U+2069) by
    // default, which leak into persisted strings like rule descriptions saved to JSON.
    // Note: language::apply_language() must re-disable this after every language switch, since
    // FluentLanguageLoader::select() reloads bundles internally and resets this to true.
    loader.set_use_isolating(false);

    loader
});

#[macro_export]
macro_rules! fls {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localizer::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::localizer::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

pub fn generate_translation_hashmap(vec: Vec<(&'static str, String)>) -> HashMap<&'static str, String> {
    let mut hashmap: HashMap<&'static str, String> = HashMap::new();
    for (key, value) in vec {
        hashmap.insert(key, value);
    }
    hashmap
}

#[cfg(test)]
mod test {
    use super::generate_translation_hashmap;

    #[test]
    fn test_no_bidi_isolate_chars_in_interpolated_strings() {
        let s = fls!("rule_description_zeros", generate_translation_hashmap(vec![("zeros", "3".to_string())]));
        assert!(!s.contains('\u{2068}') && !s.contains('\u{2069}'), "bidi isolate chars found in: {s:?}");
    }
}
