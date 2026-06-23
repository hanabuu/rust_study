//! 入力ディレクトリを走査し、処理対象ファイルを分類して収集するモジュールです。
//!
//! Dstruct ファイルを起点に関連 JSON を束ね、マスタ系ファイルは個別に拾い上げます。

use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

use crate::config::AppConfig;
use crate::error::{AppError, AppResult};

static RE_IDENTIFY_JSON: Lazy<Regex> = Lazy::new(|| Regex::new("identifyData.json").unwrap());  // JSONファイル判定
static RE_SAMPLE_JSON: Lazy<Regex> = Lazy::new(|| Regex::new("sampleJson.json").unwrap());  // JSONファイル判定
static RE_CSV: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\.csv$").unwrap());  // CSVファイル判定
static RE_TEXT: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\.txt$").unwrap());  // テキストファイル判定

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// 収集したファイルの論理種別です。
///
/// ファイル名パターンに応じて、後続処理の分岐に利用します。
///
/// # Examples
/// ```ignore
/// let kind = FileKind::CSVDATA;
/// assert_eq!(kind.to_string(), "CSV");
/// ```
pub enum FileKind {
    /// identifyData.json を表す種別。
    IDENTIFYJSONDATA,
    /// sampleJson.json を表す種別。
    SAMPLEJSONDATA,
    /// CSV ファイルを表す種別。
    CSVDATA,
    /// テキストファイルを表す種別。
    TEXTDATA,
}

impl fmt::Display for FileKind {
    /// ログ出力向けの短い種別ラベルへ変換します。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            FileKind::IDENTIFYJSONDATA => "IDENTIFY_JSON",
            FileKind::SAMPLEJSONDATA => "SAMPLE_JSON",
            FileKind::CSVDATA => "CSV",
            FileKind::TEXTDATA => "TXT",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone)]
/// 走査で見つかった1ファイル分の情報です。
///
/// `identify` は IDENTIFY JSON から抽出した識別子で、関連ファイルに引き継がれます。
///
/// # Examples
/// ```ignore
/// let entry = FileEntry {
///     path: std::path::PathBuf::from("sampleJson.json"),
///     kind: FileKind::SAMPLEJSONDATA,
///     identify: Some("A001".to_string()),
/// };
/// assert!(entry.identify.is_some());
/// ```
pub struct FileEntry {
    /// ファイルの絶対または相対パス。
    pub path: PathBuf,
    /// ファイルの論理種別。
    pub kind: FileKind,
    /// 識別子。対象外ファイルでは `None`。
    pub identify: Option<String>,
}

/// 設定済みルート配下から処理対象ファイルを収集するスキャナです。
///
/// `AppConfig` への参照を保持し、JSON/CSV/TEXT を横断して `FileEntry` の一覧を構築します。
pub struct FileScanner<'a> {
    /// 走査対象ルートを保持する設定参照。
    config: &'a AppConfig,
}

impl<'a> FileScanner<'a> {
    /// 設定参照を保持したスキャナを生成します。
    ///
    /// # Examples
    /// ```ignore
    /// let config = AppConfig::load_from_default()?;
    /// let scanner = FileScanner::new(&config);
    /// # let _ = scanner;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(config: &'a AppConfig) -> Self {
        Self { config }
    }

    /// 設定された各ルートディレクトリを走査し、処理対象の一覧を返します。
    ///
    /// 同一ディレクトリは重複処理を避け、結果はパス順にソートされます。
    ///
    /// # Examples
    /// ```ignore
    /// let config = AppConfig::load_from_default()?;
    /// let scanner = FileScanner::new(&config);
    /// let entries = scanner.collect()?;
    /// println!("{}", entries.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    /// AppError - ディレクトリ列挙失敗、JSON 読み取り/解析失敗、identify 欠落時に返ります。
    pub fn collect(&self) -> AppResult<Vec<FileEntry>> {
        let mut entries = Vec::new();
        let mut visited_dirs = HashSet::new();

        collect_from_root(&self.config.root_json_dir, 2, true, &mut visited_dirs, &mut entries)?;
        collect_csv(self.config.root_csv_dir.as_path(), &mut entries, FileKind::CSVDATA, &RE_CSV)?;
        collect_text(self.config.root_text_dir.as_path(), &mut entries)?;

        entries.sort_by(|a, b| a.path.cmp(&b.path));
        entries.dedup_by(|a, b| a.path == b.path && a.kind == b.kind);

        Ok(entries)
    }
}

