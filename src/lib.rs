use std::io;
use std::io::{stdin, stdout, Read, Write};

/// 何かしらのキーが押されるまで待機する
pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

/// 文字列を入力させ読み取る
pub fn read_texts() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("文字列の読み込みに失敗しました。処理を終了します。");
    //改行コードとスペースを削除する
    s.trim_end().to_string()
}