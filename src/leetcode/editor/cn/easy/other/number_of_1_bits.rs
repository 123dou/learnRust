//编写一个函数，输入是一个无符号整数，返回其二进制表达式中数字位数为 ‘1’ 的个数（也被称为汉明重量）。
//
//
//
// 示例 1：
//
// 输入：00000000000000000000000000001011
//输出：3
//解释：输入的二进制串 00000000000000000000000000001011 中，共有三位为 '1'。
//
//
// 示例 2：
//
// 输入：00000000000000000000000010000000
//输出：1
//解释：输入的二进制串 00000000000000000000000010000000 中，共有一位为 '1'。
//
//
// 示例 3：
//
// 输入：11111111111111111111111111111101
//输出：31
//解释：输入的二进制串 11111111111111111111111111111101 中，共有 31 位为 '1'。
//
//
//
// 提示：
//
//
// 请注意，在某些语言（如 Java）中，没有无符号整数类型。在这种情况下，输入和输出都将被指定为有符号整数类型，并且不应影响您的实现，因为无论整数是有符号的
//还是无符号的，其内部的二进制表示形式都是相同的。
// 在 Java 中，编译器使用二进制补码记法来表示有符号整数。因此，在上面的 示例 3 中，输入表示有符号整数 -3。
//
//
//
//
// 进阶:
//如果多次调用这个函数，你将如何优化你的算法？
// Related Topics 位运算
// 👍 233 👎 0

#[cfg(test)]
pub mod tests {
    use crate::leetcode::editor::cn::easy::other::number_of_1_bits::Solution;

    #[test]
    fn test_number_of_1_bits() {
        println!("{}", Solution::hammingWeight(55));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    //noinspection ALL
    pub fn hammingWeight(n: u32) -> i32 {
        // 考虑2进制只有两位的情况, num_of_1 = x - (x >> 1)
        let mut res = n;
        res = res - ((res >> 1) & 0x55555555);
        res = (res & 0x33333333) + ((res >> 2) & 0x33333333);
        res = (res + (res >> 4)) & 0x0f0f0f0f;
        res = res + (res >> 8);
        res = res + (res >> 16);
        (res & 0x3f) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
