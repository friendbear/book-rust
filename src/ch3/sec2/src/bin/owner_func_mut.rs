fn add_quote(str: &mut String) {
    str.insert(0, '「');
    str.push('」');
}

fn main() {
    let mut msg = String::from("強い心は病気の支えになる");
    println!("{}", msg);
    add_quote(&mut msg);
    println!("{}", msg);
}