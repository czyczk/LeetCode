mod solution;

use solution::Solution;

fn main() {
    let n1 = vec![-4, -1, 0, 3, 10];
    let n2 = vec![-7, -3, 2, 3, 11];
    let n3 = vec![1];
    let n4 = vec![-1];
    let n5 = vec![0];

    assert_eq!(
        "[0, 1, 9, 16, 100]",
        format!("{:?}", Solution::sorted_squares(n1))
    );

    assert_eq!(
        "[4, 9, 9, 49, 121]",
        format!("{:?}", Solution::sorted_squares(n2))
    );

    assert_eq!("[1]", format!("{:?}", Solution::sorted_squares(n3)));

    assert_eq!("[1]", format!("{:?}", Solution::sorted_squares(n4)));

    assert_eq!("[0]", format!("{:?}", Solution::sorted_squares(n5)));
}
