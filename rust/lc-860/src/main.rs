use solution::Solution;

mod solution;

fn main() {
    let bills1 = vec![5, 5, 5, 10, 20];
    let bills2 = vec![5, 5, 10];
    let bills3 = vec![10, 10];
    let bills4 = vec![5, 5, 10, 10, 20];
    let bills5 = vec![
        5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5,
    ];

    // Expecting true
    println!("{}", Solution::lemonade_change(bills1));
    // Expecting true
    println!("{}", Solution::lemonade_change(bills2));
    // Expecting false
    println!("{}", Solution::lemonade_change(bills3));
    // Expecting false
    println!("{}", Solution::lemonade_change(bills4));
    // Expecting true
    println!("{}", Solution::lemonade_change(bills5));
}
