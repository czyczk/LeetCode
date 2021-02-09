pub struct Solution {}

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n < 2 {
            return n as i32;
        }

        let mut last_flow = arr[1].cmp(&arr[0]);
        let mut cur_len = match last_flow {
            std::cmp::Ordering::Equal => 1,
            _ => 2,
        };
        let mut max_len = cur_len;

        for i in 2..n {
            match arr[i].cmp(&arr[i - 1]) {
                std::cmp::Ordering::Equal => {
                    last_flow = std::cmp::Ordering::Equal;
                    max_len = max_len.max(cur_len);
                    cur_len = 1;
                }
                flow if flow == last_flow => {
                    max_len = max_len.max(cur_len);
                    cur_len = 2;
                }
                opposite => {
                    cur_len += 1;
                    last_flow = opposite;
                }
            }
        }

        max_len.max(cur_len)
    }
}
