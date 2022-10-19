fn main() {

    let mut s = String::from("所有権を戻す関数に渡す文字列");
    s = show_message(s);
    println!("{}", s);
}

fn show_message(s: String) -> String {
    println!("{}", s);
    s
}