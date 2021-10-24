mod solution;

use solution::Solution;

fn main() {
    let price1 = vec![2, 5];
    let special1 = vec![vec![3, 0, 5], vec![1, 2, 10]];
    let needs1 = vec![3, 2];

    let price2 = vec![2, 3, 4];
    let special2 = vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]];
    let needs2 = vec![1, 2, 1];

    let price3 = price2.clone();
    let special3 = special2.clone();
    let needs3 = vec![];

    assert_eq!(14, Solution::shopping_offers(price1, special1, needs1));
    assert_eq!(11, Solution::shopping_offers(price2, special2, needs2));
    assert_eq!(0, Solution::shopping_offers(price3, special3, needs3));
}
