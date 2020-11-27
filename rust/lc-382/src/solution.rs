use crate::list_node::ListNode;
use rand::Rng;

pub struct Solution {
    head: Option<Box<ListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    /** @param head The linked list's head.
    Note that the head is guaranteed to be not null, so it contains at least one node. */
    pub fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    /** Returns a random node's value. */
    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();

        let mut cursor = &self.head;
        let mut count = 1;
        let mut result = 0;

        while let Some(node) = cursor {
            if rng.gen_range(0, count) == 0 {
                result = node.val;
            }

            cursor = &cursor.as_ref().unwrap().next;
            count += 1;
        }

        return result;
    }
}
