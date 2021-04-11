//给定一个整数数组，判断是否存在重复元素。
//
// 如果任意一值在数组中出现至少两次，函数返回 true 。如果数组中每个元素都不相同，则返回 false 。
//
//
//
// 示例 1:
//
// 输入: [1,2,3,1]
//输出: true
//
// 示例 2:
//
// 输入: [1,2,3,4]
//输出: false
//
// 示例 3:
//
// 输入: [1,1,1,3,3,4,3,2,4,2]
//输出: true
// Related Topics 数组 哈希表

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let v = vec![1, 2, 3, 4, 54, 5];
        println!("{}", Solution::contains_duplicate(v));
    }
}

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for x in nums {
            if set.contains(&x) {
                return true;
            } else {
                set.insert(x);
            }
        }
        false
    }
}
//leetcode submit region end(Prohibit modification and deletion)
