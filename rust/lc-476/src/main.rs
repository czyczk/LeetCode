mod solution;

use solution::Solution;

fn main() {
    assert_eq!(2, Solution::find_complement(5));
    assert_eq!(0, Solution::find_complement(1));
    assert_eq!(13393220, Solution::find_complement(20161211));
}

fn print_as_binary(num: i32) {
    let num = num;
    let mut mask = 0x40000000_i32;
    for _ in 0..32 {
        print!("{}", (num & mask != 0) as i32);
        mask >>= 1;
    }

    println!();
}
