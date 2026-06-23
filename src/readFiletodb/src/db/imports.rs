use std::collections::HashMap;

use rusqlite::params_from_iter;
use serde_json::{Map, Value};

use crate::error::{AppError, AppResult};

use super::Database;

// SD JSON の各レコードをテーブル列順へ並べるためのフィールド定義。

// const IDENTIFY_FIELD: &[&str] = &["identify"];

const SAMPLE_JSON_FIELDS: &[&str] = &[
    "id",
    "name",
    "type",
    "path",
    "price",
    "category",
];

const SAMPLE_DATA_WIDTHS: &[usize] = &[3,1,1,8];

impl Database {

    pub fn import_identify(&self, identify: &str) -> AppResult<()> {

        self.insert_rows("Identify", &[vec![identify.to_string()]])?;
        Ok(())
    }

    pub fn import_sample_json(&self, identify: &str, json_data: &Value) -> AppResult<()> {
        let array_targets: &[(&str, &str, &[&str])] = &[
            ("SampleJsonData", "sample_json_data", SAMPLE_JSON_FIELDS),
             // ここに新しいテーブルのフィールド定義を追加していく。
        ];

        for (table, key, fields) in array_targets {
            self.insert_rows(table, &array_rows(json_data, key, fields, Some(identify))?)?;
        }

        Ok(())
    }

    /// csv_data をヘッダ行を除いて保存します。
    pub fn import_csv_data(&self, rows: &[Vec<String>]) -> AppResult<usize> {
        self.insert_csv_rows(rows, "AllAddress", 15, "code")
    }

    /// マスタ CSV 用の共通登録処理です。
    ///
    /// ヘッダ行を除外し、列数を検証したうえで保存件数を返します。
    fn insert_csv_rows(
        &self,
        rows: &[Vec<String>],
        table: &str,
        width: usize,
        header_marker: &str,
    ) -> AppResult<usize> {
        let filtered = rows
            .iter()
            .filter(|row| row.first().is_some_and(|value| !value.contains(header_marker)))
            .cloned()
            .collect::<Vec<_>>();

        for row in &filtered {
            if row.len() != width {
                return Err(AppError::InvalidData(format!(
                    "{table} row has invalid column count: expected {width}, got {}",
                    row.len()
                )));
            }
        }

        let imported = filtered.len();
        self.insert_rows(table, &filtered)?;
        tracing::info!(table, imported_rows = imported, skipped_rows = rows.len().saturating_sub(imported), "master rows imported");
        Ok(imported)
    }

    pub fn import_text_data(&self, identify: &str, lines: &[Vec<u8>]) -> AppResult<()> {
        let mut sample_text = Vec::new();

        for line in lines {
            if line.starts_with(b"CBB") {
                let mut row = vec![identify.to_string()];
                row.extend(split_fixed(line, SAMPLE_DATA_WIDTHS)?.into_iter().skip(1));
                sample_text.push(row);
            }
        }

        self.insert_rows("SampleText", &sample_text)?;

        Ok(())
    }

    pub fn import_text_data2(&self, lines: &[Vec<u8>]) -> AppResult<()> {
        let mut sample_text = Vec::new();

        for line in lines {
            if line.starts_with(b"CBB") {
                let row = split_fixed(line, SAMPLE_DATA_WIDTHS)?.into_iter().skip(1).collect::<Vec<_>>();
                sample_text.push(row);
            }
        }

        self.insert_rows("SampleText2", &sample_text)?;

        Ok(())
    }

    /// 1テーブル分の行配列をそのまま INSERT します。
    ///
    /// すべて同じ列数であることを確認してから prepared statement で登録します。
    fn insert_rows(&self, table: &str, rows: &[Vec<String>]) -> AppResult<()> {
        if rows.is_empty() {
            return Ok(());
        }

        let placeholders = vec!["?"; rows[0].len()].join(",");
        let sql = format!("INSERT INTO {table} VALUES ({placeholders})");
        let mut stmt = self.connection().prepare(&sql)?;
        for row in rows {
            if row.len() != rows[0].len() {
                return Err(AppError::InvalidData(format!(
                    "row width mismatch for {table}: expected {}, got {}",
                    rows[0].len(),
                    row.len()
                )));
            }
            stmt.execute(params_from_iter(row.iter()))?;
        }
        Ok(())
    }

}

#[allow(dead_code)]
/// JSON 内に存在する場合だけ単票レコードを 1 行として保存します。
fn insert_optional_single(
    database: &Database,
    table: &str,
    document: &Value,
    key: &str,
    fields: &[&str],
    identify: &str,
) -> AppResult<()> {
    if let Some(row) = optional_single_row(document, key, fields, Some(identify))? {
        database.insert_rows(table, &[row])?;
    }
    Ok(())
}

