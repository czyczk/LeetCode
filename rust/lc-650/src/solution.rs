pub struct Solution {}

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut dp = std::collections::HashMap::new();
        dp.insert(1, 0);
        dp.insert(2, 2);
        get_steps(n, dp).0
    }
}

fn get_steps(
    n: i32,
    mut dp: std::collections::HashMap<i32, i32>,
) -> (i32, std::collections::HashMap<i32, i32>) {
    match dp.get(&n) {
        Some(&ret) => return (ret, dp),
        None => {
            if n % 2 == 0 {
                let (ret_half, new_dp) = get_steps(n / 2, dp);
                return (ret_half + 2, new_dp);
            } else {
                let mut min_times = 3;
                let mut max_div = 0;
                for i in 3..=(n as f64).sqrt().floor() as i32 {
                    if n % i == 0 {
                        min_times = i;
                        max_div = n / i;
                        break;
                    }
                }

                if max_div == 0 {
                    dp.insert(n, n);
                    return (n, dp);
                }

                let (ret_max_div, new_dp) = get_steps(max_div, dp);
                dp = new_dp;
                let ret = ret_max_div + min_times;
                dp.insert(n, ret);
                return (ret, dp);
            }
        }
    }
}
