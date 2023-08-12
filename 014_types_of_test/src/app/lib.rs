pub fn count_substrings<T>(find: String, find_in: T) -> u32
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

pub fn run_with_strings<T>(mut strings: T) -> u32
where
    T: Iterator<Item = String>,
{
    match strings.next() {
        Some(first) => count_substrings(first, strings),
        None => {
            eprintln!("Requires at least one argument");
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::count_substrings;

    #[test]
    fn test_count_substrings() {
        let count = count_substrings("of".to_string(), ["professor".to_string()].into_iter());
        assert_eq!(count, 1);
    }
}
