mod solution;

fn main() {
    let na1 = solution::NumArray::new(vec![-2, 0, 3, -5, 2, -1]);

    // Expecting 1
    println!("{}", na1.sum_range(0, 2));
    // Expecting -1
    println!("{}", na1.sum_range(2, 5));
    // Expecting -3
    println!("{}", na1.sum_range(0, 5));
    
}
