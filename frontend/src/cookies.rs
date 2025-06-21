use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

pub fn set_cookie(name: &str, value: &str) -> Result<(), String> {
    let document = document();

    let all_cookies = document
        .unchecked_into::<HtmlDocument>()
        .set_cookie(&format!("{}={}", name, value))
        .map_err(|_| "Failed to set cookie".to_string())?;

    Ok(())
}

pub fn get_cookie(name: &str) -> Result<String, String> {
    let document = document();

    let all_cookies = document
        .unchecked_into::<HtmlDocument>()
        .cookie()
        .map_err(|_| "Failed to get cookie from document".to_string())?;

    let parts: Vec<&str> = all_cookies.split(';').collect();
    for part in parts {
        let pair: Vec<&str> = part.split('=').collect();
        if pair.len() < 2 {
            // Unexpected branch, but better safe than sorry
            continue;
        }
        if pair[0].trim() == name {
            return Ok(pair[1].to_string());
        }
    }

    Err("Cookie not found".to_string())
}
