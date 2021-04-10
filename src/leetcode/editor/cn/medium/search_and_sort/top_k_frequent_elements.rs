//给定一个非空的整数数组，返回其中出现频率前 k 高的元素。
//
//
//
// 示例 1:
//
// 输入: nums = [1,1,1,2,2,3], k = 2
//输出: [1,2]
//
//
// 示例 2:
//
// 输入: nums = [1], k = 1
//输出: [1]
//
//
//
// 提示：
//
//
// 你可以假设给定的 k 总是合理的，且 1 ≤ k ≤ 数组中不相同的元素的个数。
// 你的算法的时间复杂度必须优于 O(n log n) , n 是数组的大小。
// 题目数据保证答案唯一，换句话说，数组中前 k 个高频元素的集合是唯一的。
// 你可以按任意顺序返回答案。
//
// Related Topics 堆 哈希表
// 👍 620 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent_elements() {
        let v = vec![1, 1, 1, 2, 2, 3];
        println!("solutino = {:?}", Solution::top_k_frequent(v, 2));
    }
}

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut key_freq_map: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut res = vec![];
        for i in nums.iter() {
            if key_freq_map.contains_key(i) {
                if let Some(temp) = key_freq_map.get_mut(i) {
                    temp.1 = temp.1 + 1;
                }
            } else {
                key_freq_map.insert(*i, (*i, 1));
            }
        }
        let mut key_freq = key_freq_map
            .values()
            .map(|(key, val)| (*key, *val))
            .collect::<Vec<(i32, i32)>>();
        key_freq.sort_by_key(|var| var.1);
        for i in (key_freq.len() - k as usize..=key_freq.len() - 1).rev() {
            res.push(key_freq[i].0);
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
