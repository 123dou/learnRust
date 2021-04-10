//给定一个非负整数 numRows，生成杨辉三角的前 numRows 行。
//
//
//
// 在杨辉三角中，每个数是它左上方和右上方的数的和。
//
// 示例:
//
// 输入: 5
//输出:
//[
//     [1],
//    [1,1],
//   [1,2,1],
//  [1,3,3,1],
// [1,4,6,4,1]
//]
// Related Topics 数组
// 👍 374 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_pascals_triangle() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            0 => vec![],
            1 => vec![vec![1]],
            _ => {
                let mut res = vec![vec![1]];
                for i in 1..num_rows as usize {
                    let mut ve = vec![1; (i + 1) as usize];
                    for j in 1..ve.len() - 1 {
                        ve[j] = res[i - 1][j] + res[i - 1][j - 1];
                    }
                    res.push(ve);
                }
                res
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
