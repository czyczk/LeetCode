pub struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1 = version1.as_bytes();
        let version2 = version2.as_bytes();
        let (mut i1, mut i2) = (0, 0);
        let (n1, n2) = (version1.len(), version2.len());

        loop {
            if i1 == n1 && i2 == n2 {
                return 0;
            }

            let (mut num1, mut num2) = (0, 0);
            let (mut is_leading_zero1, mut is_leading_zero2) = (true, true);

            while i1 < n1 {
                let ch1 = version1[i1];
                if is_leading_zero1 {
                    if ch1 == '0' as u8 {
                        i1 += 1;
                        continue;
                    } else {
                        is_leading_zero1 = false;
                    }
                }

                if ch1 == '.' as u8 {
                    i1 += 1;
                    break;
                }

                num1 *= 10;
                num1 += (ch1 - '0' as u8) as u32;
                i1 += 1;
            }

            while i2 < n2 {
                let ch2 = version2[i2];
                if is_leading_zero2 {
                    if ch2 == '0' as u8 {
                        i2 += 1;
                        continue;
                    } else {
                        is_leading_zero2 = false;
                    }
                }

                if ch2 == '.' as u8 {
                    i2 += 1;
                    break;
                }

                num2 *= 10;
                num2 += (ch2 - '0' as u8) as u32;
                i2 += 1;
            }

            match num1.cmp(&num2) {
                std::cmp::Ordering::Less => return -1,
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Greater => return 1,
            }
        }
    }
}
