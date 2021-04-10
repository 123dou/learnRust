//给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复
//的三元组。
//
// 注意：答案中不可以包含重复的三元组。
//
//
//
// 示例：
//
// 给定数组 nums = [-1, 0, 1, 2, -1, -4]，
//
//满足要求的三元组集合为：
//[
//  [-1, 0, 1],
//  [-1, -1, 2]
//]
//
// Related Topics 数组 双指针
// 👍 2748 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_3sum() {
        let ve = vec![-4, -2, -1];
        println!("{:?}", Solution::three_sum(ve));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        nums.sort();
        let mut idx = 0;
        let mut res = vec![];
        while idx < nums.len() && nums[idx] <= 0 {
            if idx != 0 && nums[idx] == nums[idx - 1] {
                idx += 1;
                continue;
            }
            Solution::two_sum(&nums.as_slice()[idx + 1..], -nums[idx], &mut res);
            idx += 1;
        }
        res
    }

    fn two_sum(nums: &[i32], target: i32, ve: &mut Vec<Vec<i32>>) {
        if nums.len() == 0 {
            return;
        }
        let mut lo = 0usize;
        let mut hi = nums.len() - 1;
        while lo < hi {
            let sum = nums[lo] + nums[hi];
            if lo != 0 && nums[lo - 1] == nums[lo] {
                lo += 1;
                continue;
            }
            if sum == target {
                ve.push(vec![nums[lo], nums[hi], -target]);
                lo += 1;
                hi -= 1;
            } else if sum < target {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
