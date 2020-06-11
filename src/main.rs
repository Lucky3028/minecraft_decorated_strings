mod util;
mod format_code;

use std::process::Command;
use util::{pause, read_texts};
use format_code::{FormatCode, ColorCode};

fn main() {
    //文字コードをUS-ASCIIにする
    let _ = Command::new("cmd.exe").arg("/c").arg("chcp").arg("20127").status();

    let format_code = {
        let mut v = Vec::new();
        v.push(FormatCode::new("§l", "Bold", "太字"));
        v.push(FormatCode::new("§o", "Italic", "斜め"));
        v.push(FormatCode::new("§n", "Underline", "下線"));
        v.push(FormatCode::new("§k", "Obfuscated", "難読化"));
        v.push(FormatCode::new("§m", "Strike through", "取り消し線"));
        v.push(FormatCode::new("§r", "Reset", "文字リセット"));
        v
    };

    let color_code = {
        let mut v = Vec::new();
        v.push(ColorCode::new("§9", "Blue"));
        v.push(ColorCode::new("§1", "Dark Blue"));
        v.push(ColorCode::new("§a", "Green"));
        v.push(ColorCode::new("§2", "Dark Green"));
        v.push(ColorCode::new("§b", "Aqua"));
        v.push(ColorCode::new("§3", "Dark Aqua"));
        v.push(ColorCode::new("§c", "Red"));
        v.push(ColorCode::new("§4", "Dark Red"));
        v.push(ColorCode::new("§d", "Light Purple"));
        v.push(ColorCode::new("§5", "Dark Purple"));
        v.push(ColorCode::new("§7", "Gray"));
        v.push(ColorCode::new("§8", "Dark Gray"));
        v.push(ColorCode::new("§6", "Gold"));
        v.push(ColorCode::new("§e", "Yellow"));
        v.push(ColorCode::new("§f", "White"));
        v.push(ColorCode::new("§0", "Black"));
        v
    };

    println!("変換したい文字列を入力してください。：");
    //TODO: 1文字ずつor連続文
    let target_str = read_texts();
    if  {target_str.is_empty()} {
        println!("文字列が入力されていません。処理を終了します。");
        return;
    }

    println!("コードを入力してください。なお、装飾コード、カラーコードの順に入力してください。（例：xbxiy1）");
    println!("また、コード一覧を見たい場合はhelpと入力してください。");
    let target_code = read_texts();
    if  {target_code.is_empty()} {
        println!("文字列が入力されていません。処理を終了します。");
        return;
    }

    //helpサブコマンド処理
    if {target_code == "help".to_string()} {
        println!("==装飾コード一覧 / Format Codes==");
        for fmt_code in &format_code {
            println!(" {} -> {}：{}", fmt_code.id, fmt_code.name_en, fmt_code.name_ja);
        }
        println!();
        println!("==カラーコード一覧 / Color Codes==");
        for col_code in &color_code {
            println!(" {} -> {}", col_code.id, col_code.colored_text);
        }
        pause();
        return;
    }

    let splited_target_code = {
        let chars: Vec<char> = target_code.chars().collect();
        let ret = chars.chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>();
        ret
    };

}