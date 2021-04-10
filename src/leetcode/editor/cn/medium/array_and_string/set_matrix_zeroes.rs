//给定一个 m x n 的矩阵，如果一个元素为 0，则将其所在行和列的所有元素都设为 0。请使用原地算法。
//
// 示例 1:
//
// 输入:
//[
//  [1,1,1],
//  [1,0,1],
//  [1,1,1]
//]
//输出:
//[
//  [1,0,1],
//  [0,0,0],
//  [1,0,1]
//]
//
//
// 示例 2:
//
// 输入:
//[
//  [0,1,2,0],
//  [3,4,5,2],
//  [1,3,1,5]
//]
//输出:
//[
//  [0,0,0,0],
//  [0,4,5,0],
//  [0,3,1,0]
//]
//
// 进阶:
//
// // 一个直接的解决方案是使用 O(mn) 的额外空间，但这并不是一个好的解决方案。 // 一个简单的改进方案是使用 O(m + n) 的额外空间，但这仍然不是最好的解决方案。 // 你能想出一个常数空间的解决方案吗？
//
// Related Topics 数组
// 👍 329 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_set_matrix_zeroes() {}
}

struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row_set = HashSet::new();
        let mut col_set = HashSet::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    row_set.insert(i);
                    col_set.insert(j);
                }
            }
        }
        for i in row_set {
            Solution::set_zero_of_row(matrix, i);
        }
        for i in col_set {
            Solution::set_zero_of_col(matrix, i);
        }
    }
    fn set_zero_of_row(arr: &mut Vec<Vec<i32>>, row: usize) {
        for i in 0..arr[row].len() {
            arr[row][i] = 0;
        }
    }
    fn set_zero_of_col(arr: &mut Vec<Vec<i32>>, col: usize) {
        for i in 0..arr.len() {
            arr[i][col] = 0;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
