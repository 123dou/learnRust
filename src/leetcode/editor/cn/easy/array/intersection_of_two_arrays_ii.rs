//给定两个数组，编写一个函数来计算它们的交集。
//
// 示例 1:
//
// 输入: nums1 = [1,2,2,1], nums2 = [2,2]
//输出: [2,2]
//
//
// 示例 2:
//
// 输入: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
//输出: [4,9]
//
// 说明：
//
//
// 输出结果中每个元素出现的次数，应与元素在两个数组中出现的次数一致。
// 我们可以不考虑输出结果的顺序。
//
//
// 进阶:
//
//
// 如果给定的数组已经排好序呢？你将如何优化你的算法？
// 如果 nums1 的大小比 nums2 小很多，哪种方法更优？
// 如果 nums2 的元素存储在磁盘上，磁盘内存是有限的，并且你不能一次加载所有的元素到内存中，你该怎么办？
//
// Related Topics 排序 哈希表 双指针 二分查找
// 👍 299 👎 0

use std::cmp::Ordering;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_intersection_of_two_arrays_ii() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        println!("{:?}", Solution::intersect(nums1, nums2));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.is_empty() || nums2.is_empty() {
            return vec![];
        }
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut res = vec![];
        let mut num2_idx = 0;
        for &num1 in nums1.iter() {
            if num1 > *nums2.last().unwrap() {
                break;
            }
            for (idx, &num2) in nums2[num2_idx..].iter().enumerate() {
                match num1.cmp(&num2) {
                    Ordering::Equal => {
                        res.push(num1);
                        num2_idx += idx + 1;
                        break;
                    }
                    Ordering::Less => {
                        num2_idx += idx;
                        break;
                    }
                    _ => {}
                }
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
