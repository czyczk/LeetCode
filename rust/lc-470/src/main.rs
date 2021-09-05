mod solution;

use solution::Solution;

fn main() {
    let mut buckets = [false; 10];

    for _ in 0..100000 {
        let r = Solution::rand10();
        assert_eq!(true, r >= 1 && r <= 10);
        buckets[r as usize - 1] = true;
    }

    for i in 0..10 {
        assert_eq!(true, buckets[i]);
    }
}
