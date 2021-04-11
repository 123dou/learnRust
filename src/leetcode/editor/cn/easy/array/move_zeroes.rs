//给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
//
// 示例:
//
// 输入: [0,1,0,3,12]
//输出: [1,3,12,0,0]
//
// 说明:
//
//
// 必须在原数组上操作，不能拷贝额外的数组。
// 尽量减少操作次数。
//
// Related Topics 数组 双指针
// 👍 644 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);
        println!("{:?}", v);
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        }
        let mut zero = 0;
        let mut not_zero = 0;
        while zero < nums.len() && not_zero < nums.len() {
            if nums[zero] == 0 {
                while not_zero < nums.len() {
                    if nums[not_zero] != 0 {
                        nums.swap(zero, not_zero);
                        break;
                    }
                    not_zero += 1;
                }
            }
            zero += 1;
            not_zero += 1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
