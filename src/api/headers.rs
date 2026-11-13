use crate::ui::app;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn add_headers(requ: &mut app::PostMan) -> Result<(), String> {
    let key = HeaderName::from_bytes(requ.header_key.as_bytes()).map_err(|e| e.to_string())?;
    let value = HeaderValue::from_bytes(requ.header_value.as_bytes()).map_err(|e| e.to_string())?;
    let headermap = requ.headermap.clone();
    if if_header_exist(&key, &value, headermap) == true {
        return Err("Header already exist!".into());
    }
    requ.headermap.insert(key, value);
    Ok(())
}

fn if_header_exist(key: &HeaderName, value: &HeaderValue, headermap: HeaderMap) -> bool {
    for k in headermap.keys() {
        if k == key {
            for j in headermap.values() {
                if j == value {
                    return true;
                }
            }
        }
    }
    return false;
}
