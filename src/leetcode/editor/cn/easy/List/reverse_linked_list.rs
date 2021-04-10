//反转一个单链表。
//
// 示例:
//
// 输入: 1->2->3->4->5->NULL
//输出: 5->4->3->2->1->NULL
//
// 进阶:
//你可以迭代或递归地反转链表。你能否用两种方法解决这道题？
// Related Topics 链表
// 👍 1125 👎 0

use crate::leetcode::editor::cn::ListNode::ListNode;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_reverse_linked_list() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut last = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            // node 是一个 Box 指针, 这里省略了 *
            cur = node.next.take();
            node.next = last;
            last = Some(node);
        }
        return last;
    }
}

// pub fn reverse_list2(pre: &mut Option<Box<ListNode>>, curr: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     if curr.unwrap().next.is_none() {
//         curr.unwrap().next = *pre;
//     }
//     None
// }
// leetcode submit region end(Prohibit modification and deletion)
