mod solution;

use solution::SolutionOriginal;

fn main() {
    let (k1, prices1) = (2, vec![2, 4, 1]);
    let (k2, prices2) = (2, vec![3, 2, 6, 5, 0, 3]);
    let (k3, prices3) = (2, vec![4, 2, 1, 7]);

    // Expecting 2
    println!("{}", SolutionOriginal::max_profit(k1, prices1));
    // Expecting 7
    println!("{}", SolutionOriginal::max_profit(k2, prices2));
    // Expecting 6
    println!("{}", SolutionOriginal::max_profit(k3, prices3));
}
