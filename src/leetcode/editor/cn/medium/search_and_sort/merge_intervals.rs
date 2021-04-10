//给出一个区间的集合，请合并所有重叠的区间。
//
//
//
// 示例 1:
//
// 输入: intervals = [[1,3],[2,6],[8,10],[15,18]]
//输出: [[1,6],[8,10],[15,18]]
//解释: 区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
//
//
// 示例 2:
//
// 输入: intervals = [[1,4],[4,5]]
//输出: [[1,5]]
//解释: 区间 [1,4] 和 [4,5] 可被视为重叠区间。
//
// 注意：输入类型已于2019年4月15日更改。 请重置默认代码定义以获取新方法签名。
//
//
//
// 提示：
//
//
// intervals[i][0] <= intervals[i][1]
//
// Related Topics 排序 数组
// 👍 785 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_merge_intervals() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn merge(mut arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut res = vec![];
        let len = arr.len() - 1;
        for i in 0..len {
            // 如果可以则更新后一个，不行则直接添加到结果
            if arr[i][1] >= arr[i + 1][0] {
                arr[i + 1][0] = arr[i][0];
                arr[i + 1][1] = arr[i][1].max(arr[i + 1][1]);
            } else {
                res.push(arr[i].clone());
            }
        }
        res.push((arr.last().unwrap()).clone());
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
