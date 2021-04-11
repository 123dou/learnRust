//给定一个 n × n 的二维矩阵表示一个图像。
//
// 将图像顺时针旋转 90 度。
//
// 说明：
//
// 你必须在原地旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要使用另一个矩阵来旋转图像。
//
// 示例 1:
//
// 给定 matrix =
//[
//  [1,2,3],
//  [4,5,6],
//  [7,8,9]
//],
//
//原地旋转输入矩阵，使其变为:
//[
//  [7,4,1],
//  [8,5,2],
//  [9,6,3]
//]
//
//
// 示例 2:
//
// 给定 matrix =
//[
//  [ 5, 1, 9,11],
//  [ 2, 4, 8,10],
//  [13, 3, 6, 7],
//  [15,14,12,16]
//],
//
//原地旋转输入矩阵，使其变为:
//[
//  [15,13, 2, 5],
//  [14, 3, 4, 1],
//  [12, 6, 8, 9],
//  [16, 7,10,11]
//]
//
// Related Topics 数组
// 👍 493 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_rotate_image() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            // 行
            for j in i..n - i - 1 {
                // 列
                let mut val = matrix[i][j];
                let mut pos = (i, j);
                for _ in 0..4 {
                    pos = Solution::next_pos(pos, n - 1);
                    std::mem::swap(&mut val, &mut matrix[pos.0][pos.1]);
                }
            }
        }
    }

    pub fn next_pos(pos: (usize, usize), n: usize) -> (usize, usize) {
        (pos.1, n - pos.0)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
