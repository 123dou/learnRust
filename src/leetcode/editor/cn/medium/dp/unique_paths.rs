//一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。
//
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。
//
// 问总共有多少条不同的路径？
//
//
//
// 示例 1：
//
//
//输入：m = 3, n = 7
//输出：28
//
// 示例 2：
//
//
//输入：m = 3, n = 2
//输出：3
//解释：
//从左上角开始，总共有 3 条路径可以到达右下角。
//1. 向右 -> 向下 -> 向下
//2. 向下 -> 向下 -> 向右
//3. 向下 -> 向右 -> 向下
//
//
// 示例 3：
//
//
//输入：m = 7, n = 3
//输出：28
//
//
// 示例 4：
//
//
//输入：m = 3, n = 3
//输出：6
//
//
//
// 提示：
//
//
// 1 <= m, n <= 100
// 题目数据保证答案小于等于 2 * 109
//
// Related Topics 数组 动态规划
// 👍 870 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_unique_paths() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 1 || n <= 1 {
            return 1;
        }
        let mut dp = vec![];
        let mut ve = vec![];
        Vec::resize(&mut ve, n as usize, 1);
        Vec::resize(&mut dp, m as usize, ve);
        for i in 1..dp.len() {
            for j in 1..dp[i].len() {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m as usize - 1][n as usize - 1]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
