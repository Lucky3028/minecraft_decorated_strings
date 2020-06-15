use super::color_code::ColorCode;
use super::format_code::FormatCode;
use std::io;
use std::io::{stdin, stdout, Read, Write};
use std::process::Command;

pub fn change_code_page_utf8() {
    Command::new("cmd.exe")
        .arg("/c")
        .arg("chcp")
        .arg("20127")
        .status()
        .expect("文字コードの変更に失敗しました。");
}

#[allow(clippy::unused_io_amount)]
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
        .expect("文字列の読み込みに失敗しました。");
    //改行コードとスペースを削除する
    s.trim_end().to_string()
}

/// カラーコードに応じてtextに色付け
pub fn paint_txt(rgb_r: u8, rgb_g: u8, rgb_b: u8, text: String) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[m", rgb_r, rgb_g, rgb_b, text)
}

pub fn search_fmt_code(target_str: String, already_code: String) -> Result<String, String> {
    let format_codes = FormatCode::gen_from_enum();
    let color_codes = ColorCode::gen_from_enum();

    match format_codes.iter().find(|&x| &target_str == &x.id) {
        Some(fmt) => Ok(format!("{}{}", already_code, fmt.code)),
        None => match color_codes.iter().find(|&x| &target_str == &x.id) {
            Some(clr) => Ok(format!("{}{}", already_code, clr.code)),
            None => Err("指定されたコードが見つかりませんでした。".to_owned()),
        },
    }
}
