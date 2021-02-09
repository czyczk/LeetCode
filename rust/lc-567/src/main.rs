mod solution;

use solution::Solution;

fn main() {
    let s1 = "ab".to_owned();
    let t1 = "eidbaooo".to_owned();

    let s2 = "ab".to_owned();
    let t2 = "eidboaoo".to_owned();

    // Expecting true
    println!("{}", Solution::check_inclusion(s1, t1));
    // Expecting false
    println!("{}", Solution::check_inclusion(s2, t2));
}
