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

    let format_codes = FormatCode::gen_from_enum();
    let color_codes = ColorCode::gen_from_enum();

    println!("変換したい文字列を入力してください。：");
    //TODO: 1文字ずつor連続文
    let target_str = read_texts();
    if target_str.is_empty() {
        println!("文字列が入力されていません。");
        return;
    }

    println!("次のコードが利用できます。");
    println!("==装飾コード一覧 / Format Codes==");
    for f in &format_codes {
        println!(" {} -> {}：{}", f.id, f.name_en, f.name_ja);
    }
    println!("==カラーコード一覧 / Color Codes==");
    for c in &color_codes {
        println!(" {} -> {}", c.id, c.colored_text);
    }

    println!("装飾コードを入力してください。");
    println!("不要な場合はそのままEnterを入力してください。");
    let target_format_code = read_texts().to_lowercase();
    println!("カラーコードを入力してください。");
    println!("不要な場合はそのままEnterを入力してください。");
    let target_color_code = read_texts().to_lowercase();
    if target_format_code.is_empty() && target_color_code.is_empty() {
        println!("どちらのコードも入力されませんでした。");
        return;
    }

    // 入力コードをそれぞれ2文字ずつ分割
    let slitted_target_format_code = {
        let chars: Vec<char> = target_format_code.chars().collect();
        chars
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .unique()
            .collect::<Vec<_>>()
    };
    let slitted_target_color_code = {
        let chars: Vec<char> = target_color_code.chars().collect();
        chars
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .unique()
            .collect::<Vec<_>>()
    };

    let mut found_format_code = String::new();
    for f in &slitted_target_format_code {
        found_format_code = match compare_format_id_and_code(f.to_owned(), found_format_code) {
            Ok(n) => n,
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
        };
    }
    let mut found_color_code = String::new();
    for f in &slitted_target_color_code {
        found_color_code = match compare_color_id_and_code(f.to_owned(), found_color_code) {
            Ok(n) => n,
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
        };
    }

    println!("{}{}{}", &found_format_code, &found_color_code, &target_str);
    println!(
        "{}{}{}",
        found_format_code.replace("§", r#"\u00a7"#),
        found_color_code.replace("§", r#"\u00a7"#),
        target_str
    );

    pause();
}
