//给定一个字符串数组，将字母异位词组合在一起。字母异位词指字母相同，但排列不同的字符串。
//
// 示例:
//
// 输入: ["eat", "tea", "tan", "ate", "nat", "bat"]
//输出:
//[
//  ["ate","eat","tea"],
//  ["nat","tan"],
//  ["bat"]
//]
//
// 说明：
//
//
// 所有输入均为小写字母。
// 不考虑答案输出的顺序。
//
// Related Topics 哈希表 字符串
// 👍 518 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_group_anagrams() {}
}

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        let mut res = vec![];
        for str in strs.iter() {
            let mut key = Vec::from(str.as_str());
            key.sort();
            if let Some(val) = map.get_mut(&key) {
                val.push(String::from(str));
            } else {
                let ve = vec![String::from(str)];
                map.insert(key, ve);
            }
        }
        for (_, val) in map {
            res.push(val);
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
