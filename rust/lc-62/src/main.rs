use solution::Solution;

mod solution;

fn main() {
    let (m1, n1) = (3, 7);
    let (m2, n2) = (3, 2);
    let (m3, n3) = (7, 3);
    let (m4, n4) = (3, 3);
    let (m5, n5) = (1, 1);
    let (m6, n6) = (23, 12);

    // Expecting 28
    println!("{}", Solution::unique_paths(m1, n1));
    // Expecting 3
    println!("{}", Solution::unique_paths(m2, n2));
    // Expecting 28
    println!("{}", Solution::unique_paths(m3, n3));
    // Expecting 6
    println!("{}", Solution::unique_paths(m4, n4));
    // Expecting 1
    println!("{}", Solution::unique_paths(m5, n5));
    // Expecting 193536720
    println!("{}", Solution::unique_paths(m6, n6));
}
