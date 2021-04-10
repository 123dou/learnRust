//将一个按照升序排列的有序数组，转换为一棵高度平衡二叉搜索树。
//
// 本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。
//
// 示例:
//
// 给定有序数组: [-10,-3,0,5,9],
//
//一个可能的答案是：[0,-3,9,-10,null,5]，它可以表示下面这个高度平衡二叉搜索树：
//
//      0
//     / \
//   -3   9
//   /   /
// -10  5
//
// Related Topics 树 深度优先搜索
// 👍 535 👎 0

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

use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::editor::cn::TreeNode::TreeNode;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_convert_sorted_array_to_binary_search_tree() {
        let nums = vec![-10, -3, 0, 5, 9];
        Solution::sorted_array_to_bst(nums);
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match nums.len() {
            0 => None,
            _ => Solution::to_bst(&nums, 0, (nums.len() - 1) as i32),
        }
    }

    pub fn to_bst(nums: &Vec<i32>, lo: i32, hi: i32) -> Node {
        if lo > hi {
            return None;
        }
        let mid = lo + (hi - lo) / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));
        node.borrow_mut().left = Solution::to_bst(nums, lo, mid - 1);
        node.borrow_mut().right = Solution::to_bst(nums, mid + 1, hi);
        return Some(node);
    }
}
//leetcode submit region end(Prohibit modification and deletion)
