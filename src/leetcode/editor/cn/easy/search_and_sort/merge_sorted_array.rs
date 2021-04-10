//给你两个有序整数数组 nums1 和 nums2，请你将 nums2 合并到 nums1 中，使 nums1 成为一个有序数组。
//
//
//
// 说明:
//
//
// 初始化 nums1 和 nums2 的元素数量分别为 m 和 n 。
// 你可以假设 nums1 有足够的空间（空间大小大于或等于 m + n）来保存 nums2 中的元素。
//
//
//
//
// 示例:
//
// 输入:
//nums1 = [1,2,3,0,0,0], m = 3
//nums2 = [2,5,6],       n = 3
//
//输出: [1,2,2,3,5,6]
// Related Topics 数组 双指针
// 👍 585 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_array() {
        let mut num1 = vec![2, 0];
        let mut num2 = vec![1];
        Solution::merge(&mut num1, 1, &mut num2, 1);
        eprintln!("num1 = {:?}", num1);
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut right = nums1.len();
        while n > 0 {
            right -= 1;
            if m == 0 || nums1[m - 1] < nums2[n - 1] {
                nums1[right] = nums2[n - 1];
                if n > 0 {
                    n -= 1
                }
            } else {
                nums1.swap(m - 1, right);
                if m > 0 {
                    m -= 1
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
