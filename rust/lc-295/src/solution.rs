pub struct MedianFinder {
    max_heap: std::collections::BinaryHeap<i32>,
    min_heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            max_heap: std::collections::BinaryHeap::new(),
            min_heap: std::collections::BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        if self.min_heap.is_empty() || num >= self.min_heap.peek().unwrap().0 {
            self.min_heap.push(std::cmp::Reverse(num));
            if self.min_heap.len() > self.max_heap.len() + 1 {
                self.max_heap.push(self.min_heap.pop().unwrap().0);
            }
        } else {
            self.max_heap.push(num);
            if self.max_heap.len() > self.min_heap.len() {
                self.min_heap
                    .push(std::cmp::Reverse(self.max_heap.pop().unwrap()));
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.min_heap.is_empty() {
            return 0.0;
        } else if self.min_heap.len() > self.max_heap.len() {
            return self.min_heap.peek().unwrap().0 as f64;
        } else {
            return (self.min_heap.peek().unwrap().0 as f64
                + *self.max_heap.peek().unwrap() as f64)
                / 2.0;
        }
    }
}
