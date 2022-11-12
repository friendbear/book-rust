fn main() {
    let s = "隣の客はよく柿食う客だ";

    match s.find('柿') {
        Some(i) => println!("柿={}", &s[i..(i+3)]),
        None => println!("無し"),
    }
}