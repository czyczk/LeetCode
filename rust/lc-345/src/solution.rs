pub struct Solution {}

impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        let s_bytes;
        unsafe {
            s_bytes = s.as_bytes_mut();
        }

        let mut vowels = std::collections::HashSet::new();
        vowels.insert('a' as u8);
        vowels.insert('e' as u8);
        vowels.insert('i' as u8);
        vowels.insert('o' as u8);
        vowels.insert('u' as u8);
        vowels.insert('A' as u8);
        vowels.insert('E' as u8);
        vowels.insert('I' as u8);
        vowels.insert('O' as u8);
        vowels.insert('U' as u8);

        let (mut left, mut right) = (0, s_bytes.len() - 1);
        while left < right {
            while left < right && !vowels.contains(&s_bytes[right]) {
                right -= 1;
            }

            while left < right && !vowels.contains(&s_bytes[left]) {
                left += 1;
            }

            if left < right {
                let temp = s_bytes[left];
                s_bytes[left] = s_bytes[right];
                s_bytes[right] = temp;
            }

            let (new_right, is_overflow) = right.overflowing_sub(1);
            if is_overflow {
                break;
            } else {
                right = new_right;
            }

            left += 1;
        }

        return String::from_utf8(s_bytes.to_owned()).unwrap();
    }
}
