mod solution;

use solution::Solution;

fn main() {
    let m1 = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    let m2 = vec![vec!['0']];
    let m3 = vec![vec!['0', '1'], vec!['1', '0']];

    assert_eq!(4, Solution::maximal_square(m1));
    assert_eq!(0, Solution::maximal_square(m2));
    assert_eq!(1, Solution::maximal_square(m3));
}
