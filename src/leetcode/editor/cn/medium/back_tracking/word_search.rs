//给定一个二维网格和一个单词，找出该单词是否存在于网格中。
//
// 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。
//
//
//
// 示例:
//
// board =
//[
//  ['A','B','C','E'],
//  ['S','F','C','S'],
//  ['A','D','E','E']
//]
//
//给定 word = "ABCCED", 返回 true
//给定 word = "SEE", 返回 true
//给定 word = "ABCB", 返回 false
//
//
//
// 提示：
//
//
// board 和 word 中只包含大写和小写英文字母。
// 1 <= board.length <= 200
// 1 <= board[i].length <= 200
// 1 <= word.length <= 10^3
//
// Related Topics 数组 回溯算法
// 👍 752 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_word_search() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        for x in 0..board.len() {
            for y in 0..board[x].len() {
                if Self::_exist(&mut board, word.as_bytes(), 0, x, y) {
                    return true;
                }
            }
        }
        return false;
    }

    fn _exist(board: &mut Vec<Vec<char>>, word: &[u8], index: usize, x: usize, y: usize) -> bool {
        if x >= board.len()
            || y >= board[x].len()
            || board[x][y] == b' ' as char
            || index >= word.len()
            || word[index] as char != board[x][y]
        {
            return false;
        }
        if index == word.len() - 1 && board[x][y] == word[index] as char {
            return true;
        }
        let origin = board[x][y];
        board[x][y] = b' ' as char;
        if x != 0 && Self::_exist(board, word, index + 1, x - 1, y) {
            board[x][y] = origin;
            return true;
        }
        if y != 0 && Self::_exist(board, word, index + 1, x, y - 1) {
            board[x][y] = origin;
            return true;
        }
        if Self::_exist(board, word, index + 1, x + 1, y) {
            board[x][y] = origin;
            return true;
        }
        if Self::_exist(board, word, index + 1, x, y + 1) {
            board[x][y] = origin;
            return true;
        }
        board[x][y] = origin;
        return false;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
