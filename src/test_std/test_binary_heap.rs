use std::cmp::Ordering;
use std::collections::BinaryHeap;

use itertools::Itertools;

#[derive(PartialEq, Eq)] // 自动实现了PartialEq和Eq
struct Rev<T>(T);
impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
} // 实现了partial_cmp,lt,le,gt,ge方法
impl<T: Ord> Ord for Rev<T> {
    //     实现了Ord，才能排序
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        // enum Ordering 包含 Less = -1, Equal = 0, Greater = 1,
        other.0.cmp(&self.0)
    }
}

#[test]
pub fn test_binary_heap() {
    let mut que = BinaryHeap::new();
    que.push("abcd");
    que.push("adbc");
    que.push("acbd");
    let str2 = que.into_iter_sorted().join("-");
    println!("str = {:?}", str2);
    let mut que = BinaryHeap::new();
    que.push(Rev("abcd"));
    que.push(Rev("adbc"));
    que.push(Rev("acbd"));
    let str2 = que.into_iter_sorted().map(|val| val.0).join("-");
    println!("str = {:?}", str2);
}
