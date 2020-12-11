use solution::Solution;

mod solution;
fn main() {
    let senate1 = String::from("RD");
    let senate2 = String::from("RDD");
    let senate3 = String::from("RDRDD");

    // Expecting "Radiant"
    println!("{}", Solution::predict_party_victory(senate1));
    // Expecting "Dire"
    println!("{}", Solution::predict_party_victory(senate2));
    // Expecting "Radiant"
    println!("{}", Solution::predict_party_victory(senate3));
}
