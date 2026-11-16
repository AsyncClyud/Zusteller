use crate::ui::app;
use reqwest::header::{HeaderName, HeaderValue};

pub fn show_current_headers(headermap: reqwest::header::HeaderMap) -> String {
    let mut headers = String::from("");
    for (key, value) in headermap.iter() {
        headers += key.as_str();
        headers += ": ";
        headers += value.to_str().unwrap();
        headers += " "
    }
    return headers;
}

pub fn add_headers(requ: &mut app::PostMan) -> Result<(), String> {
    let key = HeaderName::from_bytes(requ.header_key.as_bytes()).map_err(|e| e.to_string())?;
    let value = HeaderValue::from_bytes(requ.header_value.as_bytes()).map_err(|e| e.to_string())?;
    requ.headermap.insert(key, value);
    Ok(())
}
