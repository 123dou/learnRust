//给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
//
// 示例:
//
// 输入: [-2,1,-3,4,-1,2,1,-5,4]
//输出: 6
//解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。
//
//
// 进阶:
//
// 如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的分治法求解。
// Related Topics 数组 分治算法 动态规划
// 👍 2615 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_maximum_subarray() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            _ => {
                let mut res = i32::MIN;
                let mut sum = 0;
                for val in nums.iter() {
                    sum += *val;
                    res = res.max(sum.max(*val));
                    if sum < *val {
                        sum = *val;
                    }
                }
                res
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
