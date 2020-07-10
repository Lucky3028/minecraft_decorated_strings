enum Lang {
    Ja,
    En,
}

pub enum MsgType {
    InductionInput,
    InductionFmtCodeInput,
    InductionClrCodeInput,
    InductionUnnecessaryCodeInput,
    InfoAvailable,
    InfoClrCodeTableTtl,
    InfoFmtCodeTableTtl,
    ErrNoTextInput,
    ErrNeitherCodeInput,
    ErrTooMuchOrLittleClrCodeInput,
    ErrClrCodeNotFound,
    ErrFmtCodeNotFound,
    FailureCHCP,
    FailureReadLine,
}

pub fn get_msg(kind: MsgType) -> String {
    let lang = match &*locate_locale::user() {
        "ja-JP" => Lang::Ja,
        _ => Lang::En,
    };

    // 返り値
    match kind {
        MsgType::InductionInput => match lang {
            Lang::Ja => "変換したい文字列を入力してください。",
            Lang::En => "Enter texts you want to decorate.",
        },
        MsgType::InductionFmtCodeInput => match lang {
            Lang::Ja => "装飾コードを入力してください。",
            Lang::En => "Enter the decoration codes.",
        },
        MsgType::InductionClrCodeInput => match lang {
            Lang::Ja => "カラーコードを入力してください。",
            Lang::En => "Enter the color code.",
        },
        MsgType::InductionUnnecessaryCodeInput => match lang {
            Lang::Ja => "不要な場合はそのままEnterを入力してください。",
            Lang::En => "If unnecessary, press Enter key.",
        },
        MsgType::InfoAvailable => match lang {
            Lang::Ja => "次のコードが利用できます。",
            Lang::En => "The codes below are available.",
        },
        MsgType::InfoClrCodeTableTtl => match lang {
            Lang::Ja => "カラーコード一覧",
            Lang::En => "Color Code Table",
        },
        MsgType::InfoFmtCodeTableTtl => match lang {
            Lang::Ja => "装飾コード一覧",
            Lang::En => "Format Code Table",
        },
        MsgType::ErrNoTextInput => match lang {
            Lang::Ja => "文字列が入力されていません。",
            Lang::En => "There is no text input.",
        },
        MsgType::ErrNeitherCodeInput => match lang {
            Lang::Ja => "どちらのコードも入力されていません。",
            Lang::En => "There is no code input.",
        },
        MsgType::ErrTooMuchOrLittleClrCodeInput => match lang {
            Lang::Ja => "カラーコードは1つのみ指定できます。",
            Lang::En => "Only 1 color code can be specified.",
        },
        MsgType::ErrClrCodeNotFound => match lang {
            Lang::Ja => "指定されたカラーコードが見つかりませんでした。",
            Lang::En => "Can't find the specified color codes.",
        },
        MsgType::ErrFmtCodeNotFound => match lang {
            Lang::Ja => "指定された装飾コードが見つかりませんでした。",
            Lang::En => "Can't find the specified format codes.",
        },
        MsgType::FailureCHCP => match lang {
            Lang::Ja => "文字コードの変更に失敗しました。",
            Lang::En => "Failed to change code page.",
        },
        MsgType::FailureReadLine => match lang {
            Lang::Ja => "入力された文字列の処理中にエラーが発生しました。",
            Lang::En => "Failed to read texts.",
        },
    }
    .to_string()
}
