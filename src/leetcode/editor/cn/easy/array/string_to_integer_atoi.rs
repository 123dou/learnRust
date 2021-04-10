//请你来实现一个 atoi 函数，使其能将字符串转换成整数。
//
// 首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。接下来的转化规则如下：
//
//
// 如果第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字字符组合起来，形成一个有符号整数。
// 假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成一个整数。
// 该字符串在有效的整数部分之后也可能会存在多余的字符，那么这些字符可以被忽略，它们对函数不应该造成影响。
//
//
// 注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，则你的函数不需要进行转换，即无法进行有效转换。
//
// 在任何情况下，若函数不能进行有效的转换时，请返回 0 。
//
// 提示：
//
//
// 本题中的空白字符只包括空格字符 ' ' 。
// 假设我们的环境只能存储 32 位大小的有符号整数，那么其数值范围为 [−231, 231 − 1]。如果数值超过这个范围，请返回 INT_MAX (231
// − 1) 或 INT_MIN (−231) 。
//
//
//
//
// 示例 1:
//
// 输入: "42"
//输出: 42
//
//
// 示例 2:
//
// 输入: "   -42"
//输出: -42
//解释: 第一个非空白字符为 '-', 它是一个负号。
//     我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。
//
//
// 示例 3:
//
// 输入: "4193 with words"
//输出: 4193
//解释: 转换截止于数字 '3' ，因为它的下一个字符不为数字。
//
//
// 示例 4:
//
// 输入: "words and 987"
//输出: 0
//解释: 第一个非空字符是 'w', 但它不是数字或正、负号。
//     因此无法执行有效的转换。
//
// 示例 5:
//
// 输入: "-91283472332"
//输出: -2147483648
//解释: 数字 "-91283472332" 超过 32 位有符号整数范围。
//     因此返回 INT_MIN (−231) 。
//
// Related Topics 数学 字符串

use std::*;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        // let strs = " ";
        // let output = Solution::my_atoi(strs.to_string());
        // println!("{}", output);
        println!("{}", "1332\n2");
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn my_atoi2(str: String) -> i32 {
        if str.is_empty() {
            return 0;
        }
        // 去掉空格
        let mut idx = 0;
        while idx < str.len() && (str.as_bytes()[idx] as char).eq(&' ') {
            idx += 1;
        }
        let mut pos = true;
        // 判断正负号
        if idx < str.len() && (str.as_bytes()[idx].eq(&b'-') || str.as_bytes()[idx].eq(&b'+')) {
            if str.as_bytes()[idx].eq(&b'-') {
                pos = false;
            }
            idx += 1;
        } else if idx < str.len() && !str.as_bytes()[idx].is_ascii_digit() {
            return 0;
        }
        // 开始转换
        let mut res: i64 = 0;
        while idx < str.len() && str.as_bytes()[idx].is_ascii_digit() {
            if res > i32::MAX as i64 {
                return if pos { i32::MAX } else { i32::MIN };
            }
            res += (str.as_bytes()[idx] - b'0') as i64;
            res *= 10;
            idx += 1;
        }
        if !pos {
            res = 0 - res;
        }
        res /= 10;
        return if res < i32::MIN as i64 {
            i32::MIN
        } else if res > i32::MAX as i64 {
            i32::MAX
        } else {
            res as i32
        };
    }

    pub fn my_atoi(str: String) -> i32 {
        if str.is_empty() {
            return 0;
        }
        let mut res: i64 = 0;
        let mut pos = true;
        for (i, c) in str.trim_start().chars().enumerate() {
            if i == 0 && c == '+' {
                continue;
            }
            if i == 0 && c == '-' {
                pos = false;
                continue;
            }
            if !c.is_ascii_digit() {
                break;
            }
            if res > i32::MAX as i64 {
                return if pos { i32::MAX } else { i32::MIN };
            }
            res += c.to_digit(10).unwrap() as i64;
            res *= 10;
        }
        if !pos {
            res = -res;
        }
        res /= 10;
        if res < i32::MIN as i64 {
            return i32::MIN;
        } else if res > i32::MAX as i64 {
            return i32::MAX;
        }
        return res as i32;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
