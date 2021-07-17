pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        if coins.len() == 0 {
            return -1;
        }

        let f = vec![0; amount as usize];

        return Solution::coin_change_rec(&coins, amount, f).0;
    }

    fn coin_change_rec(coins: &Vec<i32>, rem: i32, mut f: Vec<i32>) -> (i32, Vec<i32>) {
        if rem == 0 {
            return (0, f);
        }

        if let Some(&num) = f.get(rem as usize - 1) {
            if num != 0 {
                return (num, f);
            }
        }

        let mut min_num = std::i32::MAX;
        for &c in coins.iter() {
            if c > rem {
                continue;
            }

            let sub_rem = rem - c;
            let (sub_min_num, sub_f) = Solution::coin_change_rec(coins, sub_rem, f);
            f = sub_f;
            if sub_min_num != -1 && sub_min_num < min_num {
                min_num = sub_min_num;
            }
        }

        if min_num == std::i32::MAX {
            f[rem as usize - 1] = -1;
            return (-1, f);
        }

        f[rem as usize - 1] = min_num + 1;
        return (min_num + 1, f);
    }
}
