mod solution;

use solution::SolutionCenter;
use solution::SolutionManacher;

fn main() {
    let str1 = "abc".to_owned();
    let str2 = "aaa".to_owned();

    // Expecting 3
    assert_eq!(3, SolutionCenter::count_substrings(str1.clone()));
    assert_eq!(3, SolutionManacher::count_substrings(str1));
    // Expecting 6
    assert_eq!(6, SolutionCenter::count_substrings(str2.clone()));
    assert_eq!(6, SolutionManacher::count_substrings(str2));
}
