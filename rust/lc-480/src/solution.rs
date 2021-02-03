pub struct Solution {}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let n = nums.len();
        let mut dh = DualHeap::with_capacity(k);
        let mut ret = Vec::with_capacity(n - k + 1);

        for (i, &num) in nums.iter().enumerate() {
            dh.push(num);

            if i < k - 1 {
                continue;
            }

            ret.push(dh.get_median());
            dh.remove(nums[i + 1 - k]);
        }

        ret
    }
}

pub struct DualHeap {
    capacity: usize,
    min_heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
    max_heap: std::collections::BinaryHeap<i32>,
    delayed: std::collections::HashMap<i32, usize>,
    min_heap_len: usize,
    max_heap_len: usize,
}

impl DualHeap {
    pub fn with_capacity(capacity: usize) -> DualHeap {
        DualHeap {
            capacity,
            min_heap: std::collections::BinaryHeap::new(),
            max_heap: std::collections::BinaryHeap::new(),
            delayed: std::collections::HashMap::new(),
            min_heap_len: 0,
            max_heap_len: 0,
        }
    }

    pub fn get_median(&self) -> f64 {
        return if self.capacity & 1 == 1 {
            *self.max_heap.peek().unwrap() as f64
        } else {
            (*self.max_heap.peek().unwrap() as f64 + self.min_heap.peek().unwrap().0 as f64) / 2.0
        };
    }

    pub fn push(&mut self, num: i32) {
        match self.max_heap.peek() {
            None => {
                self.push_to_max_heap(num);
            }
            Some(&largest) if num <= largest => {
                self.push_to_max_heap(num);
            }
            _ => {
                self.push_to_min_heap(num);
            }
        }

        self.make_balance();
    }

    pub fn remove(&mut self, num: i32) {
        *self.delayed.entry(num).or_insert(0) += 1;
        let &largest_in_max_heap = self.max_heap.peek().unwrap();

        if num <= largest_in_max_heap {
            self.max_heap_len -= 1;
            if num == largest_in_max_heap {
                self.prune_max_heap();
            }
        } else {
            self.min_heap_len -= 1;
            if num == self.min_heap.peek().unwrap().0 {
                self.prune_min_heap();
            }
        }

        self.make_balance();
    }

    fn push_to_max_heap(&mut self, num: i32) {
        self.max_heap.push(num);
        self.max_heap_len += 1;
    }

    fn push_to_min_heap(&mut self, num: i32) {
        self.min_heap.push(std::cmp::Reverse(num));
        self.min_heap_len += 1;
    }

    fn make_balance(&mut self) {
        if self.max_heap_len > self.min_heap_len + 1 {
            self.min_heap
                .push(std::cmp::Reverse(self.max_heap.pop().unwrap()));
            self.min_heap_len += 1;
            self.max_heap_len -= 1;
            self.prune_max_heap();
        } else if self.max_heap_len < self.min_heap_len {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
            self.max_heap_len += 1;
            self.min_heap_len -= 1;
            self.prune_min_heap();
        }
    }

    fn prune_max_heap(&mut self) {
        while !self.max_heap.is_empty() {
            let &num = self.max_heap.peek().unwrap();
            let remaining = self.delayed.entry(num).or_default();
            if *remaining > 0 {
                self.max_heap.pop();
                *remaining -= 1;
                if *remaining == 0 {
                    self.delayed.remove(&num);
                }
            } else {
                break;
            }
        }
    }

    fn prune_min_heap(&mut self) {
        while !self.min_heap.is_empty() {
            let num = self.min_heap.peek().unwrap().0;
            let remaining = self.delayed.entry(num).or_default();
            if *remaining > 0 {
                self.min_heap.pop();
                *remaining -= 1;
                if *remaining == 0 {
                    self.delayed.remove(&num);
                }
            } else {
                break;
            }
        }
    }
}
