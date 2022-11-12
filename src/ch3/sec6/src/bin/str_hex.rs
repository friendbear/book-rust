fn main() {
    hex_dump("自分の口を見張る人は自分の命を守る。");
    str_slice();
}

fn hex_dump(s: &str) {

    for (i, c) in s.bytes().enumerate() {

        // アドレスを表示
        if i % 16 == 0 {
            print!("{:08x}|", i);
        }

        // 4桁ごとに区切り文字を入れる
        if i % 4 == 3 {
            print!("{:02x}|", c);
        } else {
            print!("{:02x} ", c);
        }
        // 16バイトごとに改行を入れる
        if i % 16 == 15 {
            println!("");
        }
    }
    println!("");
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