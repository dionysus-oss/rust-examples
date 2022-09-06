fn factorial(num: u64) -> u64 {
    fn f(num: u64, value: u64) -> u64 {
        if num == 0 {
            return value;
        }

        f(num - 1, num * value)
    }

    f(num, 1)
}

fn main() {
    println!("5! is {}", factorial(5));
    println!("10! is {}", factorial(10));
    println!("0! is {}", factorial(0));
}
