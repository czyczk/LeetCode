pub struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = a[0].len();
        for line in a.iter_mut() {
            for j in 0..(m + 1) / 2 {
                let k = m - j - 1;
                if j == k {
                    line[j] = 1 - line[j];
                } else {
                    let temp = 1 - line[j];
                    line[j] = 1 - line[k];
                    line[k] = temp;
                }
            }
        }

        a
    }
}
