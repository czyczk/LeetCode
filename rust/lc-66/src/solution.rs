pub struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = false;
        {
            let last_digit = digits.last_mut().unwrap();
            let new_digit = *last_digit + 1;
            if new_digit >= 10 {
                carry = true;
            }
            *last_digit = new_digit % 10;
        }

        for digit in digits.iter_mut().rev().skip(1) {
            if !carry {
                break;
            }

            let new_digit = *digit + 1;
            *digit = new_digit % 10;
            carry = new_digit >= 10;
        }

        if carry {
            digits.insert(0, 1);
        }

        digits
    }
}
