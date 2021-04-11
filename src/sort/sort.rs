use rand::prelude::*;

/// 冒泡排序
pub fn bubble_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

//noinspection ALL
// 选择排序
pub fn choice_sort(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }
    let mut min = i32::min_value();
    for i in 0..arr.len() {
        min = arr[i];
        for j in i..arr.len() {
            if arr[j] < min {
                min = arr[j];
            }
        }
        arr[i] = min;
    }
}

//noinspection ALL
// 插入排序
pub fn insert_sort(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }
    for i in 1..arr.len() {
        let curr = arr[i];
        let mut pre = i - 1;
        while arr[pre] > curr {
            arr[pre + 1] = arr[pre];
            if pre == 0 {
                arr[0] = curr;
                break;
            }
            pre -= 1;
        }
        if arr[pre] < curr {
            arr[pre + 1] = curr;
        }
    }
}

// shell排序
pub fn shell_sort(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }
    let mut inc = arr.len() / 2;
    // println!("{:?}", arr);
    while inc > 0 {
        for i in inc..arr.len() {
            let mut pre = (i - inc) as i32;
            let curr = arr[i];
            while pre >= 0 && arr[pre as usize] > curr {
                arr[pre as usize + inc] = arr[pre as usize];
                pre -= inc as i32;
            }
            arr[(pre + inc as i32) as usize] = curr;
        }
        inc /= 2;
    }
}

//noinspection ALL
// shell排序
pub fn shell_sort2(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }
    let mut inc = arr.len() / 2;
    while inc > 0 {
        for i in inc..arr.len() {
            let mut pre = i - inc;
            let curr = arr[i];
            while arr[pre] > curr {
                arr[pre + inc] = arr[pre];
                if pre <= inc {
                    arr[pre] = curr;
                    break;
                }
                pre -= inc;
            }
            if pre > inc {
                arr[pre + inc] = curr;
            }
        }
        inc /= 2;
    }
}

// heap 排序, 建造最大堆
pub fn heap_sort(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }
    // 堆化数组
    for i in 1..arr.len() {
        let mut curr = i;
        let mut parent = (curr - 1) / 2;
        while curr > 0 && arr[curr] > arr[parent] {
            arr.swap(curr, parent);
            curr = parent;
            if curr != 0 {
                parent = (curr - 1) / 2;
            }
        }
    }
    // 删除头元素
    let mut len = arr.len() - 1;
    while len > 0 {
        arr.swap(0, len);
        let mut curr = 0;
        let mut child = 1;
        while child < len {
            if child + 1 < len && arr[child + 1] > arr[child] {
                child += 1;
            }
            if arr[child] < arr[curr] {
                break;
            }
            arr.swap(child, curr);
            curr = child;
            child = curr * 2 + 1;
        }
        len -= 1;
    }
}

// 归并排序
pub fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mid = (arr.len()) / 2;
    let left = &arr[0..mid];
    let right = &arr[mid..];
    return merge(merge_sort(left).as_slice(), merge_sort(right).as_slice());
}

// 将两个有序的数组合并成一个
fn merge(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut arr3 = Vec::with_capacity(arr1.len() + arr2.len());
    let mut len1 = 0;
    let mut len2 = 0;
    while len1 < arr1.len() && len2 < arr2.len() {
        if arr1[len1] < arr2[len2] {
            arr3.push(arr1[len1]);
            len1 += 1;
        } else {
            arr3.push(arr2[len2]);
            len2 += 1;
        }
    }
    if len1 < arr1.len() {
        arr3.extend_from_slice(&arr1[len1..]);
    }
    if len2 < arr2.len() {
        arr3.extend_from_slice(&arr2[len2..]);
    }
    arr3
}

// 归并排序
pub fn merge2_sort(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }
    let mut aux = arr.clone();
    let mut sz = 1;
    while sz < arr.len() {
        let mut i = 0;
        while i < arr.len() - sz {
            merge2(arr, i, sz, (i + sz + sz).min(arr.len()), &mut aux);
            i = i + sz + sz;
        }
        sz += sz;
    }
}

fn merge2(arr: &mut Vec<i32>, left: usize, sz: usize, hi: usize, aux: &mut Vec<i32>) {
    // for i in left..hi {
    //     aux[i] = arr[i];
    // }
    aux[left..hi].clone_from_slice(&arr[left..hi]);
    let mut l_arr_idx = left;
    let mut r_arr_idx = left + sz;
    for i in left..hi {
        if l_arr_idx >= left + sz {
            arr[i] = aux[r_arr_idx];
            r_arr_idx += 1;
        } else if r_arr_idx >= hi {
            arr[i] = aux[l_arr_idx];
            l_arr_idx += 1;
        } else if aux[l_arr_idx] < aux[r_arr_idx] {
            // 用辅助数组来比较
            arr[i] = aux[l_arr_idx];
            l_arr_idx += 1;
        } else {
            arr[i] = aux[r_arr_idx];
            r_arr_idx += 1;
        }
    }
}

// 快速排序

pub fn quick_sort(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }
    let mut rng = thread_rng();
    quick_sort_aux(arr, 0, arr.len() - 1, &mut rng);
}

fn quick_sort_aux(arr: &mut Vec<i32>, lo: usize, hi: usize, rng: &mut ThreadRng) {
    if lo >= hi {
        return;
    }
    let pivot = partition(arr, lo, hi, rng);
    if pivot > lo {
        quick_sort_aux(arr, lo, pivot - 1, rng);
    }
    if pivot < hi {
        quick_sort_aux(arr, pivot + 1, hi, rng);
    }
}

fn partition(arr: &mut Vec<i32>, lo: usize, hi: usize, rng: &mut ThreadRng) -> usize {
    let pivot = rng.gen_range(lo, hi + 1);
    arr.swap(pivot, hi);
    let mut pre = (true, lo);
    for i in lo..hi + 1 {
        if arr[i] <= arr[hi] {
            if !pre.0 {
                pre.1 += 1;
            } else {
                pre.0 = false;
            }
            if i > pre.1 {
                arr.swap(i, pre.1);
            }
        }
    }
    pre.1
}

pub fn count_sort(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }
    let min = *arr.iter().min().unwrap_or(&0);
    let max = *arr.iter().max().unwrap_or(&0);
    let len = (max - min + 1) as usize;
    let mut aux = vec![0; len];
    arr.iter().for_each(|&i| aux[i as usize] += 1);
    let mut idx = 0;
    for i in 0..aux.len() {
        while aux[i] > 0 {
            arr[idx] = i as i32 + min;
            aux[i] -= 1;
            idx += 1;
        }
    }
}
