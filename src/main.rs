mod util;
mod format_code;

use std::process::Command;
use ansi_term::{Colour::RGB, ANSIString};
use hashlink::LinkedHashMap;
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

    //カラーコードのうち、入力として、インデックスを使うための変数
    //ただし、1桁の場合は十の位を0埋め
    let col_code_id = {
        let mut vec = Vec::new();
        for i in 0..color_code.len() {
            vec.push(format!("{0: >02}", i));
        }
        vec
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
            println!(" {} -> {}",
                     col_code.id,
                     //英語説明文
                     make_text_colored(col_code.code.as_ref(), col_code.name.as_ref())
            );
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

fn make_text_colored<'a>(color_code: &'a str, plained_text: &'a str) -> ANSIString<'a> {
    let mut colored_text = RGB(0, 0, 0).paint(plained_text);
    match color_code {
        "§0" => colored_text = RGB(  0,   0,   0).paint(plained_text),
        "§1" => colored_text = RGB(  0,   0, 170).paint(plained_text),
        "§2" => colored_text = RGB(  0, 170,   0).paint(plained_text),
        "§3" => colored_text = RGB(  0, 170, 170).paint(plained_text),
        "§4" => colored_text = RGB(170,   0,   0).paint(plained_text),
        "§5" => colored_text = RGB(170,   0, 170).paint(plained_text),
        "§6" => colored_text = RGB(255, 170,   0).paint(plained_text),
        "§7" => colored_text = RGB(170, 170, 170).paint(plained_text),
        "§8" => colored_text = RGB( 85,  85,  85).paint(plained_text),
        "§9" => colored_text = RGB( 85,  85, 255).paint(plained_text),
        "§a" => colored_text = RGB( 85, 255,  85).paint(plained_text),
        "§b" => colored_text = RGB( 85, 255, 255).paint(plained_text),
        "§c" => colored_text = RGB(255,  85,  85).paint(plained_text),
        "§d" => colored_text = RGB(255,  85, 255).paint(plained_text),
        "§e" => colored_text = RGB(255, 255,  85).paint(plained_text),
        "§f" => colored_text = RGB(255, 255, 255).paint(plained_text),
        _ => {}
    }
    colored_text
}