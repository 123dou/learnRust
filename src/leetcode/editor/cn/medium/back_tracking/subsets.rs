//给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。
//
// 解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。
//
//
//
// 示例 1：
//
//
//输入：nums = [1,2,3]
//输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//
//
// 示例 2：
//
//
//输入：nums = [0]
//输出：[[],[0]]
//
//
//
//
// 提示：
//
//
// 1 <= nums.length <= 10
// -10 <= nums[i] <= 10
// nums 中的所有元素 互不相同
//
// Related Topics 位运算 数组 回溯算法
// 👍 963 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        let nums = vec![1, 2, 3, 4];
        println!("solution = {:?}", Solution::subsets(nums));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for i in 0..=nums.len() {
            Self::_subsets(&nums, i, 0, &mut vec![], &mut res);
        }
        res
    }

    fn _subsets(
        nums: &Vec<i32>,
        size: usize,
        index: usize,
        item: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if item.len() == size {
            res.push(item.clone());
            return;
        }

        for i in index..nums.len() {
            item.push(nums[i]);
            // 这里递归的时候，要用i,不能用index，这样才能保证不会重复
            Self::_subsets(nums, size, i + 1, item, res);
            item.pop();
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
