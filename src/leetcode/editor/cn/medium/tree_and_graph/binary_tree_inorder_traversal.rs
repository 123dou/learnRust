//给定一个二叉树的根节点 root ，返回它的 中序 遍历。
//
//
//
// 示例 1：
//
//
//输入：root = [1,null,2,3]
//输出：[1,3,2]
//
//
// 示例 2：
//
//
//输入：root = []
//输出：[]
//
//
// 示例 3：
//
//
//输入：root = [1]
//输出：[1]
//
//
// 示例 4：
//
//
//输入：root = [1,2]
//输出：[2,1]
//
//
// 示例 5：
//
//
//输入：root = [1,null,2]
//输出：[1,2]
//
//
//
//
// 提示：
//
//
// 树中节点数目在范围 [0, 100] 内
// -100 <= Node.val <= 100
//
//
//
//
// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
// Related Topics 栈 树 哈希表
// 👍 825 👎 0

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
use std::rc::Rc;

use crate::leetcode::editor::cn::TreeNode::TreeNode;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_binary_tree_inorder_traversal() {}
}

struct Solution {}

impl Solution {
    // 迭代
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut curr = root;
        while !stack.is_empty() || curr.is_some() {
            match curr {
                Some(node) => {
                    stack.push(node.clone());
                    curr = node.borrow().left.clone();
                }
                None => {
                    if let Some(node) = stack.pop() {
                        let ref_node = node.borrow();
                        res.push(ref_node.val);
                        curr = ref_node.right.clone();
                    }
                }
            }
        }
        res
    }
    // 递归
    pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Solution::inorder(&root, &mut res);
        res
    }

    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(rc_node) = root {
            let node = rc_node.borrow();
            Solution::inorder(&node.left, vec);
            vec.push(node.val);
            Solution::inorder(&node.right, vec);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
