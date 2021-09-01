mod solution;

use solution::MedianFinder;

fn main() {
    let mut mf = MedianFinder::new();
    mf.add_num(1);
    mf.add_num(2);
    assert_eq!(1.5, mf.find_median());
    mf.add_num(3);
    assert_eq!(2.0, mf.find_median());
}
