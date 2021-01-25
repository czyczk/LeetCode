mod solution;

use solution::Solution;

fn main() {
    let grid1 = vec![" /".to_owned(), "/ ".to_owned()];
    let grid2 = vec![" /".to_owned(), "  ".to_owned()];
    let grid3 = vec!["\\/".to_owned(), "/\\".to_owned()];
    let grid4 = vec!["/\\".to_owned(), "\\/".to_owned()];
    let grid5 = vec!["//".to_owned(), "/ ".to_owned()];
    
    // Expecting 2
    println!("{}", Solution::regions_by_slashes(grid1));
    // Expecting 1
    println!("{}", Solution::regions_by_slashes(grid2));
    // Expecting 4
    println!("{}", Solution::regions_by_slashes(grid3));
    // Expecting 5
    println!("{}", Solution::regions_by_slashes(grid4));
    // Expecting 3
    println!("{}", Solution::regions_by_slashes(grid5));
}
