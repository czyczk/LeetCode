mod solution;

use solution::Solution;

fn main() {
    let g1 = vec![vec![1, 2], vec![3], vec![3], vec![]];
    let g2 = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
    let g3 = vec![vec![1], vec![]];

    check_eq(
        vec![vec![0, 1, 3], vec![0, 2, 3]],
        Solution::all_paths_source_target(g1),
    );
    check_eq(
        vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ],
        Solution::all_paths_source_target(g2),
    );
    check_eq(vec![vec![0, 1]], Solution::all_paths_source_target(g3));
}

fn check_eq(expected: Vec<Vec<i32>>, actual: Vec<Vec<i32>>) {
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
