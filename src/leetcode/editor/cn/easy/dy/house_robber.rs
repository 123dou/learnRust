//你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上
//被小偷闯入，系统会自动报警。
//
// 给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。
//
//
//
// 示例 1：
//
// 输入：[1,2,3,1]
//输出：4
//解释：偷窃 1 号房屋 (金额 = 1) ，然后偷窃 3 号房屋 (金额 = 3)。
//     偷窃到的最高金额 = 1 + 3 = 4 。
//
// 示例 2：
//
// 输入：[2,7,9,3,1]
//输出：12
//解释：偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
//     偷窃到的最高金额 = 2 + 9 + 1 = 12 。
//
//
//
//
// 提示：
//
//
// 0 <= nums.length <= 100
// 0 <= nums[i] <= 400
//
// Related Topics 动态规划
// 👍 1157 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_house_robber() {
        let ve = vec![2, 1, 1, 2];
        let res = Solution::rob(ve);
        println!("{}", res);
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    // fn(n) = max{fn(n-1), fn(n - 2) + arr[n]}
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            2 => nums[1].max(nums[0]),
            _ => {
                let mut n2 = nums[0];
                let mut n1 = nums[1].max(nums[0]);
                for val in nums[2..].iter() {
                    let temp = n1;
                    n1 = n1.max(n2 + *val);
                    n2 = temp;
                }
                n1
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
