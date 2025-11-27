// [Rustで指定ディレクトリ内の全てのファイルのパスを取得する](https://qiita.com/Kanahiro/items/44cbc69a5d8849c8f00f)
use std::error::Error;
use std::fs;
use std::path;

/// 指定したディレクトリ直下のファイル／サブディレクトリを列挙して返します。
/// 
/// # 引数
/// * `path` - 走査対象となるディレクトリのパス文字列。
///
/// # 戻り値
/// `PathBuf` のベクタを含む `Result`。
///
/// # エラー
/// 指定ディレクトリが存在しない、または読み取り権限が無い場合にはエラーを返します。
fn read_dir(path: &str) -> Result<Vec<path::PathBuf>, Box<dyn Error>> {
    // dirはstd::fs::ReadDir型. into_iter()によりイテレーターとしてパスを順次取得可能
    // [fs::ReadDir](https://doc.rust-lang.org/std/fs/struct.ReadDir.html)
    let dir: fs::ReadDir = fs::read_dir(path)?;
    let mut files: Vec<path::PathBuf> = Vec::new();
    for item in dir.into_iter() {
        // itemはfs::DirEntry型.path()によりpath::PathBuf型でパスを取り出せる
        // [fs::DirEntry](https://doc.rust-lang.org/std/fs/struct.DirEntry.html)
        files.push(item?.path());
    }
    Ok(files)
}

/// ディレクトリ配下の全ファイルを再帰的に列挙して返します。
///
/// # 引数
/// * `path` - 再帰走査を開始するルートディレクトリ。
///
/// # 戻り値
/// 配下の全ファイル `PathBuf` を収集した `Result`。
///
/// # エラー
/// 途中のディレクトリでアクセス不能な場合などにエラーを返します。
fn read_dir_recursive(path: &path::Path) -> Result<Vec<path::PathBuf>, Box<dyn Error>> {
    let mut collected = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            collected.extend(read_dir_recursive(&entry_path)?);
        } else {
            collected.push(entry_path);
        }
    }
    Ok(collected)
}

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