// 入出力に関する標準ライブラリ
use std::io;

fn main() {
    // println!は、文字列をコンソールに出力するマクロです。
    println!("Guess the number!");          // 数を当ててごらん

    println!("Please input your guess.");   // ほら、予想を入力してね

    // String::new()は、空の文字列を作成する関数です。
    // mutは可変な変数を宣言するためのキーワードです。
    let mut guess = String::new();

    io::stdin()
        // #mutは、可変な参照を作成するためのキーワードです。
        // read_lineは、標準入力から1行を読み込んで、result型の値を返すメソッドです。
        .read_line(&mut guess)
        // expectメソッドは、result型の値がエラーの場合に、指定したメッセージを表示してプログラムを終了するためのメソッドです。
        // ok値の場合は、読み込んだ行の数を返します。
        .expect("Failed to read line");     // 行の読み込みに失敗しました

    // {}はプレースホルダー(後で実際の値が入る仮の場所)
    println!("You guessed: {guess}");       // 次のように予想しました: {guess}
}