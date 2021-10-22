pub struct Solution {}

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let sqrt = (area as f64).sqrt().floor() as i32;
        for i in (1..=sqrt).rev() {
            if area % i == 0 {
                return vec![area / i, i];
            }
        }

        panic!();
    }
}
