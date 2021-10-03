pub struct Solution {}

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_owned();
        }

        let is_positive = numerator >= 0 && denominator >= 0 || numerator < 0 && denominator < 0;
        let numerator = (numerator as i64).abs();
        let denominator = (denominator as i64).abs();

        let mut rem = numerator % denominator;
        let integer_part = numerator / denominator;
        let mut ret = if is_positive {
            integer_part.to_string()
        } else {
            format!("-{}", integer_part.to_string())
        };

        if rem == 0 {
            return ret;
        }

        ret.push('.');

        // Decimal part
        let mut rem_idx_map = std::collections::HashMap::new();

        while rem != 0 {
            rem_idx_map.insert(rem, ret.len());
            rem *= 10;
            ret.push_str(&(rem / denominator).to_string());
            rem %= denominator;

            if let Some(&idx) = rem_idx_map.get(&rem) {
                let mut new_str = String::with_capacity(ret.len() + 2);
                new_str.push_str(&ret[..idx]);
                new_str.push('(');
                new_str.push_str(&ret[idx..]);
                new_str.push(')');
                return new_str;
            }
        }

        ret
    }
}
