//给定一个单链表，把所有的奇数节点和偶数节点分别排在一起。请注意，这里的奇数节点和偶数节点指的是节点编号的奇偶性，
// 而不是节点的值的奇偶性。
//
// 请尝试使用原地算法完成。你的算法的空间复杂度应为 O(1)，时间复杂度应为 O(nodes)，nodes 为节点总数。
//
// 示例 1:
//
// 输入: 1->2->3->4->5->NULL
//输出: 1->3->5->2->4->NULL
//
//
// 示例 2:
//
// 输入: 2->1->3->5->6->4->7->NULL
//输出: 2->3->6->7->1->5->4->NULL
//
// 说明:
//
//
// 应当保持奇数节点和偶数节点的相对顺序。
// 链表的第一个节点视为奇数节点，第二个节点视为偶数节点，以此类推。
//
// Related Topics 链表
// 👍 358 👎 0

use crate::leetcode::editor::cn::ListNode::ListNode;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_odd_even_linked_list() {}
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
type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn odd_even_list(mut head: Node) -> Node {
        let mut res = None;
        let (mut h_left, mut h_right) = (&mut res, &mut head);
        let mut even = true;
        while (*h_left).is_some() || (*h_right).is_some() {
            if even {
                *h_left = (*h_right).take();
                h_left = &mut (*h_left).as_mut().unwrap().next;
            } else {
                *h_right = (*h_left).take();
                h_right = &mut (*h_right).as_mut().unwrap().next;
            }
            even = !even;
        }
        *h_left = head;
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
