use std::env;

fn main() {
    // コマンドライン引数を収集します
    let args: Vec<String> = env::args().collect();
    println!("arg len {}", args.len());
    let query = &args[1];
    let filename = &args[2];

    // {}を探しています
    println!("arg[1] {}", query);
    // {}というファイルの中
    println!("arg[2] {}", filename);
}