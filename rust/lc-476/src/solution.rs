pub struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask1 = 0x40000000;
        let mut mask2 = 0x7fffffff;
        while num & mask1 == 0 {
            mask1 >>= 1;
            mask2 >>= 1;
        }

        num ^ mask2
    }
}
