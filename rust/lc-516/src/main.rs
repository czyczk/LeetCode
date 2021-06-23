mod solution;

use solution::Solution;

fn main() {
    let s1 = "bbbab".to_owned();

    assert_eq!(4, Solution::longest_palindrome_subseq(s1));
}
