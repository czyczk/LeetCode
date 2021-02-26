mod solution;

use solution::Solution;

fn main() {
    let words1 = to_vec_of_string(vec!["aaaa", "asas", "able", "ability", "actt", "actor", "access"]);
    let puzzles1 = to_vec_of_string(vec!["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"]);

    // Expecting [1, 1, 3, 2, 4, 0]
    println!("{:?}", Solution::find_num_of_valid_words(words1, puzzles1));
}

fn to_vec_of_string(vec: Vec<&str>) -> Vec<String> {
    vec.into_iter().map(|str| str.to_owned()).collect()
}
