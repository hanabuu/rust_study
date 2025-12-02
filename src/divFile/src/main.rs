mod constants; // 同じディレクトリの constants.rs をモジュールとして読み込む

fn main() {
    println!("{}: {}", constants::APP_NAME, constants::GREETING);
    println!("Docs: {}", constants::URL_DOCS);
}