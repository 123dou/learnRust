//给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1。
//
//
//
// 示例：
//
// s = "leetcode"
//返回 0
//
//s = "loveleetcode"
//返回 2
//
//
//
//
// 提示：你可以假定该字符串只包含小写字母。
// Related Topics 哈希表 字符串
// 👍 238 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_first_unique_character_in_a_string() {
        let my_str = String::from("leetcode");
        println!("{:?}", Solution::first_uniq_char(my_str));
    }
}

struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut my_map = HashMap::new();
        // 语句一定要有分号结尾,不然会当作表达式返回
        s.chars().into_iter().for_each(|x| {
            if my_map.contains_key(&x) {
                my_map.insert(x, 1);
            } else {
                my_map.insert(x, 0);
            }
        });
        // 集合取出来的基本都是引用
        for (i, val) in s.chars().into_iter().enumerate() {
            if my_map.get(&val).unwrap() == &0 {
                return i as i32;
            }
        }
        return -1;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
