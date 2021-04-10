//序列化是将一个数据结构或者对象转换为连续的比特位的操作，进而可以将转换后的数据存储在一个文件或者内存中，同时也可以通过网络传输到另一个计算机环境，采取相反方
//式重构得到原数据。
//
// 请设计一个算法来实现二叉树的序列化与反序列化。这里不限定你的序列 / 反序列化算法执行逻辑，你只需要保证一个二叉树可以被序列化为一个字符串并且将这个字符串
//反序列化为原始的树结构。
//
// 提示: 输入输出格式与 LeetCode 目前使用的方式一致，详情请参阅 LeetCode 序列化二叉树的格式。你并非必须采取这种方式，你也可以采用其他的
//方法解决这个问题。
//
//
//
// 示例 1：
//
//
//输入：root = [1,2,3,null,null,4,5]
//输出：[1,2,3,null,null,4,5]
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
//输出：[1,2]
//
//
//
//
// 提示：
//
//
// 树中结点数在范围 [0, 104] 内
// -1000 <= Node.val <= 1000
//
// Related Topics 树 设计
// 👍 461 👎 0

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
    fn test_serialize_and_deserialize_binary_tree() {
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        node1.borrow_mut().left = Some(node2.clone());
        node1.borrow_mut().right = Some(node3.clone());
        node3.borrow_mut().left = Some(node4.clone());
        node3.borrow_mut().right = Some(node5.clone());
        let codec = Codec::new();
        let str_node = codec.serialize(Some(node1));
        println!("str_node = {:?}", str_node);
        let node = codec.deserialize(str_node);
        println!("root = {:?}", node);
    }
}

struct Solution {}

struct Codec2 {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec2 {
    fn new() -> Self {
        Codec2 {}
    }
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = "".to_string();
        if root.is_none() {
            return res;
        }
        let mut que = VecDeque::new();
        res.push_str(root.clone().unwrap().borrow().val.to_string().as_str());
        res.push_str(",");
        que.push_back(root.clone().unwrap());
        while let Some(node_curr) = que.pop_front() {
            if node_curr.borrow().left.is_some() {
                res.push_str(
                    node_curr
                        .borrow()
                        .left
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .val
                        .to_string()
                        .as_str(),
                );
                res.push_str(",");
                que.push_back(node_curr.borrow().left.clone().unwrap());
            } else {
                res.push_str("n");
                res.push_str(",");
            }
            if node_curr.borrow().right.is_some() {
                res.push_str(
                    node_curr
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .val
                        .to_string()
                        .as_str(),
                );
                res.push_str(",");
                que.push_back(node_curr.borrow().right.clone().unwrap());
            } else {
                res.push_str("n");
                res.push_str(",");
            }
        }
        String::from(&res[0..res.len() - 1])
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let vec_str = data.split(",").collect::<Vec<&str>>();
        let root = Rc::new(RefCell::new(TreeNode::new(vec_str[0].parse().unwrap())));
        let mut que: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        que.push_back(root.clone());
        let mut idx = 1;
        while idx < vec_str.len() {
            let node_curr = que.pop_front().unwrap();
            if "n" != vec_str[idx] {
                let node_left = Rc::new(RefCell::new(TreeNode::new(
                    vec_str[idx].parse::<i32>().unwrap(),
                )));
                que.push_back(node_left.clone());
                node_curr.borrow_mut().left = Some(node_left.clone());
            }
            idx += 1;
            if "n" != vec_str[idx] {
                let node_right = Rc::new(RefCell::new(TreeNode::new(
                    vec_str[idx].parse::<i32>().unwrap(),
                )));
                que.push_back(node_right.clone());
                node_curr.borrow_mut().right = Some(node_right.clone());
            }
            idx += 1;
        }
        Some(root)
    }
}

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    // T -> (T) num (T) | X
    // 巴科斯范式（BNF）
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(root) = root {
            let l = ["(", &self.serialize(root.borrow().left.clone()), ")"].concat();
            let r = ["(", &self.serialize(root.borrow().right.clone()), ")"].concat();
            return l + &root.borrow().val.to_string() + &r;
        } else {
            "X".to_string()
        }
    }

    fn parse_sub_tree(&self, chars: &[u8], ptr: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        // 跳过左括号
        *ptr += 1;
        let sub_tree = self.parse(chars, ptr);
        // 跳过右括号
        *ptr += 1;
        sub_tree
    }

    fn parse_int(&self, chars: &[u8], ptr: &mut usize) -> i32 {
        let mut x = 0;
        let mut sgn = 1;

        if !chars[*ptr].is_ascii_digit() {
            sgn = -1;
            *ptr += 1;
        }

        while chars[*ptr].is_ascii_digit() {
            x = x * 10 + (chars[*ptr] - b'0') as i32;
            *ptr += 1;
        }

        return x * sgn;
    }

    fn parse(&self, chars: &[u8], ptr: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if chars[*ptr] == b'X' {
            *ptr += 1;
            return None;
        }
        let mut cur = TreeNode::new(0);
        cur.left = self.parse_sub_tree(chars, ptr);
        cur.val = self.parse_int(chars, ptr);
        cur.right = self.parse_sub_tree(chars, ptr);
        Some(Rc::new(RefCell::new(cur)))
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        println!("res = {:?}", &data);
        let mut ptr: usize = 0;
        let chars = data.as_bytes();
        self.parse(&chars, &mut ptr)
    }
}

struct Codec3 {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec3 {
    fn new() -> Self {
        Codec3 {}
    }
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = "".to_string();
        Self::serialize_tree(root, &mut res);
        // println!("res = {:?}", &res);
        res
    }

    fn serialize_tree(root: Option<Rc<RefCell<TreeNode>>>, res: &mut String) {
        match root {
            None => {
                res.push_str("N,");
            }
            Some(node) => {
                res.push_str(&node.borrow().val.to_string());
                res.push_str(",");
                Self::serialize_tree(node.borrow().left.clone(), res);
                Self::serialize_tree(node.borrow().right.clone(), res);
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tree = data.split(",").collect();
        return Self::deserialize_tree(&mut tree);
    }

    fn deserialize_tree(tree: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        if tree.is_empty() {
            return None;
        }
        if tree[0] == "N" {
            tree.remove(0);
            return None;
        }
        let node = Rc::new(RefCell::new(TreeNode::new(tree[0].parse().unwrap())));
        tree.remove(0);
        node.borrow_mut().left = Self::deserialize_tree(tree);
        node.borrow_mut().right = Self::deserialize_tree(tree);
        Some(node)
    }
}

//leetcode submit region end(Prohibit modification and deletion)
