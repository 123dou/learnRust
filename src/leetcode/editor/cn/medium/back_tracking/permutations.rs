//给定一个 没有重复 数字的序列，返回其所有可能的全排列。
//
// 示例:
//
// 输入: [1,2,3]
//输出:
//[
//  [1,2,3],
//  [1,3,2],
//  [2,1,3],
//  [2,3,1],
//  [3,1,2],
//  [3,2,1]
//]
// Related Topics 回溯算法
// 👍 1090 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        println!("solution = {:?}", Solution::permute(vec![1, 2, 3]));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::_permute(&mut nums, 0, &mut res);
        res
    }

    fn _permute(nums: &mut Vec<i32>, index: usize, res: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            res.push(nums.clone());
            return;
        }
        for i in index..nums.len() {
            nums.swap(i, index);
            Self::_permute(nums, index + 1, res);
            nums.swap(i, index);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
