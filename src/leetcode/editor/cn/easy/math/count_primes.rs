//统计所有小于非负整数 n 的质数的数量。
//
//
//
// 示例 1：
//
// 输入：n = 10
//输出：4
//解释：小于 10 的质数一共有 4 个, 它们是 2, 3, 5, 7 。
//
//
// 示例 2：
//
// 输入：n = 0
//输出：0
//
//
// 示例 3：
//
// 输入：n = 1
//输出：0
//
//
//
//
// 提示：
//
//
// 0 <= n <= 5 * 106
//
// Related Topics 哈希表 数学
// 👍 472 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_count_primes() {
        println!("{}", Solution::count_primes(2));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut ve = vec![true; n as usize];
        let mut res = 0;
        for i in 2..ve.len() {
            match ve[i] {
                true => {
                    res += 1;
                    let mut j = i;
                    while j < n as usize {
                        ve[j] = false;
                        j += i;
                    }
                }
                _ => {}
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
