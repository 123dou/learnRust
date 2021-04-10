//给定不同面额的硬币 coins 和一个总金额 amount。编写一个函数来计算可以凑成总金额所需的最少的硬币个数。如果没有任何一种硬币组合能组成总金额，返回
// -1。
//
// 你可以认为每种硬币的数量是无限的。
//
//
//
// 示例 1：
//
//
//输入：coins = [1, 2, 5], amount = 11
//输出：3
//解释：11 = 5 + 5 + 1
//
// 示例 2：
//
//
//输入：coins = [2], amount = 3
//输出：-1
//
// 示例 3：
//
//
//输入：coins = [1], amount = 0
//输出：0
//
//
// 示例 4：
//
//
//输入：coins = [1], amount = 1
//输出：1
//
//
// 示例 5：
//
//
//输入：coins = [1], amount = 2
//输出：2
//
//
//
//
// 提示：
//
//
// 1 <= coins.length <= 12
// 1 <= coins[i] <= 231 - 1
// 0 <= amount <= 104
//
// Related Topics 动态规划
// 👍 1041 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        let coins = vec![1];
        println!("solution = {:?}", Solution::coin_change(coins, 0));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    // 完全背包问题
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut dp = vec![];
        let len = amount as usize + 1;
        dp.resize(len, amount + 1);
        for coin in coins.iter() {
            if *coin < len as i32 {
                dp[*coin as usize] = 1;
                for i in *coin as usize..len {
                    dp[i] = dp[i].min(dp[i - *coin as usize] + 1);
                }
            }
        }
        return if *dp.last().unwrap() == amount + 1 {
            -1
        } else {
            *dp.last().unwrap()
        };
    }
}
//leetcode submit region end(Prohibit modification and deletion)
