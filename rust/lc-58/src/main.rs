mod solution;

use solution::Solution;

fn main() {
    assert_eq!(5, Solution::length_of_last_word("Hello World".to_owned()));
    assert_eq!(
        4,
        Solution::length_of_last_word("   fly me   to   the moon  ".to_owned())
    );
    assert_eq!(
        6,
        Solution::length_of_last_word("luffy is still joyboy".to_owned())
    );
}
