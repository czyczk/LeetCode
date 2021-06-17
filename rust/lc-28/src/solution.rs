pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();

        let m = haystack.len();
        let n = needle.len();

        if n == 0 {
            return 0;
        }

        if m == 0 {
            return -1;
        }

        let next = Solution::build_next(needle);

        let mut j = 0usize;
        for i in 0..m {
            while j > 0 && haystack[i] != needle[j] {
                j = next[j - 1];
            }

            if haystack[i] == needle[j] {
                j += 1;
                if j == n {
                    return (i + 1 - n) as i32;
                }
            }
        }

        return -1;
    }

    fn build_next(needle: &[u8]) -> Vec<usize> {
        let n = needle.len();
        let mut next = vec![0; n];

        let mut j = 0usize;
        for i in 1..n {
            while j > 0 && needle[i] != needle[j] {
                j = next[j - 1];
            }

            if needle[i] == needle[j] {
                j += 1;
            }

            next[i] = j;
        }

        return next;
    }
}
