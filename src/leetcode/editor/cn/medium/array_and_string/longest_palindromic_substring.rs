//给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
//
// 示例 1：
//
// 输入: "babad"
//输出: "bab"
//注意: "aba" 也是一个有效答案。
//
//
// 示例 2：
//
// 输入: "cbbd"
//输出: "bb"
//
// Related Topics 字符串 动态规划
// 👍 2902 👎 0



#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_longest_palindromic_substring() {
        println!("{}", Solution::longest_palindrome("ab".to_string()));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let mut res: &[u8] = (&s[0..1]).as_ref();
        for i in 0..s.len() {
            let temp = Solution::get_longest_palindrome(s.as_bytes(), i);
            if temp.len() > res.len() {
                res = temp;
            }
        }
        unsafe { String::from_utf8_unchecked(res.to_vec()) }
    }

    fn get_longest_palindrome(str: &[u8], start: usize) -> &[u8] {
        let mut lo = start;
        let mut hi = start;
        let mut need_lo = false;
        let mut need_hi = false;
        while str[lo] == str[start] {
            if lo == 0 {
                need_lo = true;
                break;
            }
            lo -= 1;
        }
        while str[hi] == str[start] {
            if hi == str.len() - 1 {
                need_hi = true;
                break;
            }
            hi += 1;
        }
        while str[lo] == str[hi] {
            if lo == 0 || hi == str.len() - 1 {
                return &str[lo..=hi];
            } else {
                lo -= 1;
                hi += 1;
            }
        }
        if need_hi && need_lo {
            return &str[lo..=hi];
        }
        if need_lo {
            return &str[lo..hi];
        }
        if need_hi {
            return &str[lo + 1..=hi];
        }
        return &str[lo + 1..hi];
    }
}
//leetcode submit region end(Prohibit modification and deletion)
