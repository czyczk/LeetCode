mod solution;

use solution::Solution;

fn main() {
    let f1 = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    let (n1, src1, dst1, k1) = (3, 0, 2, 1);

    let f2 = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    let (n2, src2, dst2, k2) = (3, 0, 2, 0);

    assert_eq!(200, Solution::find_cheapest_price(n1, f1, src1, dst1, k1));
    assert_eq!(500, Solution::find_cheapest_price(n2, f2, src2, dst2, k2));
}
