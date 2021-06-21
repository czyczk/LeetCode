mod solution;

use solution::Solution;

fn main() {
    let (s1, k1) = ("abcdefg".to_owned(), 2);
    let (s2, k2) = ("lrloseumgh".to_owned(), 6);

    assert_eq!("cdefgab", Solution::reverse_left_words(s1, k1));
    assert_eq!("umghlrlose", Solution::reverse_left_words(s2, k2));
}
