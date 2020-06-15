extern crate strum;
#[macro_use]
extern crate strum_macros;

mod color_code;
mod format_code;
mod util;

use crate::util::search_fmt_code;
use color_code::ColorCode;
use format_code::FormatCode;
use util::{change_code_page_utf8, pause, read_texts};

fn main() {
    change_code_page_utf8();

    let format_codes = FormatCode::gen_from_enum();
    let color_codes = ColorCode::gen_from_enum();

    //TODO target_strやtarget_code、slitted_target_codeに対する、OptiomやResultを使ったエラー処理実装

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
        for fmt_code in &format_codes {
            println!(
                " {} -> {}：{}",
                fmt_code.id, fmt_code.name_en, fmt_code.name_ja
            );
        }
        println!();
        println!("==カラーコード一覧 / Color Codes==");
        for col_code in &color_codes {
            println!(" {} -> {}", col_code.id, col_code.colored_text);
        }
        pause();
        return;
    }

    // 入力コードを2文字ずつ分割
    let slitted_target_code = {
        let chars: Vec<char> = target_code.chars().collect();
        chars
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
    };

    let mut found_code = String::new();
    for k in &slitted_target_code {
        found_code = match search_fmt_code(k.to_owned(), found_code) {
            Ok(n) => n,
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
        };
    }

    println!("{}{}", found_code, target_str);
}
