//将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
//
//
//
// 示例：
//
// 输入：1->2->4, 1->3->4
//输出：1->1->2->3->4->4
//
// Related Topics 链表
// 👍 1176 👎 0

use crate::leetcode::editor::cn::ListNode::ListNode;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_merge_two_sorted_lists() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut ptr3 = &mut head;
        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val < n2.val {
                //将较小链表连接到新链表尾节点，所有权移动
                ptr3.next = l1;
                //将l3尾节点指向它的后继节点
                ptr3 = ptr3.next.as_mut().unwrap();
                //将链表从尾节点取下来，将所有权返给较小的链表
                l1 = ptr3.next.take();
            } else {
                //同上面的逻辑
                ptr3.next = l2;
                ptr3 = ptr3.next.as_mut().unwrap();
                l2 = ptr3.next.take();
            }
        }
        //剩余部分
        ptr3.next = if l1.is_some() { l1 } else { l2 };
        return head.next;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
