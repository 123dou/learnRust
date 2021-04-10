//给定一个二叉树，判断其是否是一个有效的二叉搜索树。
//
// 假设一个二叉搜索树具有如下特征：
//
//
// 节点的左子树只包含小于当前节点的数。
// 节点的右子树只包含大于当前节点的数。
// 所有左子树和右子树自身必须也是二叉搜索树。
//
//
// 示例 1:
//
// 输入:
//    2
//   / \
//  1   3
//输出: true
//
//
// 示例 2:
//
// 输入:
//    5
//   / \
//  1   4
//     / \
//    3   6
//输出: false
//解释: 输入为: [5,1,4,null,null,3,6]。
//     根节点的值为 5 ，但是其右子节点值为 4 。
//
// Related Topics 树 深度优先搜索
// 👍 686 👎 0

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

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

//noinspection ALL
//leetcode submit region begin(Prohibit modification and deletion)
use crate::leetcode::editor::cn::TreeNode::TreeNode;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_validate_binary_search_tree() {}
}

struct Solution {}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // return Solution::is_valid_bst3(root, None, None);
        return Solution::is_valid_bst4(root, &mut true, &mut 0);
    }

    pub fn is_valid_bst3(
        root: Option<Rc<RefCell<TreeNode>>>,
        lower: Option<i32>,
        upper: Option<i32>,
    ) -> bool {
        match root {
            Some(node) => {
                let curr_node = node.as_ref().borrow();
                lower.map_or(true, |x| x < curr_node.val)
                    && upper.map_or(true, |x| x > curr_node.val)
                    && Solution::is_valid_bst3(curr_node.left.clone(), lower, Some(curr_node.val))
                    && Solution::is_valid_bst3(curr_node.right.clone(), Some(curr_node.val), upper)
            }
            None => true,
        }
    }

    pub fn is_valid_bst2(
        root: Option<Rc<RefCell<TreeNode>>>,
        is_most_left: &mut bool,
        pre_val: &mut i32,
    ) -> bool {
        if root.is_none() {
            return true;
        }
        let curr_node_rc_ref = root.unwrap();
        let curr_node = curr_node_rc_ref.as_ref().borrow();
        let left_res = Solution::is_valid_bst2(curr_node.left.clone(), is_most_left, pre_val);
        if !left_res {
            return false;
        }
        let curr_val = curr_node.val;
        if *is_most_left {
            *is_most_left = false;
        } else {
            if *pre_val >= curr_val {
                return false;
            }
        }
        *pre_val = curr_val;
        return Solution::is_valid_bst2(curr_node.right.clone(), is_most_left, pre_val);
    }

    pub fn is_valid_bst4(
        root: Option<Rc<RefCell<TreeNode>>>,
        is_most_left: &mut bool,
        pre_val: &mut i32,
    ) -> bool {
        match root {
            None => true,
            Some(node) => {
                let curr_node = node.as_ref().borrow();
                let curr_val = curr_node.val;
                if !Solution::is_valid_bst4(curr_node.left.clone(), is_most_left, pre_val) {
                    return false;
                }
                if *is_most_left {
                    *is_most_left = false;
                } else {
                    if *pre_val >= curr_val {
                        return false;
                    }
                }
                *pre_val = curr_val;
                return Solution::is_valid_bst4(curr_node.right.clone(), is_most_left, pre_val);
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
