pub struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        match n {
            0 => return 0,
            1 => return stones[0],
            _ => {}
        }

        let mut heap = std::collections::BinaryHeap::new();
        for stone in stones {
            heap.push(stone);
        }

        while heap.len() > 1 {
            let stone1 = heap.pop().unwrap();
            let stone2 = heap.pop().unwrap();
            if stone1 != stone2 {
                heap.push((stone1 - stone2).abs());
            }
        }

        return match heap.len() {
            1 => heap.pop().unwrap(),
            _ => 0,
        }
    }
}
