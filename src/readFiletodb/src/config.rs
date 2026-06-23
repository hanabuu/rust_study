//! .env から設定を読み込み、アプリ各処理で利用するパス情報を提供するモジュール。
use std::path::{Path, PathBuf};

use dotenvy::dotenv;
use std::env;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone)]
/// 入出力ディレクトリと書き込み先データベースパスを保持する設定。
///
/// CSV/JSON/TEXT の各ルートディレクトリと、書き込み先 DB ファイルの絶対または相対パスを
/// 一元管理する。
///
/// # Examples
/// ```ignore
/// let config = AppConfig::load_from_default()?;
/// println!("{:?}", config.database_path);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct AppConfig {
    /// CSV ファイルのルートディレクトリ。
    pub root_csv_dir: PathBuf,
    /// JSON ファイルのルートディレクトリ。
    pub root_json_dir: PathBuf,
    /// テキストファイルのルートディレクトリ。
    pub root_text_dir: PathBuf,
    /// 書き込み先 SQLite などのデータベースファイルパス。
    pub database_path: PathBuf,
}

impl AppConfig {
    /// 既定手順（`.env` 読み込み）で `AppConfig` を構築する。
    ///
    /// 内部的には [`Self::load_env`] を呼び出すラッパーであり、アプリ起動時の標準的な
    /// 設定読み込み窓口として利用する。
    ///
    /// # Examples
    /// ```ignore
    /// let config = AppConfig::load_from_default()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    /// [`AppError`] - `.env` の読み込み失敗、必須環境変数未設定、またはカレントディレクトリ
    /// 取得失敗時に返る。
    pub fn load_from_default() -> AppResult<Self> {
        Self::load_env()        // セミコロンなしなので、式として扱われ、戻り値がそのまま返される
    }

    /// 環境変数から設定値を取得し、`AppConfig` を構築する。
    ///
    /// 必須キー `CSV_DIR`、`JSON_DIR`、`TEXT_DIR`、`WRITE_DB` を読み取り、
    /// 相対パスは作業ディレクトリ基準で絶対パスに解決する。
    ///
    /// # Examples
    /// ```ignore
    /// let config = AppConfig::load_env()?;
    /// assert!(config.database_path.as_os_str().len() > 0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    /// [`AppError`] - `.env` 読み込み失敗、必須環境変数未設定、またはパス解決失敗時に返る。
    pub fn load_env() -> AppResult<Self> {
        dotenv()?;                  // .envファイルの存在チェック。なければエラーとなる
        let csv_dir = require_env("CSV_DIR")?;
        let json_dir = require_env("JSON_DIR")?;
        let text_dir = require_env("TEXT_DIR")?;
        let write_db = require_env("WRITE_DB")?;

        Ok(Self {
            root_csv_dir: resolve_path(&csv_dir)?,
            root_json_dir: resolve_path(&json_dir)?,
            root_text_dir: resolve_path(&text_dir)?,
            database_path: resolve_path(&write_db)?,
        })
    }
}

/// 必須環境変数を取得し、未設定ならエラーを返す。
///
/// 空文字は許容されるため、未設定判定は `std::env::VarError::NotPresent` に従う。
///
/// # Examples
/// ```ignore
/// let csv_dir = require_env("CSV_DIR")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
/// [`AppError::EnvVar`] - 環境変数が未設定、または不正 Unicode などで取得失敗した場合に返る。
fn require_env(key: &str) -> AppResult<String> {
    env::var(key).map_err(|source| AppError::EnvVar {
        key: key.to_string(),
        source,
    })
}

#[allow(dead_code)]
/// 任意環境変数を優先順に探索し、最初に見つかった値を返す。
///
/// `keys` を先頭から評価し、空文字以外の値が見つかった時点で `Some(value)` を返す。
/// いずれも未設定または空文字の場合は `Ok(None)` を返す。
///
/// # Examples
/// ```ignore
/// let value = optional_env(&["PRIMARY_KEY", "FALLBACK_KEY"])?;
/// println!("{:?}", value);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
/// [`AppError::EnvVar`] - 不正 Unicode など、変数値の取得時にエラーが発生した場合に返る。
fn optional_env(keys: &[&str]) -> AppResult<Option<String>> {
    for key in keys {
        match env::var(key) {
            Ok(value) if !value.trim().is_empty() => return Ok(Some(value)),
            Ok(_) => continue,
            Err(env::VarError::NotPresent) => continue,
            Err(source) => {
                return Err(AppError::EnvVar {
                    key: (*key).to_string(),
                    source,
                });
            }
        }
    }
    Ok(None)
}

/// 入力文字列をパスへ変換し、相対パスなら作業ディレクトリ基準で解決する。
///
/// 既に絶対パスであればそのまま返し、相対パスであれば `env::current_dir()` の結果に
/// 結合して返す。
///
/// # Examples
/// ```ignore
/// let abs = resolve_path("./data/input.csv")?;
/// assert!(abs.is_absolute());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
/// [`AppError`] - カレントディレクトリ取得に失敗した場合に返る。
fn resolve_path(value: &str) -> AppResult<PathBuf> {
    let path = Path::new(value);        // Pathオブジェクトに変換する
    if path.is_absolute() {
        // 絶対パスの場合、そのまま返す。
        Ok(path.to_path_buf())
    } else {
        // 相対パスの場合、カレントディレクトリ基準で絶対パスに変換する。
        let base = env::current_dir()?; // .envの相対指定はカレントディレクトリ基準で解決する
        Ok(base.join(path))
    }
}