use ansi_term::{Colour::RGB, ANSIString};

#[derive(Debug)]
pub(crate) struct FormatCode {
    pub(crate) id:      String,
    pub(crate) code:    String,
    pub(crate) name_en: String,
    pub(crate) name_ja: String,
}

impl FormatCode {
    pub(crate) fn new (code: &str, name_en: &str, name_ja: &str) -> FormatCode {
        FormatCode {
            id: {
                let initial = name_en.chars().nth(0).unwrap().to_lowercase();
                format!("{}{}","x",initial.to_string())
            },
            code: code.to_string(),
            name_en: name_en.to_string(),
            name_ja: name_ja.to_string()
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
            id: {
                format!("{}{}", "y", code.chars().nth(1).unwrap())
            },
            code: code.to_string(),
            name: name.to_string(),
            colored_text: {
                // カラーコードに応じてnameに色付け
                let mut colored_text = RGB(0, 0, 0).paint(name);
                match code {
                    "§0" => colored_text = RGB(  0,   0,   0).paint(name),
                    "§1" => colored_text = RGB(  0,   0, 170).paint(name),
                    "§2" => colored_text = RGB(  0, 170,   0).paint(name),
                    "§3" => colored_text = RGB(  0, 170, 170).paint(name),
                    "§4" => colored_text = RGB(170,   0,   0).paint(name),
                    "§5" => colored_text = RGB(170,   0, 170).paint(name),
                    "§6" => colored_text = RGB(255, 170,   0).paint(name),
                    "§7" => colored_text = RGB(170, 170, 170).paint(name),
                    "§8" => colored_text = RGB( 85,  85,  85).paint(name),
                    "§9" => colored_text = RGB( 85,  85, 255).paint(name),
                    "§a" => colored_text = RGB( 85, 255,  85).paint(name),
                    "§b" => colored_text = RGB( 85, 255, 255).paint(name),
                    "§c" => colored_text = RGB(255,  85,  85).paint(name),
                    "§d" => colored_text = RGB(255,  85, 255).paint(name),
                    "§e" => colored_text = RGB(255, 255,  85).paint(name),
                    "§f" => colored_text = RGB(255, 255, 255).paint(name),
                    _ => {}
                }
                colored_text
            }
        }
    }
}