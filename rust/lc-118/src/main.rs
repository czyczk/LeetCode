mod solution;

fn main() {
    let num_rows = 5;

    let result = solution::Solution::generate(num_rows);
    for row in result.iter() {
        for num in row.iter() {
            println!("{}", num);
        }
    }
}
