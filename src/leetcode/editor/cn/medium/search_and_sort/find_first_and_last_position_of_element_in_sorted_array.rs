//给定一个按照升序排列的整数数组 nums，和一个目标值 target。找出给定目标值在数组中的开始位置和结束位置。
//
// 如果数组中不存在目标值 target，返回 [-1, -1]。
//
// 进阶：
//
//
// 你可以设计并实现时间复杂度为 O(log n) 的算法解决此问题吗？
//
//
//
//
// 示例 1：
//
//
//输入：nums = [5,7,7,8,8,10], target = 8
//输出：[3,4]
//
// 示例 2：
//
//
//输入：nums = [5,7,7,8,8,10], target = 6
//输出：[-1,-1]
//
// 示例 3：
//
//
//输入：nums = [], target = 0
//输出：[-1,-1]
//
//
//
// 提示：
//
//
// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109
// nums 是一个非递减数组
// -109 <= target <= 109
//
// Related Topics 数组 二分查找
// 👍 825 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_find_first_and_last_position_of_element_in_sorted_array() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        println!("solution = {:?}", Solution::search_range(nums, 8));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        let left = Self::search_left(&nums, 0, nums.len() - 1, target);
        if nums[left] != target {
            return vec![-1, -1];
        }
        let right = Self::search_right(&nums, 0, nums.len() - 1, target);
        return vec![left as i32, right as i32];
    }

    fn search_left(nums: &Vec<i32>, lo: usize, hi: usize, target: i32) -> usize {
        if lo >= hi {
            return hi;
        }
        let mid = (lo + hi) / 2;
        return if target <= nums[mid] {
            Self::search_left(nums, lo, mid, target)
        } else {
            Self::search_left(nums, mid + 1, hi, target)
        };
    }

    fn search_right(nums: &Vec<i32>, lo: usize, hi: usize, target: i32) -> usize {
        if lo >= hi {
            return lo;
        }
        let mid = (lo + hi) / 2 + 1;
        return if target < nums[mid] {
            Self::search_right(nums, lo, mid - 1, target)
        } else {
            Self::search_right(nums, mid, hi, target)
        };
    }
}
//leetcode submit region end(Prohibit modification and deletion)
