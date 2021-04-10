//给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。
//
// 岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。
//
// 此外，你可以假设该网格的四条边均被水包围。
//
//
//
// 示例 1：
//
//
//输入：grid = [
//  ["1","1","1","1","0"],
//  ["1","1","0","1","0"],
//  ["1","1","0","0","0"],
//  ["0","0","0","0","0"]
//]
//输出：1
//
//
// 示例 2：
//
//
//输入：grid = [
//  ["1","1","0","0","0"],
//  ["1","1","0","0","0"],
//  ["0","0","1","0","0"],
//  ["0","0","0","1","1"]
//]
//输出：3
//
//
//
//
// 提示：
//
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 300
// grid[i][j] 的值为 '0' 或 '1'
//
// Related Topics 深度优先搜索 广度优先搜索 并查集
// 👍 941 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_number_of_islands() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] == b'1' as char {
                    islands = islands + 1;
                    Solution::dfs(&mut grid, (x, y));
                }
            }
        }
        islands
    }

    fn dfs(grid: &mut Vec<Vec<char>>, pos: (usize, usize)) {
        if pos.0 >= grid.len() || pos.1 >= grid[pos.0].len() || grid[pos.0][pos.1] != b'1' as char {
            return;
        }
        grid[pos.0][pos.1] = b' ' as char;
        if pos.0 != 0 {
            Solution::dfs(grid, (pos.0 - 1, pos.1));
        }
        if pos.1 != 0 {
            Solution::dfs(grid, (pos.0, pos.1 - 1));
        }
        Solution::dfs(grid, (pos.0 + 1, pos.1));
        Solution::dfs(grid, (pos.0, pos.1 + 1));
    }
}
//leetcode submit region end(Prohibit modification and deletion)
