//给定两个整数，被除数 dividend 和除数 divisor。将两数相除，要求不使用乘法、除法和 mod 运算符。
//
// 返回被除数 dividend 除以除数 divisor 得到的商。
//
// 整数除法的结果应当截去（truncate）其小数部分，例如：truncate(8.345) = 8 以及 truncate(-2.7335) = -2
//
//
//
// 示例 1:
//
// 输入: dividend = 10, divisor = 3
//输出: 3
//解释: 10/3 = truncate(3.33333..) = truncate(3) = 3
//
// 示例 2:
//
// 输入: dividend = 7, divisor = -3
//输出: -2
//解释: 7/-3 = truncate(-2.33333..) = -2
//
//
//
// 提示：
//
//
// 被除数和除数均为 32 位有符号整数。
// 除数不为 0。
// 假设我们的环境只能存储 32 位有符号整数，其数值范围是 [−231, 231 − 1]。本题中，如果除法结果溢出，则返回 231 − 1。
//
// Related Topics 数学 二分查找
// 👍 502 👎 0

#[cfg(test)]
pub mod tests {
    use std::ops::Div;

    use super::*;

    #[test]
    fn test_divide_two_integers() {
        let dividend = i32::MIN;
        let divisor = 3;
        println!("solution= {:?}", Solution::divide(dividend, divisor));
        println!("res = {:?}", dividend.div(divisor));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut dividend = dividend;
        let mut divisor = divisor;
        let sign = (dividend > 0) ^ (divisor > 0);
        let mut ans = 0;
        //都转换成负数
        //因为i32::min_value()的绝对值大于i32::max_value()，不能直接转换为正值
        if dividend > 0 {
            dividend = -dividend;
        }
        if divisor > 0 {
            divisor = -divisor;
        }
        while dividend <= divisor {
            //因为abs(dividend)>=abs(divisor), 所以结果至少为-1
            let mut tmp_ans = -1;
            let mut tmp_divisor = divisor;
            //下面要检查dividend/(2*tmp_divisor)
            while dividend <= (tmp_divisor << 1) {
                //在移位之前，检查是否移位后会溢出
                if tmp_divisor <= (i32::MIN >> 1) {
                    break;
                }
                //
                tmp_ans <<= 1;
                tmp_divisor <<= 1;
            }
            dividend = dividend - tmp_divisor;
            ans += tmp_ans; // temp_ans一定为2的n次方， 所以这里的+跟|是等价的
        }
        if !sign {
            //ans不会小于i32::min_value()
            if ans == i32::MIN {
                return i32::MAX;
            } else {
                ans = -ans;
            }
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
