mod solution;

use solution::Solution;

fn main() {
    let coins_1 = vec![1, 2, 5];
    let amount_1 = 11;

    let coins_2 = vec![2];
    let amount_2 = 3;

    let coins_3 = vec![1];
    let amount_3 = 0;

    let coins_4 = vec![1];
    let amount_4 = 1;

    let coins_5 = vec![1];
    let amount_5 = 2;

    // 11 = 5 + 5 + 1
    assert_eq!(3, Solution::coin_change(coins_1, amount_1));
    assert_eq!(-1, Solution::coin_change(coins_2, amount_2));
    assert_eq!(0, Solution::coin_change(coins_3, amount_3));
    assert_eq!(1, Solution::coin_change(coins_4, amount_4));
    assert_eq!(2, Solution::coin_change(coins_5, amount_5));
}
