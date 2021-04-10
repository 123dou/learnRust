//编写一个算法来判断一个数 n 是不是快乐数。
//
// 「快乐数」定义为：
//
//
// 对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。
// 然后重复这个过程直到这个数变为 1，也可能是 无限循环 但始终变不到 1。
// 如果 可以变为 1，那么这个数就是快乐数。
//
//
// 如果 n 是快乐数就返回 true ；不是，则返回 false 。
//
//
//
// 示例 1：
//
//
//输入：19
//输出：true
//解释：
//12 + 92 = 82
//82 + 22 = 68
//62 + 82 = 100
//12 + 02 + 02 = 1
//
//
// 示例 2：
//
//
//输入：n = 2
//输出：false
//
//
//
//
// 提示：
//
//
// 1 <= n <= 231 - 1
//
// Related Topics 哈希表 数学
// 👍 536 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_happy_number() {}
}

struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        set.insert(n);
        return Self::is_happy_2(n, &mut set);
    }

    fn is_happy_2(mut n: i32, set: &mut HashSet<i32>) -> bool {
        if n == 1 {
            return true;
        }
        let mut res = 0;
        while n != 0 {
            let temp = n % 10;
            res += temp * temp;
            n /= 10;
        }
        if res == 1 {
            return true;
        }
        if set.contains(&res) {
            return false;
        }
        set.insert(res);
        return Self::is_happy_2(res, set);
    }
}
//leetcode submit region end(Prohibit modification and deletion)
