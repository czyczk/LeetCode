mod solution;

use solution::Solution;

fn main() {
    let to_owned = |&string| String::from(string);
    let strs1 = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(to_owned)
        .collect::<Vec<String>>();
    let strs2 = vec!["".to_owned()];
    let strs3 = vec!["a".to_owned()];

    // Expecting [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]]
    let ans1 = Solution::group_anagrams(strs1);
    print_result(&ans1);
    // Expecting [[""]]
    let ans2 = Solution::group_anagrams(strs2);
    print_result(&ans2);
    // Expecting [["a"]]
    let ans3 = Solution::group_anagrams(strs3);
    print_result(&ans3);
}

fn print_result(result: &Vec<Vec<String>>) {
    print!("[");

    for (i, vec) in result.iter().enumerate() {
        print!("[");
        for (j, word) in vec.iter().enumerate() {
            print!("\"{}\"", word);
            if j < vec.len() - 1 {
                print!(", ");
            } else {
                print!("]");
            }
        }

        if i < result.len() - 1 {
            print!(", ");
        }
    }

    println!("]");
}
