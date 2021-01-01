mod solution;

use solution::Solution;

fn main() {
    let fb1 = vec![1, 0, 0, 0, 1];
    let n1 = 1;
    let fb2 = vec![1, 0, 0, 0, 1];
    let n2 = 2;
    let fb3 = vec![1, 0, 1, 0, 0];
    let n3 = 1;
    let fb4 = vec![0, 1, 0];
    let n4 = 1;
    let fb5 = vec![0,0,1,0,0,0,0,1,
    0,1,0,0,0,1,0,0,
    1,0,1,0,1,0,0,0,
    1,0,1,0,1,0,0,1,
    0,0,0,0,0,1,0,1,
    0,0,0,1,0,0,1,0,
    0,0,1,0,0,1,0,0,
    1,0,0,0,1,0,0,0,
    0,1,0,0,1,0,0,0,
    0,1,0,0,0,1,0,1,
    0,0,0,0,0,0];
    let n5 = 17;


    // Expecting true
    println!("{}", Solution::can_place_flowers(fb1, n1));
    // Expecting false
    println!("{}", Solution::can_place_flowers(fb2, n2));
    // Expecting true
    println!("{}", Solution::can_place_flowers(fb3, n3));
    // Expecting false
    println!("{}", Solution::can_place_flowers(fb4, n4));
    // Expecting false
    println!("{}", Solution::can_place_flowers(fb5, n5));
}
