//给定一个二叉树，返回其节点值的锯齿形层序遍历。（即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。
//
// 例如：
//给定二叉树 [3,9,20,null,null,15,7],
//
//
//    3
//   / \
//  9  20
//    /  \
//   15   7
//
//
// 返回锯齿形层序遍历如下：
//
//
//[
//  [3],
//  [20,9],
//  [15,7]
//]
//
// Related Topics 栈 树 广度优先搜索
// 👍 371 👎 0

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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::editor::cn::TreeNode::TreeNode;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_zigzag_level_order_traversal() {
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node9 = Rc::new(RefCell::new(TreeNode::new(9)));
        let node20 = Rc::new(RefCell::new(TreeNode::new(20)));
        let node15 = Rc::new(RefCell::new(TreeNode::new(15)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
        node20.borrow_mut().left = Some(node15);
        node20.borrow_mut().right = Some(node7);
        node3.borrow_mut().left = Some(node9);
        node3.borrow_mut().right = Some(node20);
        let root = Some(node3);
        println!("root = {:#?}", root);
        println!("solution = {:?}", Solution::zigzag_level_order(root));
    }
}

struct Solution {}

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn zigzag_level_order(root: Node) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }
        let mut level_vec = vec![];
        let mut deque = VecDeque::new();
        let curr = root;
        let mut order = true;
        deque.push_back(curr);
        deque.push_back(None);
        loop {
            if order {
                if let Some(op_node) = deque.pop_front() {
                    if let Some(node) = op_node {
                        level_vec.push(node.borrow().val);
                        let left_node = node.borrow().left.clone();
                        let right_node = node.borrow().right.clone();
                        if left_node.is_some() {
                            deque.push_back(left_node);
                        }
                        if right_node.is_some() {
                            deque.push_back(right_node);
                        }
                    } else {
                        res.push(level_vec);
                        level_vec = vec![];
                        if deque.is_empty() {
                            break;
                        }
                        deque.push_front(None);
                        order = !order;
                    }
                }
            } else {
                if let Some(op_node) = deque.pop_back() {
                    if let Some(node) = op_node {
                        level_vec.push(node.borrow().val);
                        let left_node = node.borrow().left.clone();
                        let right_node = node.borrow().right.clone();
                        if right_node.is_some() {
                            deque.push_front(right_node);
                        }
                        if left_node.is_some() {
                            deque.push_front(left_node);
                        }
                    } else {
                        res.push(level_vec);
                        level_vec = vec![];
                        if deque.is_empty() {
                            break;
                        }
                        deque.push_back(None);
                        order = !order;
                    }
                }
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
