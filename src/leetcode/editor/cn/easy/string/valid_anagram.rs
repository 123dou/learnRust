//给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
//
// 示例 1:
//
// 输入: s = "anagram", t = "nagaram"
//输出: true
//
//
// 示例 2:
//
// 输入: s = "rat", t = "car"
//输出: false
//
// 说明:
//你可以假设字符串只包含小写字母。
//
// 进阶:
//如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？
// Related Topics 排序 哈希表
// 👍 219 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_valid_anagram() {
        let s = "a".to_string();
        let t = "b".to_string();
        println!("{}", Solution::is_anagram(s, t));
        let v = vec!['h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8];
        let s = String::from_utf8(v);
        println!("{:?}", s);
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut a = s.clone();
        let mut b = t.clone();
        let av = unsafe { a.as_mut_vec() };
        let bv = unsafe { b.as_mut_vec() };
        av.sort();
        bv.sort();
        return av.eq(&bv);
    }
}
//leetcode submit region end(Prohibit modification and deletion)
