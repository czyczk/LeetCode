mod solution;

use solution::KthLargest;

fn main() {
    let mut kl1 = KthLargest::new(3, vec![4, 5, 8, 2]);
    // 4
    println!("{}", kl1.add(3));
    // 5
    println!("{}", kl1.add(5));
    // 5
    println!("{}", kl1.add(10));
    // 8
    println!("{}", kl1.add(9));
    // 8
    println!("{}", kl1.add(4));
}
