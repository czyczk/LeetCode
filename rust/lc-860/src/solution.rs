pub struct Solution {}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut fives, mut tens) = (0 as i16, 0 as i16);

        for bill in bills.iter() {
            match bill {
                5 => fives += 1,
                10 => {
                    if fives == 0 {
                        return false;
                    }

                    fives -= 1;
                    tens += 1;
                }
                _ => {
                    if tens != 0 {
                        if fives != 0 {
                            tens -= 1;
                            fives -= 1;
                        } else {
                            return false;
                        }
                    } else if fives > 2 {
                        fives -= 3;
                    } else {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}
