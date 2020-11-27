pub struct Solution {}

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let mut max_gap = 0;
        radix_sort(&mut nums);

        for i in 0..nums.len() - 1 {
            max_gap = (nums[i + 1] - nums[i]).max(max_gap);
        }

        return max_gap;
    }
}

fn radix_sort(nums: &mut Vec<i32>) {
    let n = nums.len();
    let max_str_len = nums.iter().max().unwrap().to_string().len();

    let mut buf = vec![0; n];
    let mut exp = 1;
    for _ in 0..max_str_len {
        let mut buckets = [0; 10];

        for &num in nums.iter() {
            buckets[((num / exp) % 10) as usize] += 1;
        }

        for j in 0..buckets.len() - 1 {
            buckets[j + 1] += buckets[j];
        }

        for j in (0..=n - 1).rev() {
            let num = nums[j];
            let index = ((num / exp) % 10) as usize;
            buf[buckets[index] - 1] = num;
            buckets[index] -= 1;
        }

        nums.clone_from_slice(&buf);
        exp *= 10;
    }
}
