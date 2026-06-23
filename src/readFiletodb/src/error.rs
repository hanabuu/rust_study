//! アプリ全体で共有するエラー型と結果型を定義するモジュール。
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::env::VarError;

use csv::Error as CsvError;
use dotenvy::Error as DotenvError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
/// アプリケーション層の失敗を文脈付きで表現する列挙型。
///
/// I/O、DB、設定、CSV/JSON 解析など複数の失敗要因を単一の型へ集約し、
/// 呼び出し側で一貫したエラーハンドリングを行えるようにする。
///
/// # Examples
/// ```ignore
/// fn do_work() -> AppResult<()> {
///     Err(AppError::InvalidData("missing id".to_string()))
/// }
/// # let _ = do_work();
/// ```
pub enum AppError {
    /// ファイルやディレクトリ操作で発生した I/O エラー。
    Io(io::Error),
    /// `rusqlite` 経由で発生したデータベースエラー。
    Database(rusqlite::Error),
    /// `.env` の読み込み失敗や形式不正を示すエラー。
    Dotenv(DotenvError),
    /// 環境変数取得時のエラー。
    ///
    /// `key` は失敗した変数名、`source` は `std::env::VarError` を保持する。
    EnvVar { key: String, source: VarError },
    /// JSON の解析またはシリアライズに失敗したエラー。
    Json(SerdeJsonError),
    /// CSV の解析に失敗したエラー。
    Csv(CsvError),
    /// データ内容が業務仕様を満たしていないことを示すエラー。
    InvalidData(String),
}

/// `AppError` の表示文字列を提供する。
///
/// 各バリアントに応じて、ログや CLI 出力で判読しやすいメッセージへ整形する。
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "I/O error: {}", err),
            AppError::Database(err) => write!(f, "database error: {}", err),
            AppError::Dotenv(err) => write!(f, ".env error: {}", err),
            AppError::EnvVar { key, source } => write!(f, "env var {key} error: {}", source),
            AppError::Json(err) => write!(f, "json error: {}", err),
            AppError::Csv(err) => write!(f, "csv error: {}", err),
            AppError::InvalidData(message) => write!(f, "invalid data: {}", message),
        }
    }
}

/// 標準エラー連鎖 (`source`) を提供する。
///
/// ラップされたエラーを参照できるため、呼び出し側で根本原因の追跡が可能になる。
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Database(err) => Some(err),
            AppError::Dotenv(err) => Some(err),
            AppError::EnvVar { source, .. } => Some(source),
            AppError::Json(err) => Some(err),
            AppError::Csv(err) => Some(err),
            AppError::InvalidData(_) => None,
        }
    }
}

impl From<io::Error> for AppError {
    /// `io::Error` を `AppError::Io` へ変換する。
    ///
    /// # Examples
    /// ```ignore
    /// let io_err = std::io::Error::other("io failed");
    /// let app_err: AppError = io_err.into();
    /// # let _ = app_err;
    /// ```
    fn from(value: io::Error) -> Self {
        AppError::Io(value)
    }
}

impl From<rusqlite::Error> for AppError {
    /// `rusqlite::Error` を `AppError::Database` へ変換する。
    ///
    /// # Examples
    /// ```ignore
    /// fn to_app(err: rusqlite::Error) -> AppError {
    ///     err.into()
    /// }
    /// # let _ = to_app;
    /// ```
    fn from(value: rusqlite::Error) -> Self {
        AppError::Database(value)
    }
}

impl From<DotenvError> for AppError {
    /// `dotenvy::Error` を `AppError::Dotenv` へ変換する。
    ///
    /// # Examples
    /// ```ignore
    /// fn to_app(err: dotenvy::Error) -> AppError {
    ///     err.into()
    /// }
    /// # let _ = to_app;
    /// ```
    fn from(value: DotenvError) -> Self {
        AppError::Dotenv(value)
    }
}

impl From<SerdeJsonError> for AppError {
    /// `serde_json::Error` を `AppError::Json` へ変換する。
    ///
    /// # Examples
    /// ```ignore
    /// fn to_app(err: serde_json::Error) -> AppError {
    ///     err.into()
    /// }
    /// # let _ = to_app;
    /// ```
    fn from(value: SerdeJsonError) -> Self {
        AppError::Json(value)
    }
}

impl From<CsvError> for AppError {
    /// `csv::Error` を `AppError::Csv` へ変換する。
    ///
    /// # Examples
    /// ```ignore
    /// fn to_app(err: csv::Error) -> AppError {
    ///     err.into()
    /// }
    /// # let _ = to_app;
    /// ```
    fn from(value: CsvError) -> Self {
        AppError::Csv(value)
    }
}

/// アプリ全体で利用する結果型の型エイリアス。
///
/// 成功時は `T`、失敗時は [`AppError`] を返す。
///
/// # Examples
/// ```ignore
/// fn parse_input() -> AppResult<u32> {
///     Ok(1)
/// }
/// # let _ = parse_input();
/// ```
pub type AppResult<T> = Result<T, AppError>;
