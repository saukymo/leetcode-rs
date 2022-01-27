#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut numbers: BinaryHeap<i32> = BinaryHeap::new();

        for mut node in lists.iter() {
            while *node != None {
                let value = (*node).as_ref().unwrap().val;
                numbers.push(value);    
                node = &(*node).as_ref().unwrap().next;
            }
        }

        let mut h = ListNode::new(0);

        while let Some(value) = numbers.pop() {
            let mut p = Box::new(ListNode::new(value));
            p.next = h.next;
            h.next = Some(p);
        }

        return h.next;
    }
}