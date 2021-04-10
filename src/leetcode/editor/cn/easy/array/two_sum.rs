//给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
//
//
//
// 示例:
//
// 给定 nums = [2, 7, 11, 15], target = 9
//
//因为 nums[0] + nums[1] = 2 + 7 = 9
//所以返回 [0, 1]
//
// Related Topics 数组 哈希表
// 👍 8624 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let v = vec![3, 2, 4];
        eprintln!("v = {:?}", Solution::two_sum(v, 6));
    }
}

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut v = vec![];
        for (i, val) in nums.iter().enumerate() {
            map.insert(*val, i);
        }
        for (i, val) in nums.iter().enumerate() {
            if map.contains_key(&(target - *val)) {
                let j = *map.get(&(target - *val)).unwrap();
                if i == j {
                    continue;
                }
                v.push(i as i32);
                v.push(j as i32);
                break;
            }
        }
        return v;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
