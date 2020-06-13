use super::util::paint_txt;
use ansi_term::ANSIString;

#[derive(Debug)]
pub(crate) struct FormatCode {
    pub(crate) id: String,
    pub(crate) code: String,
    pub(crate) name_en: String,
    pub(crate) name_ja: String,
}

impl FormatCode {
    pub(crate) fn new(code: &str, name_en: &str, name_ja: &str) -> FormatCode {
        FormatCode {
            id: {
                let initial = name_en.chars().nth(0).unwrap().to_lowercase();
                format!("{}{}", "x", initial.to_string())
            },
            code: code.to_string(),
            name_en: name_en.to_string(),
            name_ja: name_ja.to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ColorCode<'a> {
    pub(crate) id: String,
    pub(crate) code: String,
    pub(crate) name: String,
    pub(crate) colored_text: ANSIString<'a>,
}

impl ColorCode<'_> {
    pub(crate) fn new<'a>(code: &'a str, name: &'a str) -> ColorCode<'a> {
        ColorCode {
            id: { format!("{}{}", "y", code.chars().nth(1).unwrap()) },
            code: code.to_string(),
            name: name.to_string(),
            colored_text: {
                match code {
                    "§0" => paint_txt(0, 0, 0, name),
                    "§1" => paint_txt(0, 0, 170, name),
                    "§2" => paint_txt(0, 170, 0, name),
                    "§3" => paint_txt(0, 170, 170, name),
                    "§4" => paint_txt(170, 0, 0, name),
                    "§5" => paint_txt(170, 0, 170, name),
                    "§6" => paint_txt(255, 170, 0, name),
                    "§7" => paint_txt(170, 170, 170, name),
                    "§8" => paint_txt(85, 85, 85, name),
                    "§9" => paint_txt(85, 85, 255, name),
                    "§a" => paint_txt(85, 255, 85, name),
                    "§b" => paint_txt(85, 255, 255, name),
                    "§c" => paint_txt(255, 85, 85, name),
                    "§d" => paint_txt(255, 85, 255, name),
                    "§e" => paint_txt(255, 255, 85, name),
                    "§f" => paint_txt(255, 255, 255, name),
                    _ => paint_txt(0, 0, 0, name),
                }
            },
        }
    }
}
