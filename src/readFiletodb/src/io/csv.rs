use std::fs;
use std::io::Cursor;
use std::path::Path;

use csv::ReaderBuilder;
use encoding_rs::{Encoding, SHIFT_JIS};

use crate::error::{AppError, AppResult};

#[allow(dead_code)]
pub fn read_utf8(path: &Path, quoting: bool) -> AppResult<Vec<Vec<String>>> {
    let content = fs::read_to_string(path)?;
    parse_csv(content.as_bytes(), quoting)
}

pub fn read_shift_jis(path: &Path, quoting: bool) -> AppResult<Vec<Vec<String>>> {
    let bytes = fs::read(path)?;
    let (decoded, _, had_errors) = SHIFT_JIS.decode(&bytes);
    if had_errors {
        return Err(AppError::InvalidData(format!(
            "failed to decode Shift_JIS file: {}",
            path.display()
        )));
    }
    parse_csv(decoded.as_bytes(), quoting)
}

#[allow(dead_code)]
pub fn read_drsum(path: &Path, quoting: bool) -> AppResult<Vec<Vec<String>>> {
    let bytes = fs::read(path)?;

    if let Ok(decoded) = String::from_utf8(bytes.clone()) {
        return parse_csv(strip_utf8_bom(&decoded).as_bytes(), quoting);
    }

    let (decoded, _, had_errors) = SHIFT_JIS.decode(&bytes);
    if had_errors {
        return Err(AppError::InvalidData(format!(
            "failed to decode DrSum CSV: {}",
            path.display()
        )));
    }

    parse_csv(strip_utf8_bom(decoded.as_ref()).as_bytes(), quoting)
}

#[allow(dead_code)]
pub fn read_with_encoding(
    path: &Path,
    encoding: &'static Encoding,
    quoting: bool,
) -> AppResult<Vec<Vec<String>>> {
    let bytes = fs::read(path)?;
    let (decoded, _, had_errors) = encoding.decode(&bytes);
    if had_errors {
        return Err(AppError::InvalidData(format!(
            "failed to decode file {} with encoding {}",
            path.display(),
            encoding.name()
        )));
    }
    parse_csv(decoded.as_bytes(), quoting)
}

fn parse_csv(data: &[u8], quoting: bool) -> AppResult<Vec<Vec<String>>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .quoting(quoting)
        .from_reader(Cursor::new(data));

    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record?;
        rows.push(record.iter().map(|field| field.to_string()).collect());
    }
    Ok(rows)
}

#[allow(dead_code)]
fn strip_utf8_bom(input: &str) -> &str {
    input.strip_prefix('\u{feff}').unwrap_or(input)
}
