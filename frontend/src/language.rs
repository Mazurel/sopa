use gloo::utils::window;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

use crate::cookies;

const DEFAULT_LANGUAGE: &str = "en";

// TODO: Add here error handling

fn set_browser_language_setting_in_cookie(language: &str) -> Result<(), String> {
    cookies::set_cookie("language", language)
}

fn get_browser_language_setting_in_cookie() -> Option<String> {
    match cookies::get_cookie("language") {
        Ok(language) => Some(language),
        Err(_) => None,
    }
}

fn transform_browser_language(browser_language: String) -> String {
    match browser_language.as_str() {
        "en-US" => "en".to_string(),
        _ => browser_language,
    }
}

fn get_browser_language_setting_in_locale() -> Option<String> {
    let browser_navigator = window().navigator();
    browser_navigator.language().map(transform_browser_language)
}

pub fn get_emoji_for_language(language: &str) -> String {
    let emoji = match language {
        "en" => "🇺🇸",
        "pl" => "🇵🇱",
        _ => "🌐",
    };
    emoji.to_string()
}

pub fn list_supported_languages() -> Vec<String> {
    rust_i18n::available_locales!()
        .into_iter()
        .map(|l| l.to_string())
        .collect()
}

/// Set the language for the application.
pub fn set_language(language: String) -> Result<(), String> {
    let possible_locales = rust_i18n::available_locales!();
    let is_language_supported = possible_locales
        .into_iter()
        .find(|supported_language| language == *supported_language)
        .is_some();

    if !is_language_supported {
        return Err(format!("Unsupported language {language}"));
    }

    rust_i18n::set_locale(&language);
    set_browser_language_setting_in_cookie(&language)?;
    Ok(())
}

pub fn init_language_settings() {
    let browser_selected_language = get_browser_language_setting_in_locale();
    let cookie_selected_language = get_browser_language_setting_in_cookie();

    let selected_language = match cookie_selected_language {
        Some(language) => language,
        None => match browser_selected_language {
            Some(language) => language,
            None => DEFAULT_LANGUAGE.to_string(),
        },
    };

    if let Err(error_message) = set_language(selected_language) {
        log::error!("Failed to set language: {}", error_message);
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_list_supported_languages() {
        let supported_languages = list_supported_languages();
        assert!(!supported_languages.is_empty());
    }

    /* TODO: Reenable wtih Mocks */
    /*
    #[test]
    fn test_set_language() {
        let language = "en";
        assert!(set_language(language.to_string()).is_ok());
    }
    */
}
