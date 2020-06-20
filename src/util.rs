use super::color_code::ColorCode;
use super::format_code::FormatCode;
use std::io;
use std::io::{stdin, stdout, Read, Write};
use std::process::Command;

/// UTF-8にコマンドラインの文字コードを変更する
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

//TODO: エラー処理
/// 文字列を入力させ読み取る
pub fn read_texts() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("文字列を読み取れませんでした。");
    //改行コードとスペースを削除する
    s.trim().to_string()
}

/// カラーコードに応じてtextに色を付ける
pub fn paint_txt(rgb_r: u8, rgb_g: u8, rgb_b: u8, text: String) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[m", rgb_r, rgb_g, rgb_b, text)
}

/// FormatCodeの中から該当するidを探す。見つからなければError型を生成する。
/// 見つかった場合は、既に存在する文字コードの後ろに文字コードを追加してそれを返す。
///
/// #Example
/// ```
/// use util.compare_format_id_and_code;
///
/// let s = compare_format_id_and_code("xb".to_string(), "§n".to_string());
/// assert_eq!(s, Ok("§n§l".to_string()));
/// ```
pub fn compare_format_id_and_code(
    target_str: String,
    already_code: String,
) -> Result<String, String> {
    let format_codes = FormatCode::gen_from_enum();

    match format_codes.iter().find(|&x| target_str == x.id) {
        Some(fmt) => Ok(format!("{}{}", already_code, fmt.code)),
        None => Err("指定された装飾コードが見つかりませんでした。".to_owned()),
    }
}

/// ColorCodeの中から該当するidを探す。見つからなければError型を生成する。
/// 見つかった場合は、既に存在する文字コードの後ろに文字コードを追加してそれを返す。
///
/// #Example
/// ```
/// use util.compare_color_id_and_code;
///
/// let s = compare_color_id_and_code("xb".to_string(), "§n".to_string());
/// assert_eq!(s, Ok("§n§l".to_string()));
/// ```
pub fn compare_color_id_and_code(
    target_str: String,
    already_code: String,
) -> Result<String, String> {
    let color_codes = ColorCode::gen_from_enum();

    match color_codes.iter().find(|&x| target_str == x.id) {
        Some(clr) => Ok(format!("{}{}", already_code, clr.code)),
        None => Err("指定されたカラーコードが見つかりませんでした。".to_owned()),
    }
}

///  節記号（§）をJsonのエスケープシーケンス（\u00a7）に置き換える
pub fn replace_section_to_json(target: String) -> String {
    target.replace("§", r#"\u00a7"#)
}
