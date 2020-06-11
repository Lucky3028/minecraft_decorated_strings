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
pub(crate) struct ColorCode {
    pub(crate) id: String,
    pub(crate) code: String,
    pub(crate) name: String,
}

impl ColorCode {
    pub(crate) fn new (code: &str, name: &str) -> ColorCode {
        ColorCode {
            id: {
                format!("{}{}", "y", code.chars().nth(1).unwrap())
            },
            code: code.to_string(),
            name: name.to_string()
        }
    }
}