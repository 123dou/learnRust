//两个整数之间的汉明距离指的是这两个数字对应二进制位不同的位置的数目。
//
// 给出两个整数 x 和 y，计算它们之间的汉明距离。
//
// 注意：
//0 ≤ x, y < 231.
//
// 示例:
//
//
//输入: x = 1, y = 4
//
//输出: 2
//
//解释:
//1   (0 0 0 1)
//4   (0 1 0 0)
//       ↑   ↑
//
//上面的箭头指出了对应二进制位不同的位置。
//
// Related Topics 位运算
// 👍 350 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_hamming_distance() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        Solution::count_one_of_num((x ^ y) as u32)
    }

    pub fn count_one_of_num(num: u32) -> i32 {
        let mut res = num;
        res = res - ((res >> 1) & 0x55555555);
        res = (res & 0x33333333) + ((res >> 2) & 0x33333333);
        res = (res + (res >> 4)) & 0x0f0f0f0f;
        res += res >> 8;
        res += res >> 16;
        (res & 0x3f) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
