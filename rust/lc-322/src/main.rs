mod solution;

use solution::fast;
use solution::recursive;

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
    assert_eq!(3, fast::Solution::coin_change(coins_1.clone(), amount_1));
    assert_eq!(3, recursive::Solution::coin_change(coins_1, amount_1));
    assert_eq!(-1, fast::Solution::coin_change(coins_2.clone(), amount_2));
    assert_eq!(-1, recursive::Solution::coin_change(coins_2, amount_2));
    assert_eq!(0, fast::Solution::coin_change(coins_3.clone(), amount_3));
    assert_eq!(0, recursive::Solution::coin_change(coins_3, amount_3));
    assert_eq!(1, fast::Solution::coin_change(coins_4.clone(), amount_4));
    assert_eq!(1, recursive::Solution::coin_change(coins_4, amount_4));
    assert_eq!(2, fast::Solution::coin_change(coins_5.clone(), amount_5));
    assert_eq!(2, recursive::Solution::coin_change(coins_5, amount_5));
}
