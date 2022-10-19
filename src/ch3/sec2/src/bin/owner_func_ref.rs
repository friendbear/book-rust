fn main() {
    let s = String::from("山椒の実");
    show_message(&s);
    println!("{}", s)
}

fn show_message(s: &String) {
    println!("{}", s)
}