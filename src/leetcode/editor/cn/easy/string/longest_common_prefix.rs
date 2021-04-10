//编写一个函数来查找字符串数组中的最长公共前缀。
//
// 如果不存在公共前缀，返回空字符串 ""。
//
// 示例 1:
//
// 输入: ["flower","flow","flight"]
//输出: "fl"
//
//
// 示例 2:
//
// 输入: ["dog","racecar","car"]
//输出: ""
//解释: 输入不存在公共前缀。
//
//
// 说明:
//
// 所有输入只包含小写字母 a-z 。
// Related Topics 字符串
// 👍 1181 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec!["flower".to_string(), "a".to_string(), "b".to_string()];
        println!("{}", Solution::longest_common_prefix(strs).len());
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let mut min_str = strs[0].as_str();
        strs.iter().for_each(|x| {
            if x.len() < min_str.len() {
                min_str = x.as_str();
            }
        });
        for i in 1..=min_str.len() {
            if !Solution::has_common_prefix(&strs, &min_str[0..i]) {
                return if i == 0 {
                    "".to_string()
                } else {
                    String::from(&min_str[0..i - 1])
                };
            }
        }
        return String::from(min_str);
    }

    pub fn has_common_prefix(strs: &Vec<String>, str: &str) -> bool {
        for x in strs.iter() {
            if !x.starts_with(str) {
                return false;
            }
        }
        return true;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
