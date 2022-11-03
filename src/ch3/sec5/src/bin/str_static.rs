fn echo(s: &'static str) {
    println!("{}", s)
}

fn main() {
    //echo(String::new()); // -> error
    //echo(String::from("ライフタイムエラー")); // -> error

    echo("Sucsess");
    echo("lifetime Sucsess")
}