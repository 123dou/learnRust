//根据一棵树的前序遍历与中序遍历构造二叉树。
//
// 注意:
//你可以假设树中没有重复的元素。
//
// 例如，给出
//
// 前序遍历 preorder = [3,9,20,15,7]
//中序遍历 inorder = [9,3,15,20,7]
//
// 返回如下的二叉树：
//
//     3
//   / \
//  9  20
//    /  \
//   15   7
// Related Topics 树 深度优先搜索 数组
// 👍 829 👎 0

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
    use super::*;

    #[test]
    fn test_construct_binary_tree_from_preorder_and_inorder_traversal() {
        let pre_order = vec![3, 9, 20, 15, 7];
        let in_order = vec![9, 3, 15, 20, 7];
        let option = Solution::build_tree(pre_order, in_order);
        println!("root = {:#?}", option);
    }
}

struct Solution {}

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        return Solution::build_tree2(&preorder, &inorder, 0, 0, inorder.len() - 1);
    }

    fn build_tree2(
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
        curr: usize,
        start: usize,
        end: usize,
    ) -> Node {
        let node = Rc::new(RefCell::new(TreeNode::new(preorder[curr])));
        let mut curr_in_inorder = end;
        for i in start..=end {
            if preorder[curr] == inorder[i] {
                curr_in_inorder = i;
                break;
            }
        }
        if curr_in_inorder != start {
            node.borrow_mut().left =
                Solution::build_tree2(preorder, inorder, curr + 1, start, curr_in_inorder - 1);
        }
        if curr_in_inorder != end {
            //此为揭示在中序遍历中找出此节点左子树一共有多少个节点，则此节点的右节点，必定是前序遍历中向右偏移offset个的那个，
            // 因为在先序遍历中左子树的所有节点一定出现在右节点前。则为preorder[i+1+offset];
            node.borrow_mut().right = Solution::build_tree2(
                preorder,
                inorder,
                curr_in_inorder + 1 + (curr - start),
                curr_in_inorder + 1,
                end,
            );
        }
        Some(node)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
