pub struct Solution {}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }

        let len = flowerbed.len();
        match len {
            0 => return false,
            1 => return flowerbed[0] == 0,
            _ => {}
        }
        
        let mut num_planted = 0;
        let mut prev = -1;
        
        for i in 0..len {
            if flowerbed[i] == 1 {
                if prev < 0 {
                    num_planted += i as i32 / 2;
                } else {
                    num_planted += (i as i32 - prev - 2) / 2;
                }

                if num_planted >= n {
                    return true;
                }

                prev = i as i32;
            }

           
        }

        if prev < 0 {
            num_planted += (len as i32 + 1) / 2;
        } else {
            num_planted += (len as i32 - prev - 1) / 2;
        }

        num_planted >= n
    }
}