//! ファイル種別ごとの取り込み前処理をまとめるモジュールです。
//!
//! `ProcessingPipeline` は `FileEntry` の種別を見て適切な読み込み関数へ振り分け、
//! 後続のDB登録や集計処理へ渡せる状態まで入力データを検証します。

use crate::db::Database;
use crate::error::{AppError, AppResult};
use crate::file_scanner::{FileEntry, FileKind};
use crate::io::{csv, json, text};
use serde_json::Value;

/// スキャン済みファイルを種別ごとに処理するパイプラインです。
pub struct ProcessingPipeline<'a> {
    // 各入力を最終的に保存する SQLite 接続。
    db: &'a Database,
}

impl<'a> ProcessingPipeline<'a> {
    /// データベース接続を受け取り、処理パイプラインを生成します。
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// ファイル種別に応じて対応する処理関数へ振り分けます。
    ///
    /// `entry.kind` に基づいて JSON、テキスト、CSV など適切な読み込み方式を選択します。
    pub fn process(&self, entry: &FileEntry) -> AppResult<()> {
        // スキャナで判定済みの論理種別を、そのまま専用処理へ委譲する。
        match entry.kind {
            FileKind::IDENTIFYJSONDATA => self.process_identify_json_data(entry),
            FileKind::SAMPLEJSONDATA => self.process_sample_json_data(entry),
            FileKind::CSVDATA => self.process_csv_data(entry),
            FileKind::TEXTDATA => self.process_text_data(entry),
             // ここに新しい種別の処理関数を追加していく。
       }
    }


    /// identifyData.json 系 JSON ファイルを読み込みます。
    fn process_identify_json_data(&self, entry: &FileEntry) -> AppResult<()> {
        tracing::info!(path = %entry.path.display(), "process identify_json_data");
        // let json_value: Value = json::read_json(entry.path.as_path())?;
        let identify = required_identify(entry)?;
        self.db.import_identify(identify)?;
        tracing::debug!(identify, "identify data parsed");
        Ok(())
    }
    /// sample_json_data 系 JSON ファイルを読み込みます。
    fn process_sample_json_data(&self, entry: &FileEntry) -> AppResult<()> {
        tracing::info!(path = %entry.path.display(), identify = ?entry.identify, "process sample_json_data");
        // sample_json_data は JSON 全体を読み込んだうえで、DB 層で各テーブルへ展開する。
        let json_value: Value = json::read_json(entry.path.as_path())?;
        let identify = required_identify(entry)?;
        self.db.import_sample_json(identify, &json_value)?;
        tracing::debug!(?json_value, "system data parsed");
        Ok(())
    }

    fn process_csv_data(&self, entry: &FileEntry) -> AppResult<()> {
        tracing::info!(path = %entry.path.display(), identify = ?entry.identify, "process csv_data");
        let rows = csv::read_shift_jis(entry.path.as_path(), true)?;
        self.db.import_csv_data(&rows)?;
        Ok(())
    }

    fn process_text_data(&self, entry: &FileEntry) -> AppResult<()> {
        tracing::info!(path = %entry.path.display(), identify = ?entry.identify, "process text_data");
        // テキストデータは現状、DB 登録の必要がないため、単純に内容をログ出力するだけに留める。
        // 識別子ありの場合
        let content = text::read_utf8_line_bytes(entry.path.as_path())?;
        self.db.import_text_data(entry.identify.as_deref().unwrap_or_default(), &content)?;
        // 識別子なしの場合
        let content2 = text::read_utf8_line_bytes(entry.path.as_path())?;
        self.db.import_text_data2( &content2)?;
        tracing::debug!(?content, "text data read");
        Ok(())
    }

}

/// identifyData.json 起点で付与された識別番号が必須の入力から、安全に identify を取り出します。
fn required_identify(entry: &FileEntry) -> AppResult<&str> {
    entry
        .identify
        .as_deref()
        .ok_or_else(|| AppError::InvalidData(format!("missing identify for {}", entry.path.display())))
}