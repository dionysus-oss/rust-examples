use std::collections::HashMap;

fn mean(arr: &Vec<i32>) -> f64 {
    let mut sum: f64 = 0.0;
    for el in arr {
        sum += *el as f64
    }

    sum / (arr.len() as f64)
}

fn median(arr: &Vec<i32>) -> i32 {
    arr[((arr.len() as f64) / 2.0).ceil() as usize]
}

fn mode(arr: &Vec<i32>) -> i32 {
    let mut keyed = HashMap::new();
    for el in arr {
        if keyed.contains_key(el) {
            keyed.insert(el, keyed[el] + 1);
        } else {
            keyed.insert(el, 1);
        }
    }

    let mut mode = i32::MIN;
    let mut max_count = 0;
    for (k, v) in keyed {
        if v > max_count {
            mode = *k;
            max_count = v;
        }
    }

    mode
}

fn main() {
    let mut test_list = vec![1, 4, 6, 2, 4, 2, 4];
    println!("The mean of list {:?} is {:.3}", test_list, mean(&test_list));

    test_list.sort();
    println!("The median of list {:?} is {}", test_list, median(&test_list));

    test_list.sort();
    println!("The mode of list {:?} is {}", test_list, mode(&test_list));
}
