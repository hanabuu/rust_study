// Rustではデフォルトで標準ライブラリが利用可能であるが、それに入らないものはuseでスコープに入れる必要がある
// 標準のものをprelude(プレリュード)と呼ぶ
use std::io;        //標準ライブラリstdの中のioモジュールをスコープに入れる
use rand::Rng;      // randクレートのRngトレイトをスコープに入れる
use std::cmp::Ordering; // 比較用のOrdering列挙型をスコープに入れる

fn test1() {
    println!("数を当ててごらん");

    // 乱数を生成
    let secret_number = rand::thread_rng().gen_range(1..101);   //乱数生成器を取得し、1から100までの範囲で乱数を生成
                                                                //乱数生成期は現在のスレッドに固有で、オペレーティングシステムからシード値を得ています。
                                                                //secret_numberは指定がない限りi32型になる
    println!("秘密の数字は次の通り: {}", secret_number);    //乱数値を表示

    loop {
        println!("予想を入力してね");
        let mut guess = String::new();      // 可変(mutable)の変数guessを宣言、String::new()で新しい空の文字列を作成
        // let guess = "42";    // 変数guessを不変にする場合、再代入できないのでエラーになる mutで可変(immutable)にする

        // 標準入力から読み込み
        io::stdin()                             // ioの中のstdin関数を呼び出し、標準入力ハンドルを取得(useしてなかったらstd::io::stdin()で使える))
            .read_line(&mut guess)              // 標準入力から1行読み込み、guessに格納する。&で参照を渡す、mutで可変にする
            .expect("行の読み込みに失敗しました");  // 結果をResult型で返す、expectでエラー時のメッセージを指定
        
        // 文字列を数値に変換
        // let guess: u32 = guess.trim().parse()       // 変数guessをu32型に変換、shadowing(シャドーイング)で再宣言
        //     .expect("数値を入力してください！");         // expectでエラー出力してプログラムをクラッシュさせる。クラッシュせずに続ける場合は以下
        let guess: u32 = match guess.trim().parse() {   // 変数guessをu32型に変換、parse()の結果をmatchで処理
            Ok(num) => num,                             // 成功した場合、numを返す
            Err(_) => continue,                         // 失敗した場合、ループの先頭に戻る
        };
                                                    // trim()で前後の空白を削除、parse()で文字列を数値に変換
                                                    // シャドーイングは同じ名前で再宣言できる、型を変えたり可変/不変を変えたりできる
                                                    // 変数名を変えずに使い続けられるので便利
                                                    // ただし、シャドーイングは同じスコープ内でのみ有効、別のスコープでは元の変数が見える
                                                    // 型を変えるときによく使うらしい

        println!("次のように予想しました: {}", guess);

        match guess.cmp(&secret_number) {                       // 乱数値と入力値の比較(std::cmp::Orderingを使う)
            Ordering::Less => println!("小さすぎ！"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal => {
                println!("当たり！");
                break;  // ループを抜ける
            },
        }
    }
}

fn main() {
    test1();
}
