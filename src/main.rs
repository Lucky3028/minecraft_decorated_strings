extern crate strum;
#[macro_use]
extern crate strum_macros;

mod color_code;
mod format_code;
mod util;

use color_code::ColorCode;
use format_code::FormatCode;
use itertools::Itertools;
use util::*;

fn main() {
    change_code_page_utf8();

    println!("変換したい文字列を入力してください。/ Enter texts you want to decorate：");
    let target_str = read_texts();
    if target_str.is_empty() {
        println!("文字列が入力されていません。/ No texts are entered.");
        return;
    }

    println!("次のコードが利用できます。/ The codes below are available.");
    println!("==装飾コード一覧 / Format Codes==");
    for f in FormatCode::gen_from_enum() {
        println!(" {} -> {}：{}", f.id, f.name_en, f.name_ja);
    }
    println!("==カラーコード一覧 / Color Codes==");
    for c in ColorCode::gen_from_enum() {
        println!(" {} -> {}", c.id, c.colored_text);
    }

    println!("装飾コードを入力してください。/ Enter the decoration codes.");
    println!("不要な場合はそのままEnterを入力してください。/ If unnecessary, press Enter key.");
    let target_format_code = read_texts().to_lowercase();

    println!("カラーコードを入力してください。/ Enter the color code.");
    println!("不要な場合はそのままEnterを入力してください。/ If unnecessary, press Enter key.");
    let target_color_code = read_texts().to_lowercase();

    if target_format_code.is_empty() && target_color_code.is_empty() {
        println!("どちらのコードも入力されませんでした。/ Neither codes are entered.");
        return;
    }

    let mut found_format_code = String::new();
    if !target_format_code.is_empty() {
        // 入力された装飾コードを2文字ずつ分割
        let target_format_code = {
            let chars: Vec<char> = target_format_code.chars().collect();
            chars
                .chunks(2)
                .map(|chunk| chunk.iter().collect::<String>())
                .unique()
                .collect::<Vec<_>>()
        };

        for f in &target_format_code {
            found_format_code = match compare_format_id_and_code(f.to_owned(), found_format_code) {
                Ok(n) => n,
                Err(err) => {
                    println!("Error: {}", err);
                    return;
                }
            };
        }
    }

    let mut found_color_code = String::new();
    if !target_color_code.is_empty() {
        if target_color_code.len() != 2 {
            println!("カラーコードは1つのみ指定できます。/ Only 1 color code can be specified.");
            return;
        }

        found_color_code = match compare_color_id_and_code(target_color_code, found_color_code) {
            Ok(n) => n,
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
        };
    }

    println!("{}{}{}", &found_color_code, &found_format_code, &target_str);
    println!(
        "{}{}{}",
        replace_section_to_json(found_color_code),
        replace_section_to_json(found_format_code),
        target_str
    );

    pause();
}
