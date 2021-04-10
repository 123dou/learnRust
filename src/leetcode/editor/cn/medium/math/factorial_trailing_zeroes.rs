//给定一个整数 n，返回 n! 结果尾数中零的数量。
//
// 示例 1:
//
// 输入: 3
//输出: 0
//解释: 3! = 6, 尾数中没有零。
//
// 示例 2:
//
// 输入: 5
//输出: 1
//解释: 5! = 120, 尾数中有 1 个零.
//
// 说明: 你算法的时间复杂度应为 O(log n) 。
// Related Topics 数学
// 👍 420 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_factorial_trailing_zeroes() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    // 2*5 会产生一个0, 又因为2的数据必然大于5的数量，所以主要看1～n里面有多少个5相乘就行
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut res = 0;
        let mut factors_5 = 5;
        while factors_5 <= n {
            res += n / factors_5;
            factors_5 = factors_5 * 5;
        }
        return res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
