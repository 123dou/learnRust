//给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。
//
// 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
//
//
//
// 示例:
//
// 输入："23"
//输出：["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
//
//
// 说明:
//尽管上面的答案是按字典序排列的，但是你可以任意选择答案输出的顺序。
// Related Topics 深度优先搜索 递归 字符串 回溯算法
// 👍 1083 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations_of_a_phone_number() {
        let s = "23".to_string();
        println!("solution = {:?}", Solution::letter_combinations(s));
    }
}

struct Solution {}

impl Solution {
    pub fn letter_combinations1(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let mut map = HashMap::new();
        map.insert(b'2', vec![b'a', b'b', b'c']);
        map.insert(b'3', vec![b'd', b'e', b'f']);
        map.insert(b'4', vec![b'g', b'h', b'i']);
        map.insert(b'5', vec![b'j', b'k', b'l']);
        map.insert(b'6', vec![b'm', b'n', b'o']);
        map.insert(b'7', vec![b'p', b'q', b'r', b's']);
        map.insert(b'8', vec![b't', b'u', b'v']);
        map.insert(b'9', vec![b'w', b'x', b'y', b'z']);
        let mut res = vec![];
        let digits = digits.as_bytes();
        Solution::back_tracing(digits, &map, "", &mut res);
        res
    }
    fn back_tracing(digits: &[u8], map: &HashMap<u8, Vec<u8>>, s: &str, res: &mut Vec<String>) {
        if s.len() == digits.len() {
            res.push(s.to_string());
            return;
        }
        let key = digits.get(s.len()).unwrap();
        for x in map.get(key).unwrap().iter() {
            let mut temp = s.to_string();
            temp.push(*x as char);
            Solution::back_tracing(digits, map, temp.as_str(), res);
        }
    }

    // 迭代
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let alphas: HashMap<char, &str> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .cloned()
        .collect();

        if digits.is_empty() {
            return vec![];
        }
        let mut ans = vec![String::new()];
        for digit in digits.chars() {
            if let Some(&sufs) = alphas.get(&digit) {
                ans = sufs
                    .chars()
                    .flat_map(|suf| ans.iter().map(move |s| format!("{}{}", s, suf)))
                    .collect();
            }
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
