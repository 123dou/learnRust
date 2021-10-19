//给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。
//
// 求在该柱状图中，能够勾勒出来的矩形的最大面积。
//
//
//
// 示例 1:
//
//
//
//
//输入：heights = [2,1,5,6,2,3]
//输出：10
//解释：最大的矩形为图中红色区域，面积为 10
//
//
// 示例 2：
//
//
//
//
//输入： heights = [2,4]
//输出： 4
//
//
//
// 提示：
//
//
// 1 <= heights.length <=105
// 0 <= heights[i] <= 104
//
// Related Topics 栈 数组 单调栈
// 👍 1473 👎 0

use std::collections::VecDeque;
use std::process::id;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_in_histogram() {
        let temp = vec![2, 1, 5, 6, 2, 3];
        let i = Solution::largest_rectangle_area(temp);
        println!("i = {:?}", i);
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        if heights.is_empty() {
            return 0;
        }
        let mut temp = vec![-1];
        temp.append(&mut heights);
        temp.push(-1);
        let mut res = 0i32;
        let mut stack: Vec<usize> = vec![];
        for i in 1..temp.len() {
            while temp[*stack.last().unwrap_or(&0)] > temp[i] {
                let temp_idx = stack.pop().unwrap_or(0);
                res = res.max(temp[temp_idx] * (i as i32 - *stack.last().unwrap_or(&0) as i32 - 1));
            }
            stack.push(i);
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
