use std::fs::File;
use std::io::{BufRead, BufReader};

fn flip_front(arr: &mut Vec<u32>, n: usize) {
    if n == 0 || n == 1 {
        return;
    }

    let mid = ((n as f32) / 2.0).ceil() as usize;
    for i in 0..mid {
        let swap_index = n - i - 1;

        let temp = arr[i];
        arr[i] = arr[swap_index];
        arr[swap_index] = temp;
    }
}

fn index_of_max(arr: &Vec<u32>, index_limit: usize) -> usize {
    let mut max = 0;
    let mut max_index = 0;
    for i in 0..=index_limit {
        if arr[i] > max {
            max = arr[i];
            max_index = i;
        }
    }

    max_index
}

fn index_of_max_v2(arr: &Vec<u32>, index_limit: usize) -> usize {
    let mut max = 0;
    let mut max_index = 0;
    for i in 0..=index_limit {
        if arr[i] >= max {
            max = arr[i];
            max_index = i;
        }
    }

    max_index
}

fn pancake_sort(arr: &mut Vec<u32>) -> usize {
    let mut calls_count = 0;

    let mut working_index = arr.len() - 1;
    while working_index > 0 {
        let max_index = index_of_max(arr, working_index);

        flip_front(arr, max_index + 1);
        flip_front(arr, working_index + 1);
        calls_count += 2;

        working_index -= 1;
    }

    calls_count
}

fn pancake_sort_v2(arr: &mut Vec<u32>) -> usize {
    let mut calls_count = 0;

    let mut working_index = arr.len() - 1;
    while working_index > 0 {
        let max_index = index_of_max_v2(arr, working_index);

        if max_index == working_index {
            working_index -= 1;
            continue;
        }

        if max_index != 0 {
            flip_front(arr, max_index + 1);
        } else {
            calls_count -= 1;
        }

        flip_front(arr, working_index + 1);
        calls_count += 2;

        working_index -= 1;
    }

    calls_count
}

fn main() {
    let mut arr = vec![1, 2, 3, 4, 5];
    flip_front(&mut arr, 3);
    println!("{:?}", arr);

    let mut sort_arr = vec![3, 1, 2, 1];
    let calls_count = pancake_sort(&mut sort_arr);

    println!("{:?}", sort_arr);
    println!("calls count {}", calls_count);

    let mut test_arr = load_test_input();
    let test_calls_count = pancake_sort(&mut test_arr);
    println!("test calls count {}", test_calls_count);
    let mut test_v2_arr = load_test_input();
    let test_v2_calls_count = pancake_sort_v2(&mut test_v2_arr);

    if test_arr != test_v2_arr {
        println!("arrays are not equal after sorting");
    }

    println!("test calls count {}", test_v2_calls_count);
}

fn load_test_input() -> Vec<u32> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input = vec![0; 10_000];

    let mut index = 0;
    for line in reader.lines() {
        input[index] = line.unwrap().parse::<u32>().unwrap();
        index += 1;
    }

    input
}
