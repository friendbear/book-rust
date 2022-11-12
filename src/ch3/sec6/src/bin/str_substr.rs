fn main() {
    str_slice();
}
fn str_slice() {

    let pr = "知恵は武器よりも価値がある。";
    // 先頭2文字(6バイト)の部分文字列を得る
    println!("先頭2文字: {}", &pr[0..6]);
    
    // 「武器」の部分文字列を得る
    println!("4-5文字目: {}", &pr[9..15]);

    let pr_chars: Vec<char> = pr.chars().collect();

    let sub_chars = &pr_chars[3..=4];

    let sub4: String = sub_chars.into_iter().collect();
    println!("4-5文字目: {}", sub4);

}