mod solution;

use solution::Solution;

fn main() {
    let customers1 = vec![1, 0, 1, 2, 1, 1, 7, 5];
    let grumpy1 = vec![0, 1, 0, 1, 0, 1, 0, 1];
    let x1 = 3;

    let customers2 = vec![2, 6, 6, 9];
    let grumpy2 = vec![0, 0, 1, 1];
    let x2 = 1;

    let customers3 = vec![3, 8, 8, 7, 1];
    let grumpy3 = vec![1, 1, 1, 1, 1];
    let x3 = 3;

    // Expecting 16
    println!("{}", Solution::max_satisfied(customers1, grumpy1, x1));
    // Expecting 17
    println!("{}", Solution::max_satisfied(customers2, grumpy2, x2));
    // Expecting 23
    println!("{}", Solution::max_satisfied(customers3, grumpy3, x3));
}