#[allow(dead_code)]
/// JSON 配列の先頭 1 件を必須レコードとして取得します。
fn required_single_row(
    document: &Value,
    key: &str,
    fields: &[&str],
    prefix: Option<&str>,
) -> AppResult<Vec<String>> {
    let rows = array_rows(document, key, fields, prefix)?;
    rows.into_iter().next().ok_or_else(|| {
        AppError::InvalidData(format!("required record {key} is missing or empty"))
    })
}

#[allow(dead_code)]
/// JSON 配列の先頭 1 件を任意レコードとして取得します。
fn optional_single_row(
    document: &Value,
    key: &str,
    fields: &[&str],
    prefix: Option<&str>,
) -> AppResult<Option<Vec<String>>> {
    let rows = array_rows(document, key, fields, prefix)?;
    Ok(rows.into_iter().next())
}

/// JSON 配列を DB 登録用の文字列行配列へ変換します。
fn array_rows(
    document: &Value,
    key: &str,
    fields: &[&str],
    prefix: Option<&str>,
) -> AppResult<Vec<Vec<String>>> {
    let objects = array_objects(document, key)?;
    let mut rows = Vec::with_capacity(objects.len());
    for object in objects {
        rows.push(row_from_object(object, fields, prefix)?);
    }
    Ok(rows)
}

/// 指定キーの JSON 値を「オブジェクト配列」として取り出します。
fn array_objects<'a>(document: &'a Value, key: &str) -> AppResult<Vec<&'a Map<String, Value>>> {
    let Some(value) = document.get(key) else {
        return Ok(Vec::new());
    };
    let array = value.as_array().ok_or_else(|| {
        AppError::InvalidData(format!("{key} must be an array"))
    })?;
    array
        .iter()
        .map(|item| {
            item.as_object().ok_or_else(|| {
                AppError::InvalidData(format!("{key} must contain JSON objects"))
            })
        })
        .collect()
}

/// 1件の JSON オブジェクトを、指定フィールド順の DB 行へ変換します。
fn row_from_object(
    object: &Map<String, Value>,
    fields: &[&str],
    prefix: Option<&str>,
) -> AppResult<Vec<String>> {
    let mut row = Vec::with_capacity(fields.len() + usize::from(prefix.is_some()));
    if let Some(prefix) = prefix {
        row.push(prefix.to_string());
    }
    for field in fields {
        let value = object.get(*field).ok_or_else(|| {
            AppError::InvalidData(format!("missing field {field}"))
        })?;
        row.push(value_to_string(value, field)?);
    }
    Ok(row)
}

/// JSON のスカラ値を DB 保存用の文字列へ統一変換します。
fn value_to_string(value: &Value, field: &str) -> AppResult<String> {
    match value {
        Value::String(text) => Ok(text.clone()),
        Value::Number(number) => Ok(number.to_string()),
        Value::Bool(flag) => Ok(flag.to_string()),
        Value::Null => Ok(String::new()),
        _ => Err(AppError::InvalidData(format!(
            "field {field} must be a scalar value"
        ))),
    }
}

/// 固定長テキストを指定バイト幅ごとに切り出します。
///
/// 履歴ファイルの HBB/HCB などのレコード分解で利用します。
fn split_fixed(line: &[u8], widths: &[usize]) -> AppResult<Vec<String>> {
    let expected_len: usize = widths.iter().sum();
    if line.len() < expected_len {
        return Err(AppError::InvalidData(format!(
            "fixed-width line is too short: expected at least {expected_len} bytes, got {}",
            line.len()
        )));
    }

    let mut start = 0;
    let mut result = Vec::with_capacity(widths.len());
    for width in widths {
        let end = start + width;
        let segment = std::str::from_utf8(&line[start..end]).map_err(|error| {
            AppError::InvalidData(format!(
                "fixed-width segment is not valid UTF-8 at {start}..{end}: {error}"
            ))
        })?;
        result.push(segment.to_string());
        start = end;
    }
    Ok(result)
}

#[allow(dead_code)]
fn normalize_dynamic_headers(headers: &[String]) -> AppResult<Vec<String>> {
    if headers.is_empty() {
        return Err(AppError::InvalidData("DrSum header is empty".to_string()));
    }

    let mut seen = HashMap::<String, usize>::new();
    let mut normalized = Vec::with_capacity(headers.len());

    for (index, header) in headers.iter().enumerate() {
        let base_name = header.trim().trim_start_matches('\u{feff}');
        if base_name.contains('\0') {
            return Err(AppError::InvalidData(format!(
                "DrSum header contains NUL at column {}",
                index + 1
            )));
        }

        let default_name = format!("column_{}", index + 1);
        let candidate = if base_name.is_empty() {
            default_name.clone()
        } else {
            base_name.to_string()
        };

        let entry = seen.entry(candidate.clone()).or_insert(0);
        *entry += 1;
        if *entry == 1 {
            normalized.push(candidate);
        } else {
            normalized.push(format!("{}_{}", candidate, *entry));
        }
    }

    Ok(normalized)
}

#[allow(dead_code)]
fn quote_identifier(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}