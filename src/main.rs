use std::io;
use std::io::{stdin, stdout, Read, Write, BufRead};
use std::collections::BTreeMap;
use ansi_term::Colour::RGB;

fn main() {
    let decoration_code = {
        let mut m = BTreeMap::new();
        m.insert("§l", ["Bold","太字"]);
        m.insert("§o", ["Italic", "斜め"]);
        m.insert("§n", ["Underline", "下線"]);
        m.insert("§k", ["Obfuscated", "難読化"]);
        m.insert("§m", ["Strike through", "取り消し線"]);
        m.insert("§r", ["Reset", "文字リセット"]);
        m
    };

    let color_code = {
        let mut m = BTreeMap::new();
        m.insert("§9", "Blue");
        m.insert("§1", "Dark Blue");
        m.insert("§a", "Green");
        m.insert("§2", "Dark Green");
        m.insert("§b", "Aqua");
        m.insert("§3", "Dark Aqua");
        m.insert("§c", "Red");
        m.insert("§4", "Dark Red");
        m.insert("§d", "Light Purple");
        m.insert("§5", "Dark Purple");
        m.insert("§7", "Gray");
        m.insert("§8", "Dark Gray");
        m.insert("§6", "Gold");
        m.insert("§e", "Yellow");
        m.insert("§f", "White");
        m.insert("§0", "Black");
        m
    };

    println!("変換したい文字列を入力してください。：");
    //TODO: 1文字ずつor連続文
    let target_str = read_texts();
    if  {target_str.is_empty()} {
        println!("文字列が入力されていません。処理を終了します。");
        return
    }

    println!("コードを入力してください。なお、装飾コード、カラーコードの順に入力してください。（例：bs02）");
    println!("また、コード一覧を見たい場合はhelpと入力してください。");
    let target_code = read_texts();
    if  {target_code.is_empty()} {
        println!("文字列が入力されていません。処理を終了します。");
        return
    }
}

fn read_texts() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("文字列の読み込みに失敗しました。処理を終了します。");
    s.trim_end().to_string()
}