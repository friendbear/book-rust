fn main() {

    let s = String::from("過ちを犯す人は美しい");
    show_message(&s);
    println!("{}", s);
}

fn show_message(s: &String) {
    println!("{}", s);
}