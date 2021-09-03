pub struct Solution {}

impl Solution {
    pub fn smallest_k(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![];
        }

        let k = k as usize;
        let (mut l, mut r) = (0, arr.len() - 1);

        while l < r {
            let p = Solution::partition(&mut arr, l, r);
            match p.cmp(&k) {
                std::cmp::Ordering::Equal => break,
                std::cmp::Ordering::Less => {
                    l = p + 1;
                }
                std::cmp::Ordering::Greater => {
                    r = p - 1;
                }
            }
        }

        return arr[..k].to_owned();
    }

    fn partition(arr: &mut Vec<i32>, start: usize, end: usize) -> usize {
        let (mut l, mut r) = (start, end);
        let pivot = arr[start];

        while l < r {
            while l < r && arr[r] > pivot {
                r -= 1;
            }

            while l < r && arr[l] <= pivot {
                l += 1;
            }

            if l < r {
                let temp = arr[l];
                arr[l] = arr[r];
                arr[r] = temp;
            }
        }

        arr[start] = arr[l];
        arr[l] = pivot;

        return l;
    }
}
