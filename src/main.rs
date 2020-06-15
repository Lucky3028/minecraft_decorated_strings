extern crate strum;
#[macro_use]
extern crate strum_macros;

mod color_code;
mod format_code;
mod util;

use color_code::{ClrCode, ColorCode};
use format_code::{FmtCode, FormatCode};
use std::process::Command;
use strum::{EnumProperty, IntoEnumIterator};
use util::{pause, read_texts};

fn main() {
    // 文字コードをUS-ASCIIにする
    let _ = Command::new("cmd.exe")
        .arg("/c")
        .arg("chcp")
        .arg("20127")
        .status();

    let mut format_code: Vec<FormatCode> = Vec::new();
    for k in FmtCode::iter() {
        format_code.push(FormatCode::new(
            k.get_str("code").unwrap().to_string(),
            format!("{:?}", k),
            k.get_str("name_ja").unwrap().to_string(),
        ));
    }

    let mut color_code: Vec<ColorCode> = Vec::new();
    for k in ClrCode::iter() {
        color_code.push(ColorCode::new(
            k.get_str("code").unwrap().to_string(),
            format!("{:?}", k),
            k.get_str("rgb_r").unwrap().parse::<u8>().unwrap(),
            k.get_str("rgb_g").unwrap().parse::<u8>().unwrap(),
            k.get_str("rgb_b").unwrap().parse::<u8>().unwrap(),
        ));
    }

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
    if target_code == "help" {
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
    let splitted_target_code = {
        let chars: Vec<char> = target_code.chars().collect();
        chars
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
    };

    for i in splited_target_code {
        println!("{}", i);
    }
}
