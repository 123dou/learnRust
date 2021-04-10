//给定一个Excel表格中的列名称，返回其相应的列序号。
//
// 例如，
//
//     A -> 1
//    B -> 2
//    C -> 3
//    ...
//    Z -> 26
//    AA -> 27
//    AB -> 28
//    ...
//
//
// 示例 1:
//
// 输入: "A"
//输出: 1
//
//
// 示例 2:
//
// 输入: "AB"
//输出: 28
//
//
// 示例 3:
//
// 输入: "ZY"
//输出: 701
//
// 致谢：
//特别感谢 @ts 添加此问题并创建所有测试用例。
// Related Topics 数学
// 👍 199 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_excel_sheet_column_number() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut res = 0;
        let bys = s.as_bytes();
        let fac: i32 = 26;
        for i in 0..bys.len() {
            let idx = bys.len() - 1 - i;
            res += (bys[idx] - b'A' + 1) as i32 * fac.pow(i as u32);
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
