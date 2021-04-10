use std::cell::RefCell;
use std::rc::Rc;

use learn_rust::leetcode::editor::cn::TreeNode::TreeNode;

fn main() {
    let mut node = TreeNode::new(1);
    let node_left = TreeNode::new(2);
    node.left = Some(Rc::new(RefCell::new(node_left)));
    let root = Some(Rc::new(RefCell::new(node)));
    let node_clone = root.clone();
    println!("root = {:#?}", root);
    println!("root_p = {:p}", root.unwrap());
    println!("root_clone= {:#?}", node_clone);
    println!("root_clone_p = {:p}", node_clone.unwrap());

    let a = Box::new(2);
    let a_clone = a.clone();
    println!("a = {:p}", a);
    println!("a_clone = {:p}", a_clone);

    let a_refcell = RefCell::new(2);
    let a_refcell_clone = a_refcell.clone();
    println!("a_refcell = {:p}", a_refcell.as_ptr());
    println!("a_refcell_clone = {:p}", a_refcell_clone.as_ptr());
}
