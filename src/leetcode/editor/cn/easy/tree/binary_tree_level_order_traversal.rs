//给你一个二叉树，请你返回其按 层序遍历 得到的节点值。 （即逐层地，从左到右访问所有节点）。
//
//
//
// 示例：
//二叉树：[3,9,20,null,null,15,7],
//
//     3
//   / \
//  9  20
//    /  \
//   15   7
//
//
// 返回其层次遍历结果：
//
// [
//  [3],
//  [9,20],
//  [15,7]
//]
//
// Related Topics 树 广度优先搜索
// 👍 585 👎 0

use std::cell::RefCell;
use std::rc::Rc;

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
use crate::leetcode::editor::cn::TreeNode::TreeNode;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_binary_tree_level_order_traversal() {}
}

struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut que = match root {
            Some(node) => vec![node],
            None => vec![],
        };
        let mut res = vec![];
        while !que.is_empty() {
            let (mut level, mut temp_que) = (vec![], vec![]);
            for node in que {
                level.push(node.borrow().val);
                if let Some(left_node) = node.borrow().left.clone() {
                    temp_que.push(left_node);
                }
                if let Some(right_node) = node.borrow().right.clone() {
                    temp_que.push(right_node);
                }
            }
            res.push(level);
            que = temp_que;
        }
        return res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
