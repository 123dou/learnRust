//给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
//
// 示例 1:
//
// 输入: "abcabcbb"
//输出: 3
//解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
//
//
// 示例 2:
//
// 输入: "bbbbb"
//输出: 1
//解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
//
//
// 示例 3:
//
// 输入: "pwwkew"
//输出: 3
//解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
//     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
//
// Related Topics 哈希表 双指针 字符串 Sliding Window
// 👍 4581 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
use std::option::Option::Some;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_longest_substring_without_repeating_characters() {}
}

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut max_len_of_sub_str = 1;
        let mut start_idx = 0;
        let chars = s.as_bytes();
        let mut map: HashMap<&u8, usize> = HashMap::new();
        for (i, val) in chars.into_iter().enumerate() {
            if let Some(idx_new) = map.get(val) {
                let idx_new = *idx_new; // 没有这一条赋值语句, map.remove() 会编译不通过,报多次可变借用的错误
                max_len_of_sub_str = max_len_of_sub_str.max(i - start_idx);
                for j in start_idx..=idx_new {
                    map.remove(&chars[j]);
                }
                start_idx = idx_new + 1;
            }
            map.insert(val, i);
        }
        (map.len().max(max_len_of_sub_str)) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
