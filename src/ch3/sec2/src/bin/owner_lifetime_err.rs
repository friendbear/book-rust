fn gen_message() -> &'static str {
    let msg: &'static str = "ジェネレーター";
    return &msg
}

fn main() {
    let msg = gen_message();
    println!("{}", msg);
}