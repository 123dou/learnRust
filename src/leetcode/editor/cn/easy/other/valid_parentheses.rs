//给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。
//
// 有效字符串需满足：
//
//
// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
//
//
// 注意空字符串可被认为是有效字符串。
//
// 示例 1:
//
// 输入: "()"
//输出: true
//
//
// 示例 2:
//
// 输入: "()[]{}"
//输出: true
//
//
// 示例 3:
//
// 输入: "(]"
//输出: false
//
//
// 示例 4:
//
// 输入: "([)]"
//输出: false
//
//
// 示例 5:
//
// 输入: "{[]}"
//输出: true
// Related Topics 栈 字符串
// 👍 1978 👎 0

use std::option::Option::Some;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        println!("{}", Solution::is_valid("())".to_string()));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars = s.as_bytes();
        let mut stack = vec![];
        for val in chars {
            match val {
                b'(' | b'[' | b'{' => stack.push(val),
                b')' | b']' | b'}' => {
                    if stack.is_empty() {
                        return false;
                    }
                    if let Some(temp) = stack.pop() {
                        if *val == b')' && *temp != b'(' {
                            return false;
                        }
                        if *val == b']' && *temp != b'[' {
                            return false;
                        }
                        if *val == b'}' && *temp != b'{' {
                            return false;
                        }
                    }
                }
                _ => return true,
            }
        }
        stack.is_empty()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
