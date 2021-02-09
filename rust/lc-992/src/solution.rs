pub struct Solution {}

impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        (calc_num_subarrays_with_k_distinct(&a, k) - calc_num_subarrays_with_k_distinct(&a, k - 1)) as i32
    }

}

fn calc_num_subarrays_with_k_distinct(arr: &Vec<i32>, k: i32) -> usize {
    let n = arr.len();
    let mut freq = vec![0; n + 1];
    
    let (mut left, mut right) = (0 as usize, 0 as usize);
    let mut count = 0;
    let mut res = 0;

    while right < n {
        if freq[arr[right] as usize] == 0 {
            count += 1;
        }
        freq[arr[right] as usize] += 1;

        right += 1;

        while count > k {
            freq[arr[left] as usize] -= 1;
            if freq[arr[left] as usize] == 0 {
                count -= 1;
            }

            left += 1;
        }

        res += right - left;
    }

    res
}
