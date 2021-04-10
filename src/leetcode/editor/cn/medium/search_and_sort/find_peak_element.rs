//峰值元素是指其值大于左右相邻值的元素。
//
// 给你一个输入数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。
//
// 你可以假设 nums[-1] = nums[n] = -∞ 。
//
//
//
// 示例 1：
//
//
//输入：nums = [1,2,3,1]
//输出：2
//解释：3 是峰值元素，你的函数应该返回其索引 2。
//
// 示例 2：
//
//
//输入：nums = [1,2,1,3,5,6,4]
//输出：1 或 5
//解释：你的函数可以返回索引 1，其峰值元素为 2；
//     或者返回索引 5， 其峰值元素为 6。
//
//
//
//
// 提示：
//
//
// 1 <= nums.length <= 1000
// -231 <= nums[i] <= 231 - 1
// 对于所有有效的 i 都有 nums[i] != nums[i + 1]
//
//
//
//
// 进阶：你可以实现时间复杂度为 O(logN) 的解决方案吗？
// Related Topics 数组 二分查找
// 👍 356 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_find_peak_element() {
        let nums = vec![3, 5, 4, 3, 4, 5, 7];
        println!("solution = {:?}", Solution::find_peak_element(nums));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut res = 0;
        Self::find_peak(&nums, 0, nums.len() - 1, &mut res);
        return res as i32;
    }

    fn find_peak(ve: &Vec<i32>, lo: usize, hi: usize, res: &mut usize) {
        if lo >= hi {
            *res = lo;
            return;
        }
        let mid = (lo + hi) / 2;
        if (mid == 0 && ve[mid] > ve[mid + 1])
            || (mid == ve.len() - 1 && mid != 0 && ve[mid] > ve[mid - 1])
            || (mid != 0 && ve[mid] > ve[mid - 1] && ve[mid] > ve[mid + 1])
        {
            *res = mid;
            return;
        } else if (mid == 0 && ve[mid] < ve[mid + 1])
            || (mid != 0 && ve[mid - 1] < ve[mid] && ve[mid] < ve[mid + 1])
        {
            Self::find_peak(ve, mid + 1, hi, res);
        } else {
            Self::find_peak(ve, lo, mid, res);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
