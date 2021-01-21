mod solution;

use solution::Solution;

fn main() {
    let (n1, m1) = (8, 2);
    let group1 = vec![-1, -1, 1, 0, 0, 1, 0, -1];
    let before_items1 = vec![
        vec![],
        vec![6],
        vec![5],
        vec![6],
        vec![3, 6],
        vec![],
        vec![],
        vec![],
    ];

    let (n2, m2) = (8, 2);
    let group2 = vec![-1, -1, 1, 0, 0, 1, 0, -1];
    let before_items2 = vec![
        vec![],
        vec![6],
        vec![5],
        vec![6],
        vec![3],
        vec![],
        vec![4],
        vec![],
    ];

    let (n3, m3) = (5, 5);
    let group3 = vec![2, 0, -1, 3, 0];
    let before_items3 = vec![vec![2, 1, 3], vec![2, 4], vec![], vec![], vec![]];

    // Expecting [6, 3, 4, 1, 5, 2, 0, 7]
    println!("{:?}", Solution::sort_items(n1, m1, group1, before_items1));
    // Expecting []
    println!("{:?}", Solution::sort_items(n2, m2, group2, before_items2));
    // Expecting ?
    println!("{:?}", Solution::sort_items(n3, m3, group3, before_items3));
}