/// 指定ルート配下を走査し、IDENTIFY JSON 起点で関連ファイルを収集します。
///
/// # Errors
/// AppError - ディレクトリ走査、identify 抽出、または関連ファイル収集に失敗した場合に返ります。
fn collect_from_root(
    root: &Path,
    depth: usize,
    include_identify_entry: bool,
    visited_dirs: &mut HashSet<PathBuf>,
    entries: &mut Vec<FileEntry>,
) -> AppResult<()> {
    if !root.exists() {
        return Ok(());
    }

    let candidates = collect_files(root, depth)?;
    for file in candidates {
        let file_name = file
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        if RE_IDENTIFY_JSON.is_match(file_name) {
            if let Some(parent) = file.parent() {
                if visited_dirs.insert(parent.to_path_buf()) {
                    handle_identify_directory(parent, &file, include_identify_entry, entries)?;
                }
            }
        }
   }

    Ok(())
}

/// 指定深度までのファイルを列挙して返します。
///
/// # Errors
/// AppError - ディレクトリ読み取り中に I/O エラーが発生した場合に返ります。
fn collect_files(root: &Path, depth: usize) -> AppResult<Vec<PathBuf>> {
    let mut result = Vec::new();
    collect_files_recursive(root, depth, &mut result)?;
    Ok(result)
}

/// 再帰的にディレクトリをたどり、対象深度のファイルを蓄積します。
///
/// # Errors
/// AppError - 下位ディレクトリの列挙中に I/O エラーが発生した場合に返ります。
fn collect_files_recursive(root: &Path, depth: usize, acc: &mut Vec<PathBuf>) -> AppResult<()> {
    if depth == 0 {
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursive(&path, depth - 1, acc)?;
        } else if depth == 1 {
            acc.push(path);
        }
    }

    Ok(())
}

/// IDENTIFY JSON と同じディレクトリにある関連ファイルへ identify 情報を付与して収集します。
///
/// # Errors
/// AppError - identify 抽出、またはディレクトリ列挙に失敗した場合に返ります。
fn handle_identify_directory(
    dir: &Path,
    identify_path: &Path,
    include_identify_entry: bool,
    entries: &mut Vec<FileEntry>,
) -> AppResult<()> {
    let identify = extract_identify_from_json(identify_path)?;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if RE_SAMPLE_JSON.is_match(name) {
            entries.push(FileEntry {
                path: path.clone(),
                kind: FileKind::SAMPLEJSONDATA,
                identify: Some(identify.clone()),
            });
        }
    }

    if include_identify_entry {
        entries.push(FileEntry {
            path: identify_path.to_path_buf(),
            kind: FileKind::IDENTIFYJSONDATA,
            identify: Some(identify),
        });
    }

    Ok(())
}

#[allow(dead_code)]
/// IDENTIFY JSON から `identify` の先頭値を抽出します。
///
/// # Errors
/// AppError - ファイル読み込み失敗、JSON 解析失敗、または `identify` フィールド不正時に返ります。
fn extract_identify(path: &Path) -> AppResult<String> {
    let raw = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&raw)?;
    let value = json
        .get("identify")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("identify"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            AppError::InvalidData(format!("identify missing in {}", path.display()))
        })?;
    Ok(value.to_string())
}

fn extract_identify_from_json(path: &Path) -> AppResult<String> {
    let raw = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&raw)?;

    // 1) {"identify":"test1"} 形式を優先
    if let Some(s) = json.get("identify").and_then(|v| v.as_str()) {
        return Ok(s.to_string());
    }

    // 2) {"identify":[{"identify":"test1"}]} 形式も許容
    if let Some(s) = json
        .get("identify")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("identify"))
        .and_then(|v| v.as_str())
    {
        return Ok(s.to_string());
    }

    Err(AppError::InvalidData(format!(
        "identify missing or invalid in {}",
        path.display()
    )))
}

/// 指定ディレクトリ配下から対象テキストファイルを収集します。
///
/// # Errors
/// AppError - ディレクトリ読み取りに失敗した場合に返ります。
fn collect_text(root: &Path, entries: &mut Vec<FileEntry>) -> AppResult<()> {
    if !root.exists() {
        return Ok(());
    }

    if root.is_file() {
        let name = root.file_name().and_then(|s| s.to_str()).unwrap_or_default();
        if RE_TEXT.is_match(name) {
            entries.push(FileEntry {
                path: root.to_path_buf(),
                kind: FileKind::TEXTDATA,
                identify: None,
            });
        }
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
        if RE_TEXT.is_match(name) {
            entries.push(FileEntry {
                path,
                kind: FileKind::TEXTDATA,
                identify: None,
            });
        }
    }

    Ok(())
}

/// 指定ディレクトリ直下から正規表現に一致する CSV ファイルを収集します。
///
/// # Errors
/// AppError - ディレクトリ読み取りに失敗した場合に返ります。
fn collect_csv(root: &Path,entries: &mut Vec<FileEntry>,kind: FileKind,regex: &Regex) -> AppResult<()> {
    if !root.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if !entry.path().is_file() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if regex.is_match(&name) {
            entries.push(FileEntry {
                path: entry.path(),
                kind,
                identify: None,
            });
        }
    }

    Ok(())
}