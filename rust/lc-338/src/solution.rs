pub struct Solution {}

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut ret = Vec::with_capacity(num as usize + 1);
        ret.push(0);
        let mut high_bit = 0;

        for i in 1..=num {
            if i & (i - 1) == 0 {
                high_bit = i;
            }

            ret.push(ret[(i - high_bit) as usize] + 1);
        }

        ret
    }
}

pub struct SolutionHacker {}

impl SolutionHacker {
    pub fn count_bits(num: i32) -> Vec<i32> {
        (0..=num).map(|mut i| {
            i = (i & 0x55555555) + ((i >> 1) & 0x55555555);
            i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
            i = (i & 0x0f0f0f0f) + ((i >> 4) & 0x0f0f0f0f);
            i = (i & 0x00ff00ff) + ((i >> 8) & 0x00ff00ff);
            i = (i & 0x0000ffff) + ((i >> 16) & 0x0000ffff);
            i
        }).collect()
    }
}
