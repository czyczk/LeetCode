pub struct Solution {}

impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut trend = std::cmp::Ordering::Equal;
        let n = a.len();
        if n == 1 {
            return true;
        }

        for (i, cur) in a[1..].iter().enumerate() {
            let diff = a[i].cmp(cur);
            if diff == trend {
                if trend == std::cmp::Ordering::Equal {
                    continue;
                } else {
                    return false;
                }
            } else if trend == std::cmp::Ordering::Equal {
                trend = diff.reverse();
            }
        }

        true
    }
}
