//给定一个数组，将数组中的元素向右移动 k 个位置，其中 k 是非负数。
//
// 示例 1:
//
// 输入: [1,2,3,4,5,6,7] 和 k = 3
//输出: [5,6,7,1,2,3,4]
//解释:
//向右旋转 1 步: [7,1,2,3,4,5,6]
//向右旋转 2 步: [6,7,1,2,3,4,5]
//向右旋转 3 步: [5,6,7,1,2,3,4]
//
//
// 示例 2:
//
// 输入: [-1,-100,3,99] 和 k = 2
//输出: [3,99,-1,-100]
//解释:
//向右旋转 1 步: [99,-1,-100,3]
//向右旋转 2 步: [3,99,-1,-100]
//
// 说明:
//
//
// 尽可能想出更多的解决方案，至少有三种不同的方法可以解决这个问题。
// 要求使用空间复杂度为 O(1) 的 原地 算法。
//
// Related Topics 数组
// 👍 617 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_rotate_array() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        println!("{:?}", v.clone());
        Solution::rotate(v.as_mut(), 14);
        println!("{:?}", v);
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = (k % nums.len() as i32) as usize;
        if k == 0 {
            return;
        }
        let left = nums.len() - k;
        let left_slice = &mut nums[..left];
        left_slice.reverse();
        let right_slice = &mut nums[left..];
        right_slice.reverse();
        nums.reverse();
    }

    pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
        let k = (k % nums.len() as i32) as usize;
        if k == 0 {
            return;
        }
        Solution::reverse(nums, 0, nums.len() - 1 - k);
        Solution::reverse(nums, nums.len() - k, nums.len() - 1);
        Solution::reverse(nums, 0, nums.len() - 1);
    }

    pub fn reverse(nums: &mut Vec<i32>, low: usize, hi: usize) {
        let mut temp_low = low;
        let mut temp_hi = hi;
        while temp_low < temp_hi {
            nums.swap(temp_hi, temp_low);
            temp_low += 1;
            temp_hi -= 1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
