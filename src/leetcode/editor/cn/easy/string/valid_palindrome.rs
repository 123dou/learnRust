//给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。
//
// 说明：本题中，我们将空字符串定义为有效的回文串。
//
// 示例 1:
//
// 输入: "A man, a plan, a canal: Panama"
//输出: true
//
//
// 示例 2:
//
// 输入: "race a car"
//输出: false
//
// Related Topics 双指针 字符串
// 👍 250 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome() {
        let s = "A man, a plan, a canal: Panama".to_string();
        println!("{}", Solution::is_palindrome(s));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut txt = vec![];
        for x in s.chars() {
            if x.is_ascii_alphanumeric() {
                txt.push(x.to_ascii_lowercase());
            }
        }
        if txt.len() == 0 {
            return true;
        }
        let mut low = 0;
        let mut hi = txt.len() - 1;
        while low < hi {
            if txt[low] != txt[hi] {
                return false;
            }
            low += 1;
            hi -= 1;
        }
        return true;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
