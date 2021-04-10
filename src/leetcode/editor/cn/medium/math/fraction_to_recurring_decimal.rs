//给定两个整数，分别表示分数的分子 numerator 和分母 denominator，以 字符串形式返回小数 。
//
// 如果小数部分为循环小数，则将循环的部分括在括号内。
//
// 如果存在多个答案，只需返回 任意一个 。
//
// 对于所有给定的输入，保证 答案字符串的长度小于 104 。
//
//
//
// 示例 1：
//
//
//输入：numerator = 1, denominator = 2
//输出："0.5"
//
//
// 示例 2：
//
//
//输入：numerator = 2, denominator = 1
//输出："2"
//
//
// 示例 3：
//
//
//输入：numerator = 2, denominator = 3
//输出："0.(6)"
//
//
// 示例 4：
//
//
//输入：numerator = 4, denominator = 333
//输出："0.(012)"
//
//
// 示例 5：
//
//
//输入：numerator = 1, denominator = 5
//输出："0.2"
//
//
//
//
// 提示：
//
//
// -231 <= numerator, denominator <= 231 - 1
// denominator != 0
//
// Related Topics 哈希表 数学
// 👍 207 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_fraction_to_recurring_decimal() {
        println!(
            "solution = {:?}",
            Solution::fraction_to_decimal(i32::MAX, 37)
        );
        println!(
            "solution = {:?}",
            Solution::fraction_to_decimal(-1, i32::MIN)
        );

        println!("solution = {:?}", Solution::fraction_to_decimal(22, 7));
        println!("solution = {:?}", Solution::fraction_to_decimal(1, 6));
        println!("solution = {:?}", Solution::fraction_to_decimal(4, 333));
        println!("solution = {:?}", Solution::fraction_to_decimal(40, 333));
        println!("solution = {:?}", Solution::fraction_to_decimal(2, 3));
        println!("solution = {:?}", Solution::fraction_to_decimal(2, 30));
        println!("solution = {:?}", Solution::fraction_to_decimal(200, 30));
    }
}

struct Solution {}

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let (mut temp_numerator, mut temp_denominator) = (numerator as i64, denominator as i64);
        // 第一个边界条件
        if temp_numerator % temp_denominator == 0 {
            return (numerator as f64 / denominator as f64).to_string();
        }
        temp_numerator = temp_numerator.abs();
        temp_denominator = temp_denominator.abs();
        let mut res = "".to_string();
        // 第二个特殊条件
        if (numerator < 0 && denominator > 0) || (numerator > 0 && denominator < 0) {
            res.insert_str(0, "-");
        }
        let mut map = HashMap::new();
        res.push_str(&(temp_numerator / temp_denominator).to_string());
        let mut remainder = temp_numerator % temp_denominator;
        // 先处理整数的部分
        res.push_str(".");
        while !map.contains_key(&remainder) {
            map.insert(remainder, res.len());
            temp_numerator = remainder * 10;
            remainder = temp_numerator % temp_denominator;
            res.push_str(&(temp_numerator / temp_denominator).to_string());
            // 非循环小数
            if remainder == 0 {
                return res;
            }
        }
        let index = *map.get(&remainder).unwrap();
        res.insert(index, '(');
        res.push(')');
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
