use app::run_with_strings;

fn main() {
    let mut args = std::env::args().skip(1).into_iter();

    let num = run_with_strings(&mut args);

    println!("Count is {}", num);
}
