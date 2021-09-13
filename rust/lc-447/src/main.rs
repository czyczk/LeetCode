mod solution;

use solution::Solution;

fn main() {
    let p1 = vec![vec![0, 0], vec![1, 0], vec![2, 0]];
    let p2 = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    let p3 = vec![vec![1, 1]];
    let p4 = vec![vec![0, 0], vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];

    assert_eq!(2, Solution::number_of_boomerangs(p1));
    assert_eq!(2, Solution::number_of_boomerangs(p2));
    assert_eq!(0, Solution::number_of_boomerangs(p3));
    assert_eq!(20, Solution::number_of_boomerangs(p4));
}
