pub struct Solution {}

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 4 {
            return vec![];
        }

        nums.sort_unstable();
        let mut ret = vec![];

        for (i, &n1) in nums.iter().enumerate() {
            if n1 > 0 && n1 > target {
                break;
            }

            if i > 0 && n1 == nums[i - 1] {
                continue;
            }

            for j in i + 1..n - 2 {
                let n2 = nums[j];

                if n2 > 0 && n2 as i32 > -n1 + target {
                    break;
                }

                if j > i + 1 && n2 == nums[j - 1] {
                    continue;
                }

                let mut l = j + 1;
                let mut r = n - 1;

                while l < r {
                    match (n2 + nums[l] + nums[r]).cmp(&(-n1 + target)) {
                        std::cmp::Ordering::Equal => {
                            ret.push(vec![n1, n2, nums[l], nums[r]]);

                            while l < r && nums[r] == nums[r - 1] {
                                r -= 1;
                            }

                            while l < r && nums[l] == nums[l + 1] {
                                l += 1;
                            }

                            l += 1;
                            r -= 1;
                        }
                        std::cmp::Ordering::Less => {
                            l += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            r -= 1;
                        }
                    }
                }
            }
        }

        ret
    }
}
