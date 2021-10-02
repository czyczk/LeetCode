pub struct Solution {}

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_owned();
        }

        let hex_map = [
            b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd',
            b'e', b'f',
        ];

        let mut is_leading_zero = num >= 0;
        let mut result = Vec::with_capacity(8);
        for i in 0..8 {
            let hex_digit = num >> ((7 - i) * 4) & 0xf;
            if is_leading_zero {
                if hex_digit == 0 {
                    continue;
                } else {
                    is_leading_zero = false;
                }
            }

            result.push(hex_map[hex_digit as usize]);
        }

        String::from_utf8(result).unwrap()
    }
}
