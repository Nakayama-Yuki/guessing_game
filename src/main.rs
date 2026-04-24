// randは乱数を生成するためのクレート
// std::cmp::Orderingは、値の順序を表す列挙型を提供するモジュール
// std::ioは、入力と出力を扱うためのモジュール
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println!は、文字列をコンソールに出力するマクロ
    println!("Guess the number!");

    // rand::thread_rng()は、スレッドごとの乱数生成器を返す関数です。
    // gen_range(1..=100)は、1から100までの範囲の乱数を生成するメソッドです。
    // 生成した乱数をsecret_numberに代入しています。
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // プレイヤーに予想した数字の入力をうながします。
        println!("Please input your guess.");

        // String::new()は、空の文字列を作成する関数です。
        let mut guess = String::new();

        io::stdin()
            // &は、可変な参照を作成するためのキーワードです。
            // read_lineは、標準入力から1行を読み込んで、result型の値を返すメソッドです。
            .read_line(&mut guess)
            // expectメソッドは、result型の値がエラーの場合に、指定したメッセージを表示してプログラムを終了するためのメソッドです。
            // ok値の場合は、読み込んだ行の数を返します。
            .expect("Failed to read line");

        // trim()で改行を取り除き、parse()で文字列を数値に変換します。
        // 数値に変換できなかったときはcontinueで次のループに進みます。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {}はプレースホルダー(後で実際の値が入る仮の場所)
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  //小さすぎ！
            Ordering::Greater => println!("Too big!"), //大きすぎ！
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
