mod solution;

use solution::Solution;

fn main() {
    let n1 = 123;
    let n2 = 12345;
    let n3 = 1234567;
    let n4 = 1234567891;
    let n5 = 0;
    let n6 = 10;
    let n7 = 20;
    let n8 = 100;
    let n9 = 1000;
    let n10 = 1000000;

    assert_eq!("One Hundred Twenty Three", Solution::number_to_words(n1));
    assert_eq!(
        "Twelve Thousand Three Hundred Forty Five",
        Solution::number_to_words(n2)
    );
    assert_eq!(
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven",
        Solution::number_to_words(n3)
    );
    assert_eq!("One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One", Solution::number_to_words(n4));
    assert_eq!("Zero", Solution::number_to_words(n5));
    assert_eq!("Ten", Solution::number_to_words(n6));
    assert_eq!("Twenty", Solution::number_to_words(n7));
    assert_eq!("One Hundred", Solution::number_to_words(n8));
    assert_eq!("One Thousand", Solution::number_to_words(n9));
    assert_eq!("One Million", Solution::number_to_words(n10));
}
