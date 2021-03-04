mod solution;

use solution::Solution;

fn main() {
    let env1 = vec![
        vec![5, 4],
        vec![6, 4],
        vec![6, 7],
        vec![2, 3],
    ];

    let env2 = vec![
        vec![1, 1],
        vec![1, 1],
        vec![1, 1],
    ];

    // Expecting 3
    println!("{}", Solution::max_envelopes(env1));

    // Expecting 1
    println!("{}", Solution::max_envelopes(env2));
}
