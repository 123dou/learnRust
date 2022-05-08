use std::collections::HashMap;

#[test]
fn test_drain() {
    // 获取vec中的某一个范围的值
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let u: Vec<i32> = v.drain(1..3).collect();
    assert_eq!(u, vec![2, 3]);
    println!("u = {:?}", u);
}

#[test]
fn test_drain_filter() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let evens = v.drain_filter(|x| (*x & 1) == 0).collect::<Vec<i32>>();
    println!("evens = {:?}", evens);
    println!("v = {:?}", v);
}

#[test]
fn test_dedup() {
    // 移除连续重复的元素，如果vec是有序的，则会移除所有的重复元素
    let mut vec = vec![1, 2, 2, 2, 3, 3, 2];
    vec.dedup();
    assert_eq!(vec, vec![1, 2, 3, 2]);
    println!("vec = {:?}", vec);
}

#[test]
fn test_extend_from_slice() {
    let mut ve = vec![1, 2];
    let x_slice = &[4, 5, 6];
    ve.extend_from_slice(x_slice);
    println!("ve = {:?}", ve);
    println!("x_slice= {:?}", x_slice);
}

#[test]
fn test_append() {
    let mut ve = vec![1, 2, 3];
    let mut ve2 = vec![4, 5, 6];
    ve.append(&mut ve2);
    println!("ve = {:?}", ve);
    assert_eq!(ve, [1, 2, 3, 4, 5, 6]);
    println!("ve2 = {:?}", ve2);
    assert!(ve2.is_empty());
}

#[test]
fn test_retain() {
    // 接收一个断言接口
    let mut ve = vec![1, 2, 3, 4, 5, 6];
    ve.retain(|item| item % 2 == 0);
    println!("ve = {:?}", ve);

    let mut ve = vec![1, 2, 3, 4, 5, 6];
    let keep = vec![true, false, true, false, true, false, true, false];
    let mut i = 0;
    ve.retain(|_| (keep[i], i += 1).0);
    println!("ve = {:?}", ve);
}

#[test]
fn test_resize() {
    let mut ve = vec![];
    ve.resize(5, 1);
    println!("ve = {:?}", ve);
    assert_eq!(ve, vec![1, 1, 1, 1, 1])
}

#[test]
fn test_resize_with() {
    let mut ve = vec![];
    let mut var = 1;
    ve.resize_with(5, || {
        var *= 2;
        var += 1;
        var
    });
    println!("ve = {:?}", ve);
}

#[test]
fn test_binary_search() {
    let mut ve = vec![1, 2, 3, 4, 5, 5, 6, 6, 7, 8, 10, 23];
    let result = ve.binary_search(&6);
    // 这个的返回值索引可能是６或者７
    assert_eq!(result, Ok(6));
    println!("result = {:?}", result);
    // 如果没有找到则会返回，保证插入的位置而不会让破坏vec的顺序
    let idx = ve.binary_search(&9).unwrap_or_else(|xx| xx);
    ve.insert(idx, 9);
    println!("ve = {:?}", ve);
}

#[test]
fn test_map() {
    let mut nums = vec![3, 2, 1];
    let map = nums
        .iter()
        .enumerate()
        .map(|(idx, &val)| (val, idx))
        .collect::<HashMap<i32, usize>>();
    println!("map = {:?}", map);
}
