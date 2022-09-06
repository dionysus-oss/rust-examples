fn factorial(num: i32) -> i32 {
    if num < 1 {
        return 1
    }

    return num * factorial(num - 1)
}

fn main() {
    println!("5! is {}", factorial(5));
    println!("10! is {}", factorial(10));
}
