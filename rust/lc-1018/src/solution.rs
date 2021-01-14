pub struct Solution {
}

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let n = a.len();
        let mut ret = Vec::with_capacity(n);
        if n == 0 {
            return ret;
        }

        let mut cur = 0;
        for num in a {
            cur <<= 1;
            cur += num;
            cur %= 5;
            match cur {
                0 => ret.push(true),
                _ => ret.push(false),
            }
        }

        ret
    }
}