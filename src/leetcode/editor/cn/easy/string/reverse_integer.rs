//给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
//
// 示例 1:
//
// 输入: 123
//输出: 321
//
//
// 示例 2:
//
// 输入: -123
//输出: -321
//
//
// 示例 3:
//
// 输入: 120
//输出: 21
//
//
// 注意:
//
// 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−231, 231 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。
// Related Topics 数学
// 👍 2034 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_reverse_integer() {
        println!("{:?}", Solution::reverse(-1230));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // let mut flag = 1;
        // if x < 0 {
        //     flag = -1;
        // }
        let mut temp = x;
        let mut num: i64 = 0;
        while temp != 0 {
            num *= 10;
            let n = temp % 10;
            num += n as i64;
            temp /= 10;
        }
        // num *= flag;
        if num < i32::min_value() as i64 || num > i32::max_value() as i64 {
            return 0;
        }
        return num as i32;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
