use std::fmt::Debug;
use strum::{EnumProperty, IntoEnumIterator};

#[derive(EnumProperty, EnumIter, Debug)]
pub enum FmtCode {
    #[strum(props(code = "§l", name_ja = "太字"))]
    Bold,
    #[strum(props(code = "§o", name_ja = "斜め"))]
    Italic,
    #[strum(props(code = "§n", name_ja = "下線"))]
    Underlin,
    #[strum(props(code = "§k", name_ja = "難読化"))]
    Obfuscated,
    #[strum(props(code = "§m", name_ja = "取り消し線"))]
    StrikeThrough,
    #[strum(props(code = "§r", name_ja = "リセット"))]
    Reset,
}

#[derive(Debug)]
pub struct FormatCode {
    pub(crate) id: String,
    pub(crate) code: String,
    pub(crate) name_en: String,
    pub(crate) name_ja: String,
}

impl FormatCode {
    fn new(code: String, name_en: String, name_ja: String) -> Self {
        Self {
            id: {
                let initial = name_en.chars().next().unwrap().to_lowercase();
                format!("{}{}", "x", initial.to_string())
            },
            code,
            name_en,
            name_ja,
        }
    }

    // コード側で何を追加するのか決定するため、get_strはunwrapする

    pub(crate) fn gen_from_enum() -> Vec<Self> {
        let mut ret: Vec<Self> = Vec::new();
        for k in FmtCode::iter() {
            ret.push(Self::new(
                k.get_str("code").unwrap().to_string(),
                format!("{:?}", k),
                k.get_str("name_ja").unwrap().to_string(),
            ));
        }
        ret
    }
}
