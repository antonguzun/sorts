extern crate rand;

use rand::distributions::Standard;
use std::time::Instant;

fn bubble_sort(array: &mut Vec<i32>) {
    let mut current_len = array.len() - 1;
    loop {
        let mut changed: bool = false;
        for i in 0..current_len {
            if i == current_len {
                break;
            }
            if array[i] > array[i + 1] {
                let temp: i32 = array[i];
                array[i] = array[i + 1];
                array[i + 1] = temp;
                changed = true;
            }
        }
        current_len = current_len - 1;
        if !changed {
            break;
        }
    }
}

fn sort_step(left_arr: Vec<i32>, right_arr: Vec<i32>) -> Vec<i32> {
    let mut array = Vec::new();
    let mut left_cur = 0;
    let mut right_cur = 0;

    for _i in 0..left_arr.len() + right_arr.len() {
        if left_cur == left_arr.len() {
            array.push(right_arr[right_cur]);
            right_cur += 1;
            continue;
        } else if right_cur == right_arr.len() {
            array.push(left_arr[left_cur]);
            left_cur += 1;
            continue;
        } else if left_arr[left_cur] < right_arr[right_cur] {
            array.push(left_arr[left_cur]);
            left_cur += 1;
        } else {
            array.push(right_arr[right_cur]);
            right_cur += 1;
        }
    }
    return array;
}

fn merge_sort(array: Vec<i32>) -> Vec<i32> {
    let length = array.len();
    if length < 2 {
        return array.to_vec();
    }
    let divider_index = length / 2;
    return sort_step(
        merge_sort(array[0..divider_index].to_vec()),
        merge_sort(array[divider_index..length].to_vec()),
    );
}

fn main() {
    let rng = thread_rng();
    let mut x: Vec<i32> = rng.sample_iter(Standard).take(100000).collect();
    let y = x.clone();
    let mut z = x.clone();

    let start = Instant::now();
    x.sort();
    let duration = start.elapsed();
    println!("Time elapsed in default sort: {:?}", duration);

    let start = Instant::now();
    let sorted_list = merge_sort(y);
    let duration = start.elapsed();
    println!("Time elapsed in merge_sort(): {:?}", duration);

    let start = Instant::now();
    bubble_sort(&mut z);
    let duration = start.elapsed();
    println!("Time elapsed in bubble_sort(): {:?}", duration);

    assert_eq!(x, sorted_list);
    assert_eq!(x, z);
}
