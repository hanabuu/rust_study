use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            print_usage(&args[0]);
        }
        "-f" => {
            if args.len() != 3 {
                eprintln!("-f にはファイルパスを1つ指定してください。");
                print_usage(&args[0]);
                std::process::exit(1);
            }

            let file = &args[2];
            let bytes = fs::read(file).map_err(|error| {
                eprintln!("ファイルの読み込みに失敗: {}", error);
                error
            })?;
            let digest = md5::compute(bytes);
            println!("MD5 : {:x}", digest);
        }
        "-c" => {
            if args.len() != 4 {
                eprintln!("-c には比較するハッシュ文字列を2つ指定してください。");
                print_usage(&args[0]);
                std::process::exit(1);
            }

            let hash1 = &args[2];
            let hash2 = &args[3];

            if hash1 == hash2 {
                println!("結果: 同じです");
            } else {
                println!("結果: 異なります");
            }
        }
        _ => {
            eprintln!("不正なオプションです: {}", args[1]);
            print_usage(&args[0]);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn print_usage(program: &str) {
    eprintln!("使い方:");
    eprintln!("  {} -f <filepath>", program);
    eprintln!("  {} -c <hash1> <hash2>", program);
}