// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            // Both trees are empty - they are the same
            (None, None) => true,

            // One tree is empty, the other is not - they are different
            (None, Some(_)) | (Some(_), None) => false,

            // Both trees have nodes - check if values match and recursively check subtrees
            (Some(p_node), Some(q_node)) => {
                let p_ref = p_node.borrow();
                let q_ref = q_node.borrow();

                // Check current values match
                p_ref.val == q_ref.val
                    // Recursively check left subtrees
                    && Solution::is_same_tree(p_ref.left.clone(), q_ref.left.clone())
                    // Recursively check right subtrees
                    && Solution::is_same_tree(p_ref.right.clone(), q_ref.right.clone())
            }
        }
    }
}

fn main() {
    // Example 1: p = [1,2,3], q = [1,2,3]
    let p1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let q1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    println!("Example 1: {}", Solution::is_same_tree(p1, q1)); // true

    // Example 2: p = [1,2], q = [1,null,2]
    let p2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    })));
    let q2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    println!("Example 2: {}", Solution::is_same_tree(p2, q2)); // false

    // Example 3: p = [1,2,1], q = [1,1,2]
    let p3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    })));
    let q3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    println!("Example 3: {}", Solution::is_same_tree(p3, q3)); // false
}
