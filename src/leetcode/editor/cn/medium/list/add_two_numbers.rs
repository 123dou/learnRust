//给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
//
// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
//
// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
//
// 示例：
//
// 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
//输出：7 -> 0 -> 8
//原因：342 + 465 = 807
//
// Related Topics 链表 数学
// 👍 5260 👎 0

use std::option::Option::Some;

use crate::leetcode::editor::cn::ListNode::ListNode;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let node = ListNode::new(9);
        let mut list_node = ListNode::new(9);
        list_node.next = Some(Box::new(ListNode::new(9)));
        println!(
            "{:?}",
            Solution::add_two_numbers(Some(Box::new(node)), Some(Box::new(list_node)))
        );
    }
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
type List = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(l1: List, l2: List) -> List {
        let mut list = ListNode { val: 0, next: None };
        let mut count = 0;
        let mut head1 = &l1;
        let mut head2 = &l2;
        let mut curr = &mut list;
        while let (Some(node1), Some(node2)) = (head1.as_ref(), head2.as_ref()) {
            let mut val = node1.val + node2.val + count;
            if val > 9 {
                count = 1;
                val %= 10;
            } else {
                count = 0;
            }
            curr.next = Some(Box::new(ListNode::new(val)));
            head1 = &node1.next;
            head2 = &node2.next;
            curr = curr.next.as_mut().unwrap();
        }
        let mut head = if let Some(_) = head1 { head1 } else { head2 };
        while let Some(node) = head {
            let mut val = count + node.val;
            if val > 9 {
                val = 0;
            } else {
                count = 0
            }
            curr.next = Some(Box::new(ListNode::new(val)));
            head = &node.next;
            curr = curr.next.as_mut().unwrap();
        }
        if count == 1 {
            curr.next = Some(Box::new(ListNode::new(1)));
        }
        return list.next;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
