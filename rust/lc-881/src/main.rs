mod solution;

use solution::best_solution;
use solution::hash_map;

fn main() {
    let (p1, l1) = (vec![1, 2], 3);
    let (p2, l2) = (vec![3, 2, 2, 1], 3);
    let (p3, l3) = (vec![3, 5, 3, 4], 5);
    let (p4, l4) = (vec![2, 2], 6);
    let (p5, l5) = (vec![3, 1, 7], 7);

    assert_eq!(1, hash_map::Solution::num_rescue_boats(p1.clone(), l1));
    assert_eq!(1, best_solution::Solution::num_rescue_boats(p1, l1));
    assert_eq!(3, hash_map::Solution::num_rescue_boats(p2.clone(), l2));
    assert_eq!(3, best_solution::Solution::num_rescue_boats(p2, l2));
    assert_eq!(4, hash_map::Solution::num_rescue_boats(p3.clone(), l3));
    assert_eq!(4, best_solution::Solution::num_rescue_boats(p3, l3));
    assert_eq!(1, hash_map::Solution::num_rescue_boats(p4.clone(), l4));
    assert_eq!(1, best_solution::Solution::num_rescue_boats(p4, l4));
    assert_eq!(2, hash_map::Solution::num_rescue_boats(p5.clone(), l5));
    assert_eq!(2, best_solution::Solution::num_rescue_boats(p5, l5));
}
