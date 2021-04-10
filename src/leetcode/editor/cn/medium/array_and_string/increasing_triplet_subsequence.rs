//给定一个未排序的数组，判断这个数组中是否存在长度为 3 的递增子序列。
//
// 数学表达式如下:
//
// 如果存在这样的 i, j, k, 且满足 0 ≤ i < j < k ≤ n-1，
//使得 arr[i] < arr[j] < arr[k] ，返回 true ; 否则返回 false 。
//
// 说明: 要求算法的时间复杂度为 O(n)，空间复杂度为 O(1) 。
//
// 示例 1:
//
// 输入: [1,2,3,4,5]
//输出: true
//
//
// 示例 2:
//
// 输入: [5,4,3,2,1]
//输出: false
// 👍 241 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_increasing_triplet_subsequence() {
        let ve = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, 3,
        ];
        println!("{}", Solution::increasing_triplet(ve));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        return match nums.len() {
            0 | 1 | 2 => false,
            _ => {
                let mut first_num = nums[0];
                let mut second_num_op = None;
                for val in nums {
                    if let Some(second_num) = second_num_op {
                        if val > second_num {
                            return true;
                        } else if val < first_num {
                            first_num = val;
                        } else if val > first_num && val < second_num {
                            second_num_op = Some(val);
                        }
                    } else {
                        if val < first_num {
                            first_num = val;
                        } else if val > first_num {
                            second_num_op = Some(val);
                        }
                    }
                }
                return false;
            }
        };
    }
}
//leetcode submit region end(Prohibit modification and deletion)
