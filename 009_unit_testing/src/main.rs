/**
 * Selects a slice from the array starting at start, taking len number of elements.
 * A sum of the positive elements from the slice is returned.
 */
fn slice_sum(arr: Vec<i32>, start: usize, len: usize) -> i32 {
    if start >= arr.len() || start + len > arr.len() {
        0
    } else {
        let end = start + len;
        arr[start..end].iter().filter(|x| **x > 0).sum()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::slice_sum;

    #[test]
    fn test_works() {
        assert_eq!(slice_sum(vec![1, 2], 0, 2), 3);
    }

    #[test]
    fn test_ignores_negative_numbers() {
        assert_eq!(slice_sum(vec![1, -5, 2], 0, 3), 3);
    }

    #[test]
    fn test_handles_empty_list() {
        assert_eq!(slice_sum(vec![], 0, 0), 0);
    }

    #[test]
    fn test_select_no_elements_from_non_empty_list() {
        assert_eq!(slice_sum(vec![1, 2, 3], 1, 0), 0);
    }

    #[test]
    fn test_handles_start_out_of_range() {
        assert_eq!(slice_sum(vec![1, 2, 3], 10, 1), 0);
    }

    #[test]
    fn test_handles_end_out_of_range() {
        assert_eq!(slice_sum(vec![1, 2, 3], 1, 10), 0);
    }
}
