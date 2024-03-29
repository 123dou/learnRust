//判断一个 9x9 的数独是否有效。只需要根据以下规则，验证已经填入的数字是否有效即可。
//
//
// 数字 1-9 在每一行只能出现一次。
// 数字 1-9 在每一列只能出现一次。
// 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
//
//
//
//
// 上图是一个部分填充的有效的数独。
//
// 数独部分空格内已填入了数字，空白格用 '.' 表示。
//
// 示例 1:
//
// 输入:
//[
//  ["5","3",".",".","7",".",".",".","."],
//  ["6",".",".","1","9","5",".",".","."],
//  [".","9","8",".",".",".",".","6","."],
//  ["8",".",".",".","6",".",".",".","3"],
//  ["4",".",".","8",".","3",".",".","1"],
//  ["7",".",".",".","2",".",".",".","6"],
//  [".","6",".",".",".",".","2","8","."],
//  [".",".",".","4","1","9",".",".","5"],
//  [".",".",".",".","8",".",".","7","9"]
//]
//输出: true
//
//
// 示例 2:
//
// 输入:
//[
//  ["8","3",".",".","7",".",".",".","."],
//  ["6",".",".","1","9","5",".",".","."],
//  [".","9","8",".",".",".",".","6","."],
//  ["8",".",".",".","6",".",".",".","3"],
//  ["4",".",".","8",".","3",".",".","1"],
//  ["7",".",".",".","2",".",".",".","6"],
//  [".","6",".",".",".",".","2","8","."],
//  [".",".",".","4","1","9",".",".","5"],
//  [".",".",".",".","8",".",".","7","9"]
//]
//输出: false
//解释: 除了第一行的第一个数字从 5 改为 8 以外，空格内其他数字均与 示例1 相同。
//     但由于位于左上角的 3x3 宫内有两个 8 存在, 因此这个数独是无效的。
//
// 说明:
//
//
// 一个有效的数独（部分已被填充）不一定是可解的。
// 只需要根据以上规则，验证已经填入的数字是否有效即可。
// 给定数独序列只包含数字 1-9 和字符 '.' 。
// 给定数独永远是 9x9 形式的。
//
// Related Topics 哈希表
// 👍 373 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_valid_sudoku() {
        let vec = vec![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'].to_vec(),
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'].to_vec(),
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'].to_vec(),
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'].to_vec(),
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'].to_vec(),
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'].to_vec(),
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'].to_vec(),
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'].to_vec(),
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'].to_vec(),
        ];
        println!("{:?}", Solution::is_valid_sudoku(vec));
    }
}

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for vec in board.iter() {
            if !Solution::check_dup(vec) {
                return false;
            }
        }
        for i in 0..board.len() {
            let mut vec = vec![];
            for j in 0..board[i].len() {
                vec.push(board[j][i]);
            }
            if !Solution::check_dup(&vec) {
                return false;
            }
        }
        for i in (0..9).filter(|x| x % 3 == 0) {
            for j in (0..9).filter(|x| x % 3 == 0) {
                let mut vec = vec![];
                for n in i..i + 3 {
                    for m in j..j + 3 {
                        vec.push(board[n][m]);
                    }
                }
                if !Solution::check_dup(&vec) {
                    return false;
                }
            }
        }
        true
    }

    pub fn check_dup(vec: &[char]) -> bool {
        let mut set = HashSet::new();
        for val in vec.iter() {
            if val.ne(&'.') && set.contains(val) {
                return false;
            } else {
                set.insert(val);
            }
        }
        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
