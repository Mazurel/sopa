use crate::cookies;
use gloo::utils::{document, window};

#[derive(Clone, Copy)]
pub enum ColorScheme {
    Light,
    Dark,
}
const DEFAULT_COLOR_SCHEME: ColorScheme = ColorScheme::Light;

impl ColorScheme {
    pub fn from_string(s: &str) -> Option<ColorScheme> {
        match s {
            "light" => Some(ColorScheme::Light),
            "dark" => Some(ColorScheme::Dark),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ColorScheme::Light => "light".to_string(),
            ColorScheme::Dark => "dark".to_string(),
        }
    }
}

fn get_browser_color_scheme() -> Option<ColorScheme> {
    match window().match_media("(prefers-color-scheme: dark)") {
        Ok(maybe_media_query_list) => {
            if let Some(media_query_list) = maybe_media_query_list {
                if media_query_list.matches() {
                    return Some(ColorScheme::Dark);
                } else {
                    return Some(ColorScheme::Light);
                }
            }
        }
        Err(_) => {}
    };

    None
}

fn sync_color_scheme_setting_with_html_document(color_scheme: ColorScheme) -> Result<(), String> {
    let html_element = document().document_element().unwrap();
    // NOTE: This is actually utilized by bulma automatically
    html_element
        .set_attribute("data-theme", &color_scheme.to_string())
        .map_err(|err| format!("Failed to set data-theme attribute: {:?}", err))
}

pub fn init_color_scheme_system() {
    let color_scheme = get_current_color_scheme();
    let _ = sync_color_scheme_setting_with_html_document(color_scheme);
}

pub fn set_current_color_scheme(color_scheme: ColorScheme) -> Result<(), String> {
    cookies::set_cookie("color_scheme", &color_scheme.to_string())
        .and_then(|_| sync_color_scheme_setting_with_html_document(color_scheme))
}

pub fn get_current_color_scheme() -> ColorScheme {
    // We always first check for cookie, browser preference is next.
    // When first logged in, we set the color scheme to the browser preference.
    // If no cookie is found, we use the browser preference.
    if let Ok(Some(color_scheme)) = cookies::get_cookie("color_scheme")
        .map(|maybe_scheme| ColorScheme::from_string(&maybe_scheme))
    {
        let _ = set_current_color_scheme(color_scheme);
        return color_scheme;
    }

    if let Some(color_scheme) = get_browser_color_scheme() {
        let _ = set_current_color_scheme(color_scheme);
        return color_scheme;
    }

    DEFAULT_COLOR_SCHEME
}
