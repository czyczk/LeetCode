pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut rec_pack = RecPack {
            ans: 0,
            branch_product: 1,
            pattern_map: std::collections::HashMap::new(),
            ret: 0,
        };

        let static_pack = StaticPack {
            wildcard_range: (b'1'..=b'9').into_iter().collect(),
            modulus: 10i32.pow(9) + 7,
        };

        rec_pack = Self::backtrace_rec(s.as_bytes(), rec_pack, &static_pack);

        rec_pack.ans
        //ret
    }

    fn backtrace_rec(pattern: &[u8], mut rec_pack: RecPack, static_pack: &StaticPack) -> RecPack {
        if pattern.is_empty() {
            rec_pack.ans = (rec_pack.ans + rec_pack.branch_product) % static_pack.modulus;
            //rec_pack.ret = rec_pack.branch_product;
            return rec_pack;
        }

        //let mut ret = 0;
        for seg_len in 1..=pattern.len() {
            rec_pack = Self::get_num_decodings(&pattern[..seg_len], rec_pack, static_pack);
            let num_ways = rec_pack.ret;
            if num_ways == 0 {
                break;
            }

            rec_pack.branch_product = ((rec_pack.branch_product as i64 * num_ways as i64)
                % static_pack.modulus as i64) as i32;
            //dbg!(String::from_utf8(pattern[..seg_len].to_vec()).unwrap());
            //dbg!(num_ways, rec_pack.branch_product);

            //dbg!(String::from_utf8(pattern[seg_len..].to_vec()).unwrap());
            rec_pack = Self::backtrace_rec(&pattern[seg_len..], rec_pack, static_pack);
            //let num_ways_latter = rec_pack.ret;
            //ret += num_ways * num_ways_latter;

            rec_pack.branch_product /= num_ways;
            //dbg!("stack out");
        }

        rec_pack
    }

    fn get_num_decodings(
        pattern: &[u8],
        mut rec_pack: RecPack,
        static_pack: &StaticPack,
    ) -> RecPack {
        match rec_pack.pattern_map.get(pattern) {
            Some(&ret) => {
                rec_pack.ret = ret;
            }
            None => {
                if Self::is_all_wildcards(pattern) {
                    // Wildcard mode
                    let mut ret: i32;
                    match pattern.len() {
                        1 => {
                            // "*" => 9
                            ret = 9;
                        }
                        2 => {
                            // "**" => sum("11", ..., "99")
                            // Actually count("11", ..., "19") + count("21", ..., "26")
                            ret = 0;
                            for &ch1 in static_pack.wildcard_range.iter().take(2) {
                                for &ch2 in static_pack.wildcard_range.iter() {
                                    rec_pack =
                                        Self::get_num_decodings(&[ch1, ch2], rec_pack, static_pack);
                                    ret += rec_pack.ret;
                                }
                            }

                            assert_eq!(ret, 15);
                        }
                        _ => {
                            // "***" => invalid
                            ret = 0;
                        }
                    }
                    rec_pack.pattern_map.insert(pattern.to_owned(), ret);
                    rec_pack.ret = ret;
                } else {
                    match pattern.len() {
                        1 => {
                            match pattern[0] {
                                b'0' => {
                                    // "0" => invalid
                                    rec_pack.pattern_map.insert(pattern.to_owned(), 0);
                                    rec_pack.ret = 0;
                                }
                                _ => {
                                    // "1", "2"... => 1
                                    rec_pack.pattern_map.insert(pattern.to_owned(), 1);
                                    rec_pack.ret = 1;
                                }
                            }
                        }
                        2 => {
                            match pattern[0] {
                                b'0' => {
                                    // "0#" => invalid
                                    rec_pack.pattern_map.insert(pattern.to_owned(), 0);
                                    rec_pack.ret = 0;
                                }
                                b'1' => {
                                    // "1#" => depends on the second char.
                                    // E.g.:
                                    // ["10", "19"] => 1;
                                    // "1*" => count("11", ..., "19") = 9;
                                    let mut ret: i32;
                                    match pattern[1] {
                                        //b'0' => {
                                        //    ret = 1;
                                        //}
                                        b'*' => {
                                            ret = 0;
                                            for &ch in static_pack.wildcard_range.iter() {
                                                rec_pack = Self::get_num_decodings(
                                                    &[pattern[0], ch],
                                                    rec_pack,
                                                    static_pack,
                                                );
                                                ret += rec_pack.ret;
                                            }
                                            assert_eq!(ret, 9);
                                        }
                                        _ => {
                                            ret = 1;
                                        }
                                    }
                                    rec_pack.pattern_map.insert(pattern.to_owned(), ret);
                                    rec_pack.ret = ret;
                                }
                                b'2' => {
                                    // "2#" => heavily depends on the second char.
                                    // E.g.:
                                    // ["20", "26"] => 1 * 1 + 1 = 2;
                                    // ["27", '29"] => invalid;
                                    // "2*" => count("21", ..., "29") = 6;
                                    let ret: i32;
                                    match pattern[1] {
                                        b'*' => {
                                            //ret = 0;
                                            //for ch in b'1'..=b'6' {
                                            //    rec_pack = Self::get_num_decodings(
                                            //        &[pattern[0], ch],
                                            //        rec_pack,
                                            //    );
                                            //    ret += rec_pack.ret;
                                            //}
                                            //assert_eq!(ret, 12);
                                            ret = 6;
                                        }
                                        ch if ch >= b'7' => {
                                            ret = 0;
                                        }
                                        _ => {
                                            ret = 1;
                                        }
                                    }
                                    rec_pack.pattern_map.insert(pattern.to_owned(), ret);
                                    rec_pack.ret = ret;
                                }
                                b'*' => {
                                    // "*#" => sum("0#", "1#", ..., "9#")
                                    // Considering only "1#" and "2#" is valid, it's actually
                                    // sum("1#", "2#")
                                    let mut ret = 0;
                                    rec_pack = Self::get_num_decodings(
                                        &[b'1', pattern[1]],
                                        rec_pack,
                                        static_pack,
                                    );
                                    ret += rec_pack.ret;
                                    rec_pack = Self::get_num_decodings(
                                        &[b'2', pattern[1]],
                                        rec_pack,
                                        static_pack,
                                    );
                                    ret += rec_pack.ret;
                                    rec_pack.pattern_map.insert(pattern.to_owned(), ret);
                                    rec_pack.ret = ret;
                                }
                                _ => {
                                    // ["3#", "9#"] => invalid
                                    let ret = 0;
                                    rec_pack.pattern_map.insert(pattern.to_owned(), ret);
                                    rec_pack.ret = ret;
                                }
                            }
                        }
                        _ => {
                            // "###" => invalid
                            rec_pack.pattern_map.insert(pattern.to_owned(), 0);
                            rec_pack.ret = 0;
                        }
                    }
                }
            }
        }

        rec_pack
    }

    fn is_all_wildcards(pattern: &[u8]) -> bool {
        pattern.iter().find(|&&x| x != b'*').is_none()
    }
}

struct RecPack {
    ans: i32,
    branch_product: i32,
    pattern_map: std::collections::HashMap<Vec<u8>, i32>,
    ret: i32,
}

struct StaticPack {
    wildcard_range: Vec<u8>,
    modulus: i32,
}
