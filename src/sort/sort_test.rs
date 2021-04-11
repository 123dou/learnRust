use std::{thread, time};

use rand::prelude::*;

use crate::sort::sort;
const SIZE: i32 = 10_000;
#[test]
pub fn test_bubble_sort() {
    let mut arr = create_arr(SIZE);
    let start = time::Instant::now();
    sort::bubble_sort(&mut arr);
    let end = start.elapsed();
    println!("bubble sort total cost time: {:?}", end);
    if !check_order(&arr) {
        println!("{:?}", "bubble sort fail..");
    } else {
        println!("{:?}", "bubble sort pass!!!");
    }
}

#[test]
pub fn test_choice_sort() {
    let mut arr = create_arr(SIZE);
    let start = time::Instant::now();
    sort::choice_sort(&mut arr);
    let end = start.elapsed();
    println!("choice sort total cost time: {:?}", end);
    if !check_order(&arr) {
        println!("{:?}", "choice sort fail..");
    } else {
        println!("{:?}", "choice sort pass!!!");
    }
}

#[test]
pub fn test_insert_sort() {
    let mut arr = create_arr(SIZE);
    let start = time::Instant::now();
    sort::insert_sort(&mut arr);
    let end = start.elapsed();
    println!("insert sort total cost time: {:?}", end);
    let is_order = check_order(&arr);
    if !is_order {
        println!("{:?}", "insert sort fail..");
    } else {
        println!("{:?}", "insert sort pass!!!");
    }
    assert!(is_order);
}

#[test]
pub fn test_shell_sort() {
    let mut arr = create_arr(SIZE);
    println!("{:?}", arr);
    let start = time::Instant::now();
    sort::shell_sort2(&mut arr);
    let end = start.elapsed();
    println!("shell sort total cost time: {:?}", end);
    let is_order = check_order(&arr);
    if !is_order {
        println!("{:?}", "shell sort fail..");
    } else {
        println!("{:?}", "shell sort pass!!!");
    }
    println!("{:?}", arr);
    assert!(is_order);
}

#[test]
pub fn test_heap_sort() {
    let mut arr = create_arr(SIZE);
    // println!("{:?}", arr);
    let start = time::Instant::now();
    sort::heap_sort(&mut arr);
    let end = start.elapsed();
    println!("heap sort total cost time: {:?}", end);
    let is_order = check_order(&arr);
    if !is_order {
        println!("{:?}", "heap sort fail..");
    } else {
        println!("{:?}", "heap sort pass!!!");
    }
    assert!(is_order);
    // println!("{:?}", arr);
}

#[test]
pub fn test_merge_sort() {
    let mut arr = create_arr(SIZE);
    // println!("{:?}", arr);
    let start = time::Instant::now();
    sort::merge2_sort(&mut arr);
    // let arr = sort::merge_sort(&arr);
    let end = start.elapsed();
    println!("merge sort total cost time: {:?}", end);
    let is_order = check_order(&arr);
    if !is_order {
        println!("{:?}", "merge sort fail..");
    } else {
        println!("{:?}", "merge sort pass!!!");
    }
    assert!(is_order);
    // println!("{:?}", arr);
}

#[test]
pub fn test_quick_sort() {
    let mut arr = create_arr(SIZE);
    // println!("{:?}", arr);
    let start = time::Instant::now();
    sort::quick_sort(&mut arr);
    let end = start.elapsed();
    println!("quick sort total cost time: {:?}", end);
    let is_order = check_order(&arr);
    if !is_order {
        println!("{:?}", "quick sort fail..");
    } else {
        println!("{:?}", "quick sort pass!!!");
    }
    assert!(is_order);
    // println!("{:?}", arr);
}

#[test]
pub fn test_count_sort() {
    let mut arr = create_arr(SIZE);
    // println!("{:?}", arr);
    let start = time::Instant::now();
    sort::count_sort(&mut arr);
    let end = start.elapsed();
    println!("count sort total cost time: {:?}", end);
    let is_order = check_order(&arr);
    if !is_order {
        println!("{:?}", "count sort fail..");
    } else {
        println!("{:?}", "count sort pass!!!");
    }
    assert!(is_order);
    // println!("{:?}", arr);
}

#[test]
pub fn test_std_sort() {
    let mut arr = create_arr(SIZE);
    // println!("{:?}", arr);
    let start = time::Instant::now();
    arr.sort();
    let end = start.elapsed();
    println!("std sort total cost time: {:?}", end);
    if !check_order(&arr) {
        println!("{:?}", "std sort fail..");
    } else {
        println!("{:?}", "std sort pass!!!");
    }
    // println!("{:?}", arr);
}

#[test]
pub fn test_while() {
    let mut i = 0;
    let now = time::Instant::now();
    while i < 250000000 {
        i += 1;
    }
    now.elapsed();
    let end = now.elapsed();
    println!("{:?}", end);
}

#[test]
pub fn test_range() {
    let now = time::Instant::now();
    for _i in 0..250000000 {}
    let end = now.elapsed();
    println!("{:?}", end);
}

// 测试时间戳
#[test]
pub fn test_time_stamp() {
    let now = time::Instant::now();
    thread::sleep(time::Duration::from_secs(5));
    let end = time::Instant::now();
    let duration = end - now;
    println!("{:?}", duration);
    println!("{:?}", now.elapsed());
}

fn create_arr(n: i32) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut arr: Vec<i32> = (0..n).collect();
    arr.shuffle(&mut rng);
    arr
}

fn check_order(arr: &Vec<i32>) -> bool {
    if arr.len() <= 1 {
        return true;
    }
    for i in 0..arr.len() - 1 {
        if arr[i + 1] < arr[i] {
            println!("arr = {:?}", arr);
            println!("not in order, index: {}, {}", i, i + 1);
            return false;
        }
    }
    true
}
