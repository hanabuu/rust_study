use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::error::AppResult;

pub fn read_json<T: DeserializeOwned>(path: &Path) -> AppResult<T> {
    let raw = fs::read_to_string(path)?;
    let value = serde_json::from_str(&raw)?;
    Ok(value)
}
