use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut cnt_map = HashMap::new();
        let mut num_sub_seq_map = HashMap::new();

        for &num in nums.iter() {
            *cnt_map.entry(num).or_insert(0) += 1;
        }

        for num in nums.iter() {
            if cnt_map[num] == 0 {
                continue;
            }

            let cnt_num = cnt_map.get_mut(num).unwrap();
            if *cnt_num == 0 {
                continue;
            } else {
                *cnt_num -= 1;
            }

            match num_sub_seq_map.get_mut(&(*num - 1)) {
                Some(prev_num_sub_seq) if *prev_num_sub_seq > 0 => {
                    *prev_num_sub_seq -= 1;
                    *num_sub_seq_map.entry(*num).or_insert(0) += 1;
                }
                _ => {
                    let cnt_num_plus_one = *cnt_map.get(&(*num + 1)).unwrap_or(&0);
                    if cnt_num_plus_one == 0 {
                        return false;
                    }

                    let cnt_num_plus_two = *cnt_map.get(&(*num + 2)).unwrap_or(&0);
                    if cnt_num_plus_two == 0 {
                        return false;
                    }

                    *cnt_map.get_mut(&(*num + 1)).unwrap() -= 1;
                    *cnt_map.get_mut(&(*num + 2)).unwrap() -= 1;
                    *num_sub_seq_map.entry(*num + 2).or_insert(0) += 1;
                }
            }
        }

        return true;
    }
}
