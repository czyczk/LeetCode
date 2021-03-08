mod solution;

use solution::Solution;

fn main() {
    let s1 = "abbaca".to_owned();

    // Expecting "ca"
    println!("{}", Solution::remove_duplicates(s1));
}
