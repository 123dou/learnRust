//实现 pow(x, n) ，即计算 x 的 n 次幂函数（即，xn）。
//
//
//
// 示例 1：
//
//
//输入：x = 2.00000, n = 10
//输出：1024.00000
//
//
// 示例 2：
//
//
//输入：x = 2.10000, n = 3
//输出：9.26100
//
//
// 示例 3：
//
//
//输入：x = 2.00000, n = -2
//输出：0.25000
//解释：2-2 = 1/22 = 1/4 = 0.25
//
//
//
//
// 提示：
//
//
// -100.0 < x < 100.0
// -231 <= n <= 231-1
// -104 <= xn <= 104
//
// Related Topics 数学 二分查找
// 👍 585 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_powx_n() {
        println!("solution = {:?}", Solution::my_pow(2f64, 11));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1f64;
        }
        let mut m: i64 = n as i64;
        if m < 0 {
            x = 1f64 / x;
            m = -m
        }
        let mut other = 1f64;
        while m > 1 {
            if m % 2 != 0 {
                other *= x;
            }
            x *= x;
            m /= 2;
        }
        x * other
    }
}
//leetcode submit region end(Prohibit modification and deletion)
