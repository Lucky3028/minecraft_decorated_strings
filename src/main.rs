extern crate strum;
#[macro_use]
extern crate strum_macros;

mod color_code;
mod format_code;
mod message;
mod util;

use color_code::ColorCode;
use format_code::FormatCode;
use itertools::Itertools;
use message::get_msg;
use message::MsgType::*;
use util::*;

fn main() {
    change_code_page_utf8();

    println!("{}", get_msg(InductionInput));
    let target_str = read_texts();
    if target_str.is_empty() {
        exit_program(get_msg(ErrNoTextInput));
    }

    println!("{}", get_msg(InfoAvailable));
    println!("=={}==", get_msg(InfoFmtCodeTableTtl));
    for f in FormatCode::gen_from_enum() {
        println!(" {} -> {}：{}", f.id, f.name_en, f.name_ja);
    }
    println!("=={}==", get_msg(InfoClrCodeTableTtl));
    for c in ColorCode::gen_from_enum() {
        println!(" {} -> {}", c.id, c.colored_text);
    }

    println!("{}", get_msg(InductionFmtCodeInput));
    println!("{}", get_msg(InductionUnnecessaryCodeInput));
    let target_format_code = read_texts().to_lowercase();

    println!("{}", get_msg(InductionClrCodeInput));
    println!("{}", get_msg(InductionUnnecessaryCodeInput));
    let target_color_code = read_texts().to_lowercase();

    if target_format_code.is_empty() && target_color_code.is_empty() {
        exit_program(get_msg(ErrNeitherCodeInput));
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
            found_format_code = match find_id_from_fmt_code(f.to_owned(), found_format_code) {
                Ok(n) => n,
                Err(err) => {
                    exit_program(err);
                    "ERR".to_string()
                }
            };
        }
    }

    let mut found_color_code = String::new();
    if !target_color_code.is_empty() {
        if target_color_code.len() != 2 {
            exit_program(get_msg(ErrTooMuchOrLittleClrCodeInput));
        }

        found_color_code = match find_id_from_clr_code(target_color_code, found_color_code) {
            Ok(n) => n,
            Err(err) => {
                exit_program(err);
                "ERR".to_string()
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
