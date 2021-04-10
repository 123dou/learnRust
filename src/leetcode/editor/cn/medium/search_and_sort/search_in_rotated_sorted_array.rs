//升序排列的整数数组 nums 在预先未知的某个点上进行了旋转（例如， [0,1,2,4,5,6,7] 经旋转后可能变为 [4,5,6,7,0,1,2] ）。
//
//
// 请你在数组中搜索 target ，如果数组中存在这个目标值，则返回它的索引，否则返回 -1 。
//
//
//
// 示例 1：
//
//
//输入：nums = [4,5,6,7,0,1,2], target = 0
//输出：4
//
//
// 示例 2：
//
//
//输入：nums = [4,5,6,7,0,1,2], target = 3
//输出：-1
//
// 示例 3：
//
//
//输入：nums = [1], target = 0
//输出：-1
//
//
//
//
// 提示：
//
//
// 1 <= nums.length <= 5000
// -10^4 <= nums[i] <= 10^4
// nums 中的每个值都 独一无二
// nums 肯定会在某个点上旋转
// -10^4 <= target <= 10^4
//
// Related Topics 数组 二分查找
// 👍 1154 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_search_in_rotated_sorted_array() {
        let nums = vec![3, 4, 5, 6, 1, 2];
        println!("search = {:?}", Solution::search(nums, 2));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut boarder = 0;
        Self::find_boarder(&nums, 0, nums.len() - 1, &mut boarder);
        let res = Self::find_target(&nums, 0, boarder, target);
        return if res != -1 {
            res
        } else {
            Self::find_target(&nums, boarder + 1, nums.len() - 1, target)
        };
    }

    fn find_target(arr: &Vec<i32>, mut lo: usize, mut hi: usize, target: i32) -> i32 {
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if arr[mid] == target {
                return mid as i32;
            } else if target < arr[mid] {
                if mid == 0 {
                    return -1;
                }
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        -1
    }

    fn find_boarder(arr: &Vec<i32>, lo: usize, hi: usize, res: &mut usize) {
        if lo > hi {
            return;
        }
        let mid = (lo + hi) / 2;
        // 边界考虑
        if mid != arr.len() - 1 && arr[mid] > arr[mid + 1] {
            *res = mid;
            return;
        }
        if mid != 0 {
            Self::find_boarder(arr, lo, mid - 1, res);
        }
        Self::find_boarder(arr, mid + 1, hi, res);
    }
}
//leetcode submit region end(Prohibit modification and deletion)
