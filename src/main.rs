mod format_code;
mod util;

use format_code::{ColorCode, FormatCode};
use std::process::Command;
use util::{pause, read_texts};

fn main() {
    // 文字コードをUS-ASCIIにする
    let _ = Command::new("cmd.exe")
        .arg("/c")
        .arg("chcp")
        .arg("20127")
        .status();

    let format_code = vec![
        FormatCode::new("§l", "Bold", "太字"),
        FormatCode::new("§o", "Italic", "斜め"),
        FormatCode::new("§n", "Underline", "下線"),
        FormatCode::new("§k", "Obfuscated", "難読化"),
        FormatCode::new("§m", "Strike through", "取り消し線"),
        FormatCode::new("§r", "Reset", "文字リセット"),
    ];

    let color_code = vec![
        ColorCode::new("§9", "Blue"),
        ColorCode::new("§1", "Dark Blue"),
        ColorCode::new("§a", "Green"),
        ColorCode::new("§2", "Dark Green"),
        ColorCode::new("§b", "Aqua"),
        ColorCode::new("§3", "Dark Aqua"),
        ColorCode::new("§c", "Red"),
        ColorCode::new("§4", "Dark Red"),
        ColorCode::new("§d", "Light Purple"),
        ColorCode::new("§5", "Dark Purple"),
        ColorCode::new("§7", "Gray"),
        ColorCode::new("§8", "Dark Gray"),
        ColorCode::new("§6", "Gold"),
        ColorCode::new("§e", "Yellow"),
        ColorCode::new("§f", "White"),
        ColorCode::new("§0", "Black"),
    ];

    println!("変換したい文字列を入力してください。：");
    //TODO: 1文字ずつor連続文
    let target_str = read_texts();
    if target_str.is_empty() {
        println!("文字列が入力されていません。処理を終了します。");
        return;
    }

    println!("コードを入力してください。なお、装飾コード、カラーコードの順に入力してください。（例：xbxiy1）");
    println!("また、コード一覧を見たい場合はhelpと入力してください。");
    let target_code = read_texts();
    if target_code.is_empty() {
        println!("文字列が入力されていません。処理を終了します。");
        return;
    }

    // helpサブコマンド処理
    if target_code == "help".to_string() {
        println!("==装飾コード一覧 / Format Codes==");
        for fmt_code in &format_code {
            println!(
                " {} -> {}：{}",
                fmt_code.id, fmt_code.name_en, fmt_code.name_ja
            );
        }
        println!();
        println!("==カラーコード一覧 / Color Codes==");
        for col_code in &color_code {
            println!(" {} -> {}", col_code.id, col_code.colored_text);
        }
        pause();
        return;
    }

    // 入力コードを2文字ずつ分割
    let splited_target_code = {
        let chars: Vec<char> = target_code.chars().collect();
        chars
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
    };
}
