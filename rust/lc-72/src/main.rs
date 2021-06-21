mod solution;

use solution::Solution;

fn main() {
    let (w11, w12) = ("horse".to_owned(), "ros".to_owned());
    let (w21, w22) = ("intention".to_owned(), "execution".to_owned());

    assert_eq!(3, Solution::min_distance(w11, w12));
    assert_eq!(5, Solution::min_distance(w21, w22));
}
