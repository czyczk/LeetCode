pub struct Solution {}

const NUMS: [&str; 20] = [
    "Zero",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];
const TIES: [&str; 10] = [
    "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];
const UNITS: [&str; 5] = ["", "Thousand", "Million", "Billion", "Trillion"];

impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_owned();
        }

        let mut unit = 0;

        let mut ret = vec![];
        while num != 0 {
            let temp = num % 1000;
            num /= 1000;

            let mut temp_result = Self::to_words(temp);
            if unit > 0 && !temp_result.is_empty() {
                temp_result.push(UNITS[unit]);
            }
            temp_result.extend_from_slice(&ret);
            ret = temp_result;

            unit += 1;
        }

        ret.join(" ")
    }

    fn to_words(mut num: i32) -> Vec<&'static str> {
        let mut ret = vec![];

        if num >= 100 {
            ret.push(NUMS[(num / 100) as usize]);
            ret.push("Hundred");
            num %= 100;
        }

        if num == 0 {
            return ret;
        }

        if num < 20 {
            ret.push(NUMS[num as usize])
        } else {
            ret.push(TIES[(num / 10) as usize]);
            if num % 10 != 0 {
                ret.push(NUMS[(num % 10) as usize]);
            }
        }

        ret
    }
}
