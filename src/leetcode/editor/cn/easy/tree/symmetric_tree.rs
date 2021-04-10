//给定一个二叉树，检查它是否是镜像对称的。
//
//
//
// 例如，二叉树 [1,2,2,3,4,4,3] 是对称的。
//
//     1
//   / \
//  2   2
// / \ / \
//3  4 4  3
//
//
//
//
// 但是下面这个 [1,2,2,null,3,null,3] 则不是镜像对称的:
//
//     1
//   / \
//  2   2
//   \   \
//   3    3
//
//
//
//
// 进阶：
//
// 你可以运用递归和迭代两种方法解决这个问题吗？
// Related Topics 树 深度优先搜索 广度优先搜索
// 👍 943 👎 0

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

//noinspection ALL
//leetcode submit region begin(Prohibit modification and deletion)
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::editor::cn::TreeNode::TreeNode;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_symmetric_tree() {}
}

struct Solution {}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.map_or(true, |node| {
            Solution::is_symmetric2(
                node.as_ref().borrow().left.clone(),
                node.as_ref().borrow().right.clone(),
            )
        })
    }

    pub fn is_symmetric2(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let (None, None) = (left.as_ref(), right.as_ref()) {
            return true;
        }
        if let (Some(left_node), Some(right_node)) = (&left, &right) {
            return left_node
                .as_ref()
                .borrow()
                .val
                .eq(&right_node.as_ref().borrow().val)
                && Solution::is_symmetric2(
                    left_node.as_ref().borrow().left.clone(),
                    right_node.as_ref().borrow().right.clone(),
                )
                && Solution::is_symmetric2(
                    left_node.as_ref().borrow().right.clone(),
                    right_node.as_ref().borrow().left.clone(),
                );
        }
        return false;
    }

    pub fn is_symmetric3(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type Tree = Option<Rc<RefCell<TreeNode>>>;
        fn is_sym(p: Tree, q: Tree) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    let (mut p, mut q) = (p.borrow_mut(), q.borrow_mut());
                    p.val == q.val
                        && is_sym(p.left.take(), q.right.take())
                        && is_sym(p.right.take(), q.left.take())
                }
                _ => false,
            }
        }
        match root {
            None => true,
            Some(root) => {
                let mut root = root.borrow_mut();
                is_sym(root.left.take(), root.right.take())
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
