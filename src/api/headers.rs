use crate::ui::app::{self};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn add_headers(requ: &mut app::PostMan) -> Result<(), String> {
    let key = HeaderName::from_bytes(requ.header_key.as_bytes()).map_err(|e| e.to_string())?;
    let value = HeaderValue::from_bytes(requ.header_value.as_bytes()).map_err(|e| e.to_string())?;
    requ.headermap.insert(key, value);
    Ok(())
}

pub fn remove_header(headermap: &mut HeaderMap, key: String) {
    let _ = headermap.remove(key);
}

pub fn clear_headers(requ: &mut app::PostMan) {
    requ.headermap.clear();
}
