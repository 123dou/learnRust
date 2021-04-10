//数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
//
//
//
// 示例 1：
//
//
//输入：n = 3
//输出：["((()))","(()())","(())()","()(())","()()()"]
//
//
// 示例 2：
//
//
//输入：n = 1
//输出：["()"]
//
//
//
//
// 提示：
//
//
// 1 <= n <= 8
//
// Related Topics 字符串 回溯算法
// 👍 1527 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_generate_parentheses() {
        let num = 3;
        println!("solution = {:?}", Solution::generate_parenthesis(num));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec![];
        }
        let mut res = vec![];
        Self::gen_parenthesis(0, 0, &mut res, n, "");
        return res;
    }
    fn gen_parenthesis(left: i32, right: i32, res: &mut Vec<String>, num: i32, s: &str) {
        if left == num && right == num {
            res.push(s.to_string());
            return;
        }
        if left > right {
            Solution::gen_parenthesis(left, right + 1, res, num, format!("{}{}", s, ")").as_str());
        }
        if left != num {
            Solution::gen_parenthesis(left + 1, right, res, num, format!("{}{}", s, "(").as_str());
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
