use std::fs;
use std::path::Path;

use encoding_rs::SHIFT_JIS;

use crate::error::{AppError, AppResult};

#[allow(dead_code)]
pub fn read_utf8_lines(path: &Path) -> AppResult<Vec<String>> {
    let content = fs::read_to_string(path)?;
    Ok(normalize_newlines(&content))
}

pub fn read_utf8_line_bytes(path: &Path) -> AppResult<Vec<Vec<u8>>> {
    let bytes = fs::read(path)?;
    std::str::from_utf8(&bytes).map_err(|error| {
        AppError::InvalidData(format!(
            "failed to decode UTF-8 text: {} ({error})",
            path.display()
        ))
    })?;
    Ok(normalize_newline_bytes(&bytes))
}

#[allow(dead_code)]
pub fn read_shift_jis_lines(path: &Path) -> AppResult<Vec<String>> {
    let bytes = fs::read(path)?;
    let (decoded, _, had_errors) = SHIFT_JIS.decode(&bytes);
    if had_errors {
        return Err(AppError::InvalidData(format!(
            "failed to decode Shift_JIS text: {}",
            path.display()
        )));
    }
    Ok(normalize_newlines(decoded.as_ref()))
}

#[allow(dead_code)]
fn normalize_newlines(input: &str) -> Vec<String> {
    input
        .replace("\r\n", "\n")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

fn normalize_newline_bytes(input: &[u8]) -> Vec<Vec<u8>> {
    input
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.strip_suffix(b"\r").unwrap_or(line).to_vec())
        .collect()
}
