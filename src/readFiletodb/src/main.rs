mod db;
mod error;
mod config;
mod file_scanner;
mod io;
mod processing;

use std::io::Write;
use std::path::Path;

use db::Database;
use error::{AppResult};

use crate::config::AppConfig;
use crate::file_scanner::FileScanner;
use crate::processing::ProcessingPipeline;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    // configsを使い、環境変数を設定する
    let config = AppConfig::load_from_default()?;

    // データベース接続と初期化
    let db_path = Path::new(&config.database_path);    // データベースパスをPath型に変換
    let mut database = Database::open(db_path)?;    // データベースを開く
    database.initialize_schema()?;                  // スキーマを初期化

    // dataフォルダ内を読込、ファイル一覧の取得とファイルリスト構造体へ格納する
    let scanner = FileScanner::new(&config);
    let file_entries = scanner.collect()?;

    let total = file_entries.len();
    let stdout = std::io::stdout();
    let pipeline = ProcessingPipeline::new(&database);
    for (i, entry) in file_entries.iter().enumerate() {
        let file_name = entry.path.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        print!("\r\x1B[K[{}/{} {}]", i + 1, total, file_name);
        println!("{}/{} {}", i + 1, total, file_name);
        stdout.lock().flush().ok();
        pipeline.process(entry)?;
    }
    println!();

    Ok(())
}