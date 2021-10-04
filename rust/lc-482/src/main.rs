mod solution;

use solution::Solution;

fn main() {
    assert_eq!(
        "5F3Z-2E9W",
        Solution::license_key_formatting("5F3Z-2e-9-w".to_owned(), 4)
    );

    assert_eq!(
        "2-5G-3J",
        Solution::license_key_formatting("2-5g-3-J".to_owned(), 2)
    );
}
