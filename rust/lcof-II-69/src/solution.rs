pub struct Solution {}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        match n {
            1 => return 0,
            2 => return if arr[0] > arr[1] { 0 } else { 1 },
            _ => {}
        }

        let (mut start, mut end) = (0, n - 1);
        while start <= end {
            let mid = (start + end) / 2;
            let num_mid = arr[mid];
            if mid == 0 {
                if num_mid > arr[1] {
                    return 0;
                } else {
                    start = mid + 1;
                }
            } else if mid == n - 1 {
                if num_mid > arr[n - 2] {
                    return (n - 1) as i32;
                } else {
                    end = mid - 1;
                }
            } else {
                let num_left = arr[mid - 1];
                let num_right = arr[mid + 1];
                if num_mid > num_left && num_mid > num_right {
                    return mid as i32;
                } else if num_mid > num_left && num_mid < num_right {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
        }

        -1
    }
}
