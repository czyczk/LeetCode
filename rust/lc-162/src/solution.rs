pub struct Solution {}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }

        let (mut start, mut end) = (0, n - 1);
        loop {
            let mid = (start + end) / 2;
            match mid {
                0 => {
                    if nums[mid] > nums[mid + 1] {
                        return mid as i32;
                    }

                    start = mid + 1;
                }
                x if x == n - 1 => {
                    if nums[mid] > nums[mid - 1] {
                        return mid as i32;
                    }

                    start = mid - 1;
                }
                _ => {
                    let cur = nums[mid];
                    let prev = nums[mid - 1];
                    let next = nums[mid + 1];

                    if prev < cur && cur > next {
                        return mid as i32;
                    }

                    if cur < next {
                        start = mid + 1;
                    } else {
                        end = mid - 1;
                    }
                }
            }
        }
    }
}
