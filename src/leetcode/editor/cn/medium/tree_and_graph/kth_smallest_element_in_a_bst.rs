//给定一个二叉搜索树，编写一个函数 kthSmallest 来查找其中第 k 个最小的元素。
//
// 说明：
//你可以假设 k 总是有效的，1 ≤ k ≤ 二叉搜索树元素个数。
//
// 示例 1:
//
// 输入: root = [3,1,4,null,2], k = 1
//   3
//  / \
// 1   4
//  \
//   2
//输出: 1
//
// 示例 2:
//
// 输入: root = [5,3,6,2,4,null,null,1], k = 3
//       5
//      / \
//     3   6
//    / \
//   2   4
//  /
// 1
//输出: 3
//
// 进阶：
//如果二叉搜索树经常被修改（插入/删除操作）并且你需要频繁地查找第 k 小的值，你将如何优化 kthSmallest 函数？
// Related Topics 树 二分查找
// 👍 338 👎 0

use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::editor::cn::TreeNode::TreeNode;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_kth_smallest_element_in_a_bst() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn kth_smallest(root: Node, k: i32) -> i32 {
        let mut curr = 0;
        return Solution::kth_smallest2(&root, &mut curr, k);
    }

    fn kth_smallest2(root: &Node, curr: &mut i32, k: i32) -> i32 {
        if let Some(node) = root {
            let left_val = Solution::kth_smallest2(&node.borrow().left.clone(), curr, k);
            if *curr == k {
                return left_val;
            }
            *curr = *curr + 1;
            if *curr == k {
                return node.borrow().val;
            }
            return Solution::kth_smallest2(&node.borrow().right.clone(), curr, k);
        }
        0
    }
}
//leetcode submit region end(Prohibit modification and deletion)
