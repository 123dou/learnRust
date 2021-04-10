//在未排序的数组中找到第 k 个最大的元素。请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。
//
// 示例 1:
//
// 输入: [3,2,1,5,6,4] 和 k = 2
//输出: 5
//
//
// 示例 2:
//
// 输入: [3,2,3,1,2,4,5,5,6] 和 k = 4
//输出: 4
//
// 说明:
//
// 你可以假设 k 总是有效的，且 1 ≤ k ≤ 数组的长度。
// Related Topics 堆 分治算法
// 👍 862 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_kth_largest_element_in_an_array() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_kth_largest2(mut arr: Vec<i32>, k: i32) -> i32 {
        arr.sort();
        return arr[arr.len() - k as usize];
    }
    pub fn find_kth_largest(mut min_heap: Vec<i32>, k: i32) -> i32 {
        for i in 0..k as usize {
            let mut child = i;
            while child > 0 {
                let parent = (child - 1) / 2;
                if min_heap[parent] > min_heap[child] {
                    min_heap.swap(child, parent);
                    child = parent;
                } else {
                    break;
                }
            }
        }
        for i in k as usize..min_heap.len() {
            if min_heap[i] > min_heap[0] {
                min_heap[0] = min_heap[i];
                let mut parent = 0;
                let mut child = parent * 2 + 1;
                while child < k as usize {
                    if child + 1 < k as usize && min_heap[child + 1] < min_heap[child] {
                        child += 1;
                    }
                    if min_heap[parent] > min_heap[child] {
                        min_heap.swap(parent, child);
                    }
                    parent = child;
                    child = parent * 2 + 1;
                }
            }
        }
        min_heap[0]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
