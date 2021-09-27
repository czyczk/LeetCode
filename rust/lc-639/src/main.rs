mod solution;

use solution::Solution;

fn main() {
    //assert_eq!(9, Solution::num_decodings("*".to_owned()));
    //assert_eq!(18, Solution::num_decodings("1*".to_owned()));
    //assert_eq!(15, Solution::num_decodings("2*".to_owned()));
    //assert_eq!(96, Solution::num_decodings("**".to_owned()));
    assert_eq!(
        196465252,
        Solution::num_decodings("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*".to_owned())
    );
}
