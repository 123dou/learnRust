//给定一个包含红色、白色和蓝色，一共 n 个元素的数组，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。
//
// 此题中，我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。
//
//
//
//
//
//
// 示例 1：
//
//
//输入：nums = [2,0,2,1,1,0]
//输出：[0,0,1,1,2,2]
//
//
// 示例 2：
//
//
//输入：nums = [2,0,1]
//输出：[0,1,2]
//
//
// 示例 3：
//
//
//输入：nums = [0]
//输出：[0]
//
//
// 示例 4：
//
//
//输入：nums = [1]
//输出：[1]
//
//
//
//
// 提示：
//
//
// n == nums.length
// 1 <= n <= 300
// nums[i] 为 0、1 或 2
//
//
//
//
// 进阶：
//
//
// 你可以不使用代码库中的排序函数来解决这道题吗？
// 你能想出一个仅使用常数空间的一趟扫描算法吗？
//
// Related Topics 排序 数组 双指针
// 👍 762 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_sort_colors() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        let mut i = 0;
        while i < nums.len() {
            if nums[i] > 1 && i < hi {
                nums.swap(i, hi);
                hi -= 1;
            } else if nums[i] < 1 {
                nums.swap(i, lo);
                lo += 1;
                i += 1;
            } else {
                i += 1;
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
