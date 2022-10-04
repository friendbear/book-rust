fn main() {

    for i in 1..101 {
        let res = match i {
            x if x % 3 == 0 && x & 5 == 0 => {
                "FizzBuzz"
            }
            y if y % 3 == 0 => {
                "Fizz"
            }
            z if z % 5 == 0 => {
                "Buzz"
            }
            _ => {
                ""
            }
        };
        println!("{} = {:?}", i, res)
    }
}