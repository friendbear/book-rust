
fn main() {
    let height_cm = input_value();
    let weight = input_value();

    let height = height_cm/100.0;
    let bmi = weight/height.powf(2.0);

    println!("BMI={:1}", bmi)
}

fn input_value() -> f64 {

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("input error.");

    buf.trim().to_owned().parse::<f64>().expect("parse error.")

}