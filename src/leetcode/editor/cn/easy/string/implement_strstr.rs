//实现 strStr() 函数。
//
// 给定一个 haystack 字符串和一个 needle 字符串，在 haystack 字符串中找出 needle 字符串出现的第一个位置 (从0开始)。如
//果不存在，则返回 -1。
//
// 示例 1:
//
// 输入: haystack = "hello", needle = "ll"
//输出: 2
//
//
// 示例 2:
//
// 输入: haystack = "aaaaa", needle = "bba"
//输出: -1
//
//
// 说明:
//
// 当 needle 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。
//
// 对于本题而言，当 needle 是空字符串时我们应当返回 0 。这与C语言的 strstr() 以及 Java的 indexOf() 定义相符。
// Related Topics 双指针 字符串
// 👍 505 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_implement_strstr() {
        let a = "hello".to_string();
        let b = "ll".to_string();
        println!("{}", Solution::str_str(a, b));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let mut res = -1;
        for i in 0..=haystack.len() - needle.len() {
            for j in 1..=needle.len() {
                if needle[0..j] != haystack[i..j + i] {
                    break;
                }
                if j == needle.len() {
                    res = i as i32;
                }
            }
            if res != -1 {
                break;
            }
        }
        return res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
