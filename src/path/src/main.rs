// [Rustで指定ディレクトリ内の全てのファイルのパスを取得する](https://qiita.com/Kanahiro/items/44cbc69a5d8849c8f00f)
use std::error::Error;
use read_dir::{read_dir, read_dir_recursive};
use std::path;

/// `read_dir` と `read_dir_recursive` を呼び出し、
/// それぞれの結果を標準出力へ表示します。
fn main() {
    // 単一ディレクトリ内のみ
    // let files = read_dir("../").unwrap();
    // for file in files {
    //     println!("{}", file.display());
    // }
    // 子ディレクトリも含めてすべてのファイルの列挙
    let recursive_file = read_dir_recursive(path::Path::new("./testDir")).unwrap();
    for file in recursive_file {
        println!("{}", file.display());
    }
}