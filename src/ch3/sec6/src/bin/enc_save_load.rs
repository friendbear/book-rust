use std::io::Write;

use encoding_rs;

fn main() {

    let file_name = "test-sjis.txt";

    save_sjis(file_name, "こっそり食べるものは美味しい。");

    let s= load_sjis(file_name);
    println!("{}", s);
}

fn save_sjis(filename: &str, text: &str) {

    let (enc, _, _) = encoding_rs::SHIFT_JIS.encode(text);
    let buf = enc.into_owned();

    let mut file = std::fs::File::create(filename).expect("作成");
    file.write(&buf[..]).expect("書き込み");
}

fn load_sjis(filename: &str) -> String {
    let buf = std::fs::read(filename).expect("読み込み");

    // decode Shift_JIS
    let (dec, _, _) = encoding_rs::SHIFT_JIS.decode(&buf[..]);
    return dec.into_owned();

}