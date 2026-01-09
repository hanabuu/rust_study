use dotenvy::dotenv;
use std::env;

fn main() {
    // .envファイルから環境変数を読み込む
    dotenv().expect(".env file not found");

    // 個別にとるパターン(env::var())
    let app_version = env::var("APP_VERSION").expect("APP_VERSION not set");
    println!("App Version: {}", app_version);
    let app_prod = env::var("APP_PROD").unwrap_or("development".to_string());
    println!("App Prod: {}", app_prod);

    // 一括でとるパターン(env::vars())
    for (key, value) in env::vars() {
        if key.starts_with("APP_") {
            println!("{key}: {value}");
        }
    }
}