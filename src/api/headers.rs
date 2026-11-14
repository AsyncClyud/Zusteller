use crate::ui::app;
use reqwest::header::{HeaderName, HeaderValue};

pub fn add_headers(requ: &mut app::PostMan) -> Result<(), String> {
    let key = HeaderName::from_bytes(requ.header_key.as_bytes()).map_err(|e| e.to_string())?;
    let value = HeaderValue::from_bytes(requ.header_value.as_bytes()).map_err(|e| e.to_string())?;
    requ.headermap.insert(key, value);
    Ok(())
}
