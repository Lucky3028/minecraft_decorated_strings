use std::io;

fn main() {
    //変換する文字列を入力する処理
    println!("変換したい文字列を入力してください。：");
    //TODO: 1文字ずつor連続文
    let target_str = {
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("文字列の読み込みに失敗しました。処理を終了します。");
        s.trim_end().to_owned() //改行コード及び空白だけの場合それらを削除する
    };
    if  {target_str.is_empty()} {
        println!("文字列が入力されていません。処理を終了します。");
        return
    }
}
