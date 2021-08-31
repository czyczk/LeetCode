pub struct Solution {}

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }

        let mut rec_pack = RecPack {
            gen_num_map: std::collections::HashMap::new(),
            max_num: 0,
        };

        for i in n / 2..=n {
            rec_pack = Solution::get_generated_num(i, rec_pack);
        }

        return rec_pack.max_num;
    }

    fn get_generated_num(n: i32, mut rec_pack: RecPack) -> RecPack {
        match rec_pack.gen_num_map.get(&n) {
            Some(_) => {}
            None => match n {
                0 | 1 => {
                    rec_pack.gen_num_map.insert(n, n);
                    rec_pack.max_num = rec_pack.max_num.max(n);
                }
                _ => {
                    rec_pack = Solution::get_generated_num(n / 2, rec_pack);
                    let mut sum = *rec_pack.gen_num_map.get(&(n / 2)).unwrap();

                    if n % 2 != 0 {
                        rec_pack = Solution::get_generated_num(n / 2 + 1, rec_pack);
                        sum += *rec_pack.gen_num_map.get(&(n / 2 + 1)).unwrap();
                    }

                    rec_pack.gen_num_map.insert(n, sum);
                    if sum > rec_pack.max_num {
                        rec_pack.max_num = sum;
                    }
                }
            },
        }

        return rec_pack;
    }
}

struct RecPack {
    gen_num_map: std::collections::HashMap<i32, i32>,
    max_num: i32,
}
