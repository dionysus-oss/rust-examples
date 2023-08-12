fn count_substrings<T>(find: String, find_in: T) -> u32
where
    T: Iterator<Item = String>,
{
    let mut count = 0;
    for s in find_in {
        if s.contains(&find) {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut args = std::env::args().skip(1);

    let num = match args.next() {
        Some(first) => count_substrings(first, args),
        None => {
            eprintln!("Requires at least one argument");
            0
        }
    };

    println!("Count is {}", num);
}

#[cfg(test)]
mod tests {
    use crate::count_substrings;

    #[test]
    fn test_count_substrings() {
        let count = count_substrings("of".to_string(), ["professor".to_string()].into_iter());
        assert_eq!(count, 1);
    }
}
