//不使用运算符 + 和 - ，计算两整数 a 、b 之和。
//
// 示例 1:
//
// 输入: a = 1, b = 2
//输出: 3
//
//
// 示例 2:
//
// 输入: a = -2, b = 3
//输出: 1
// Related Topics 位运算
// 👍 379 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_sum_of_two_integers() {
        let a = -1;
        let b = 2;
        assert_eq!(Solution::get_sum(a, b), a + b);
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut sum = 0;
        let mut carry = false; // 考虑进位
        for i in 0..32 {
            let th_bit = 1 << i;
            let a_th_bit = a & th_bit;
            let b_th_bit = b & th_bit;
            if (a_th_bit & b_th_bit) == th_bit {
                if carry {
                    sum |= th_bit;
                } else {
                    carry = true;
                }
            } else if (a_th_bit | b_th_bit) != th_bit {
                if carry {
                    sum |= th_bit;
                    carry = false;
                }
            } else {
                if !carry {
                    sum |= th_bit;
                }
            }
        }
        sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)
