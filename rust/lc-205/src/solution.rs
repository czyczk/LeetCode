pub struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let n = s.len();
        if n < 2 {
            return true;
        }

        let mut map = [0 as u8; 256];
        let mut mapped = [false; 256];

        let t_bytes = t.as_bytes();

        for (i, src) in s.bytes().enumerate() {
            let src_idx = src as usize;
            let target = t_bytes[i];
            let target_idx = target as usize;

            let mapped_target = map[src_idx];

            if mapped_target == 0 {
                if !mapped[target_idx] {
                    // Create a new mapping if no mapping is found for the ch and the target is not mapped.
                    map[src_idx] = target;
                    mapped[target_idx] = true;
                } else {
                    // If no mapping is found for the src and the target is used, it violates the rule that no two characters can map to the same one.
                    return false;
                }
            } else if mapped_target != target {
                // The existing mapping already indicates that they're not isomorphic.
                return false;
            }
        }

        true
    }
}
