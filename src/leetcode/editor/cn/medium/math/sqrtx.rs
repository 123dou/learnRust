//实现 int sqrt(int x) 函数。
//
// 计算并返回 x 的平方根，其中 x 是非负整数。
//
// 由于返回类型是整数，结果只保留整数的部分，小数部分将被舍去。
//
// 示例 1:
//
// 输入: 4
//输出: 2
//
//
// 示例 2:
//
// 输入: 8
//输出: 2
//说明: 8 的平方根是 2.82842...,
//     由于返回类型是整数，小数部分将被舍去。
//
// Related Topics 数学 二分查找
// 👍 597 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_sqrtx() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut mid = 0;
        let mut lo = 0;
        let x = x as i64;
        let mut hi = x;
        while lo <= hi {
            mid = (lo + hi) / 2;
            let mid_pow2 = mid * mid;
            let next_mid_pow2 = (mid + 1) * (mid + 1);
            if mid_pow2 <= x && next_mid_pow2 > x {
                return mid as i32;
            }
            if mid_pow2 < x {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        return mid as i32;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
