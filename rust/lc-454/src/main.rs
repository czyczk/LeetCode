mod solution;

fn main() {
    let a = vec![1, 2];
    let b = vec![-2, -1];
    let c = vec![-1, 2];
    let d = vec![0, 2];

    let count = solution::Solution::four_sum_count(a, b, c, d);
    // Expecting 2
    println!("{}", count);
}
