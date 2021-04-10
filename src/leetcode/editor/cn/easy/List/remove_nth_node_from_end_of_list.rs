//给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。
//
// 示例：
//
// 给定一个链表: 1->2->3->4->5, 和 n = 2.
//
//当删除了倒数第二个节点后，链表变为 1->2->3->5.
//
//
// 说明：
//
// 给定的 n 保证是有效的。
//
// 进阶：
//
// 你能尝试使用一趟扫描实现吗？
// Related Topics 链表 双指针
// 👍 904 👎 0

use std::option::Option::Some;

use crate::leetcode::editor::cn::ListNode::ListNode;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_remove_nth_node_from_end_of_list() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn remove_nth_from_end1(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut temp = &mut head;
        while let Some(node) = temp {
            len += 1;
            temp = &mut node.next;
        }
        let mut ptr = &mut head;
        for _i in 0..len - n {
            if let Some(node) = ptr {
                ptr = &mut node.next;
            }
        }
        *ptr = ptr.as_mut().unwrap().next.take();
        head
    }

    // 双指针
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut temp_head = head.clone();
        let mut fast = &mut temp_head;
        let mut slow = &mut head;
        for _i in 0..n {
            fast = &mut fast.as_mut().unwrap().next;
        }
        while let Some(node) = fast {
            fast = &mut node.next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        // slow是一个可变引用,*slow是可变引用指向的地方(Option<Box<ListNode>>), 替换成next
        *slow = slow.as_mut().unwrap().next.take();
        return head;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
