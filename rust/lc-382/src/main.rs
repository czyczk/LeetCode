mod list_node;
mod solution;

use crate::list_node::ListNode;
use crate::solution::Solution;

fn main() {
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(4)));

    let obj = Solution::new(head);
    let ret_1: i32 = obj.get_random();

    println!("{}", ret_1);
}
