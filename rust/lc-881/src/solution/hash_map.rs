/*
 * Use a HashMap to store the people array. However, it takes O(max(people)) to find
 * the other co-passenger. So the overall time consumption is O(n max(people)).
 */
pub struct Solution {}

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut m = std::collections::HashMap::new();
        for &p in people.iter() {
            *m.entry(p).or_insert(0) += 1;
        }

        let mut num_boats = 0;
        for p in people.iter() {
            match m.get_mut(p) {
                None => continue,
                Some(num_p) => {
                    if *num_p == 1 {
                        m.remove(p);
                    } else {
                        *num_p -= 1;
                    }
                }
            }

            let max_left = limit - *p;
            for left in (1..=max_left).rev() {
                match m.get_mut(&left) {
                    None => continue,
                    Some(num_left) => {
                        if *num_left == 1 {
                            m.remove(&left);
                        } else {
                            *num_left -= 1;
                        }

                        break;
                    }
                }
            }

            num_boats += 1;
        }

        num_boats
    }
}
