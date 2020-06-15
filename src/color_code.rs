use super::util::paint_txt;

use std::fmt::Debug;
use strum::{EnumProperty, IntoEnumIterator};

#[derive(EnumProperty, EnumIter, Debug)]
pub enum ClrCode {
    #[strum(props(code = "§9", rgb_r = "85", rgb_g = "85", rgb_b = "255"))]
    Blue,
    #[strum(props(code = "§1", rgb_r = "0", rgb_g = "0", rgb_b = "170"))]
    DarkBlue,
    #[strum(props(code = "§a", rgb_r = "85", rgb_g = "255", rgb_b = "85"))]
    Green,
    #[strum(props(code = "§2", rgb_r = "0", rgb_g = "170", rgb_b = "0"))]
    DarkGreen,
    #[strum(props(code = "§b", rgb_r = "85", rgb_g = "255", rgb_b = "255"))]
    Aqua,
    #[strum(props(code = "§3", rgb_r = "0", rgb_g = "170", rgb_b = "170"))]
    DarkAqua,
    #[strum(props(code = "§c", rgb_r = "255", rgb_g = "85", rgb_b = "85"))]
    Red,
    #[strum(props(code = "§4", rgb_r = "170", rgb_g = "0", rgb_b = "170"))]
    DarkRed,
    #[strum(props(code = "§d", rgb_r = "255", rgb_g = "85", rgb_b = "255"))]
    LightPurple,
    #[strum(props(code = "§5", rgb_r = "170", rgb_g = "0", rgb_b = "170"))]
    DarkPurple,
    #[strum(props(code = "§7", rgb_r = "170", rgb_g = "170", rgb_b = "170"))]
    Gray,
    #[strum(props(code = "§8", rgb_r = "85", rgb_g = "85", rgb_b = "85"))]
    DarkGray,
    #[strum(props(code = "§6", rgb_r = "255", rgb_g = "170", rgb_b = "0"))]
    Gold,
    #[strum(props(code = "§e", rgb_r = "255", rgb_g = "255", rgb_b = "85"))]
    Yellow,
    #[strum(props(code = "§f", rgb_r = "255", rgb_g = "255", rgb_b = "255"))]
    White,
    #[strum(props(code = "§0", rgb_r = "0", rgb_g = "0", rgb_b = "0"))]
    Black,
}

#[derive(Debug)]
pub struct ColorCode {
    pub(crate) id: String,
    pub(crate) code: String,
    pub(crate) name: String,
    pub(crate) colored_text: String,
}

impl ColorCode {
    fn new(code: String, name: String, rgb_r: u8, rgb_g: u8, rgb_b: u8) -> Self {
        Self {
            id: { format!("{}{}", "y", code.chars().nth(1).unwrap()) },
            code,
            name: name.to_owned(),
            colored_text: paint_txt(rgb_r, rgb_g, rgb_b, name),
        }
    }

    // コード側で何を追加するのか決定するため、get_strやparseはunwrapする

    pub(crate) fn gen_from_enum() -> Vec<Self> {
        let mut ret: Vec<Self> = Vec::new();
        for k in ClrCode::iter() {
            ret.push(Self::new(
                k.get_str("code").unwrap().to_string(),
                format!("{:?}", k),
                k.get_str("rgb_r").unwrap().parse::<u8>().unwrap(),
                k.get_str("rgb_g").unwrap().parse::<u8>().unwrap(),
                k.get_str("rgb_b").unwrap().parse::<u8>().unwrap(),
            ));
        }
        ret
    }
}
