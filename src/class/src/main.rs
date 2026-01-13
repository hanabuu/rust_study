
// モジュールの宣言
mod fruits;

use std::process;
// 構造体や関数のインポート(モジュール名と同じだとダメなので、モジュール名は小文字にする慣習)
use crate::fruits::Fruits;

fn main() {
    if let Err(error) = run() {
        eprintln!("アプリケーションエラー: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    let fruits = Fruits::new("Apple", "Red");
    let description = fruits.describe();
    println!("{}", description);
    Ok(())
}
