//请判断一个链表是否为回文链表。
//
// 示例 1:
//
// 输入: 1->2
//输出: false
//
// 示例 2:
//
// 输入: 1->2->2->1
//输出: true
//
//
// 进阶：
//你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？
// Related Topics 链表 双指针
// 👍 578 👎 0

use std::option::Option::Some;

use crate::leetcode::editor::cn::ListNode::ListNode;

#[cfg(test)]
pub mod tests {
    use crate::leetcode::editor::cn::ListNode::ListNode;

    use super::*;

    #[test]
    fn test_palindrome_linked_list() {
        let mut head = Box::new(ListNode::new(1));
        head.next = Some(Box::new(ListNode::new(2)));
        head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(1)));
        println!("{}", Solution::is_palindrome(Some(head)));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let len = Solution::get_size(&mut head);
        let half = len / 2;
        let mut head_1 = None;
        // 反转前半部分的链表
        for _ in 0..half {
            let mut temp = head.unwrap();
            head = temp.next;
            temp.next = head_1;
            head_1 = Some(temp);
        }
        let mut head_2 = None;
        if len & 1 == 0 {
            head_2 = head.take();
        } else {
            head_2 = head.unwrap().next.take();
        }
        while let (Some(node1), Some(node2)) = (head_1.as_ref(), head_2.as_ref()) {
            if node1.val != node2.val {
                return false;
            }
            head_1 = head_1.unwrap().next;
            head_2 = head_2.unwrap().next;
        }
        return true;
    }

    // 不要搞混mut head(head前面加mut 仅仅代表head可以重赋值), 重赋的值应该跟之前同类型
    fn get_size(mut head: &Option<Box<ListNode>>) -> usize {
        let mut res = 0;
        while let Some(node) = head {
            res += 1;
            head = &node.next;
        }
        return res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
