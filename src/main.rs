use std::io;
use std::collections::HashMap;

fn main() {
    //変換する文字列を入力する処理
    println!("変換したい文字列を入力してください。：");
    //TODO: 1文字ずつor連続文
    let target_str = {
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("文字列の読み込みに失敗しました。処理を終了します。");
        s.trim_end().to_owned() //改行コード及び空白だけの場合それらを削除する
    };
    if  {target_str.is_empty()} {
        println!("文字列が入力されていません。処理を終了します。");
        return
    }

    let decorationcode = {
        let mut m = HashMap::new();
        m.insert("§k", "Obfuscated");
        m.insert("§l", "Bold");
        m.insert("§m", "Strikethrough");
        m.insert("§n", "Underline");
        m.insert("§o", "Italic");
        m.insert("§r", "Reset");
        m
    };

    let color_code = {
        let mut m = HashMap::new();
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

}
