mod solution;

use solution::Solution;

fn main() {
    let (g1, s1) = (vec![1, 2, 3], vec![1, 1]);
    let (g2, s2) = (vec![1, 2], vec![1, 2, 3]);
    let (g3, s3) = (vec![10, 9, 8, 7], vec![5, 6, 7, 8]);

    // Expecting 1
    println!("{}", Solution::find_content_children(g1, s1));
    // Expecting 2
    println!("{}", Solution::find_content_children(g2, s2));
    // Expecting 2
    println!("{}", Solution::find_content_children(g3, s3));
}
