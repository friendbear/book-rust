fn main() {
    let ss = "気前よくな与えるから豊かになる";

    let so1 = String::from(ss);
    let so2 = ss.to_string();

    let ss2: &str = &so1;
    let ss3 = so1.as_str();

    println!("{} {} {} {}", so1, so2, ss2, ss3);
    println!("{:p} {:p}", ss2, ss3);
}