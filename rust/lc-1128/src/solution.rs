pub struct Solution {}

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut nums = [0; 100];

        for d in dominoes {
            let a = d[0];
            let b = d[1];
            match a.cmp(&b) {
                std::cmp::Ordering::Less => nums[a as usize * 10 + b as usize] += 1,
                _ => nums[b as usize * 10 + a as usize] += 1,
            }
        }

        nums.iter().fold(0, |acc, &num| acc + num * (num - 1) / 2)
    }
}
