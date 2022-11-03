fn main() {

    let s2 = "abcdefg";
    println!("{}", &s2[0..1]);

    let s = "こんにちわ";
    println!("{}", &s[..3]);

    let ch = &s[6..9];
    println!("{}", ch);
}