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
      right: None
    }
  }
}

struct Solution {

}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        let root = root.unwrap();
        let root_ref = root.borrow_mut();

        return Self::is_subtree_symmetric(root_ref.left.clone(), root_ref.right.clone());
    }

    fn is_subtree_symmetric(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if left == None && right == None {
            return true;
        }
        
        if left == None || right == None {
            return false;
        }
        
        let left = left.unwrap();
        let right = right.unwrap();

        let left_ref = left.borrow_mut();
        let right_ref = right.borrow_mut();
    
    
        if (left_ref.right == None && right_ref.left != None) || (left_ref.right != None && right_ref.left == None) {
            return false;
        }
    
        if (left_ref.left == None && right_ref.right != None) || (left_ref.left != None && right_ref.right == None) {
            return false;
        }
    
        return (left_ref.val == right_ref.val) && (Self::is_subtree_symmetric(left_ref.left.clone(), right_ref.right.clone())) && (Self::is_subtree_symmetric(left_ref.right.clone(), right_ref.left.clone()));
    }
    
}