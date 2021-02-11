pub struct KthLargest {
    heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kl = KthLargest {
            heap: std::collections::BinaryHeap::with_capacity(k as usize),
            capacity: k as usize,
        };

        for num in nums {
            kl.add(num);
        }

        kl
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.capacity {
            self.heap.push(std::cmp::Reverse(val));
        } else if self.heap.peek().unwrap().0 < val {
            self.heap.pop().unwrap();
            self.heap.push(std::cmp::Reverse(val));
        }

        return self.heap.peek().unwrap().0;
    }
}
