mod solution;

use solution::Solution;

fn main() {
    let a1 = vec![
        vec![1, 1, 0],
        vec![1, 0, 1],
        vec![0, 0, 0],
    ];

    let a2 = vec![
        vec![1, 1, 0, 0],
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 1],
        vec![1, 0, 1, 0],
    ];

    // Expecting [[1, 0, 0], [0, 1, 0], [1, 1, 1]]
    println!("{:?}", Solution::flip_and_invert_image(a1));
    // Expecting [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 1], [1, 0, 1, 0]]
    println!("{:?}", Solution::flip_and_invert_image(a2));
}
