use std::collections::LinkedList;

pub struct Solution {}

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let len = senate.len();
        let mut r_queue = LinkedList::new();
        let mut d_queue = LinkedList::new();

        let mut i = 0;
        for ch in senate.chars() {
            match ch {
                'R' => {
                    r_queue.push_back(i);
                }
                _ => {
                    d_queue.push_back(i);
                }
            }

            i += 1;
        }

        loop {
            if r_queue.is_empty() {
                return String::from("Dire");
            }

            if d_queue.is_empty() {
                return String::from("Radiant");
            }

            let r_front = r_queue.pop_front().unwrap();
            let d_front = d_queue.pop_front().unwrap();
            if r_front < d_front {
                r_queue.push_back(r_front + len);
            } else {
                d_queue.push_back(d_front + len);
            }
        }
    }
}
