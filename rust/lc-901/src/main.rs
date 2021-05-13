mod solution;

use solution::StockSpanner;

fn main() {
    let mut ss = StockSpanner::new();

    assert_eq!(1, ss.next(100));
    assert_eq!(1, ss.next(80));
    assert_eq!(1, ss.next(60));
    assert_eq!(2, ss.next(70));
    assert_eq!(1, ss.next(60));
    assert_eq!(4, ss.next(75));
    assert_eq!(6, ss.next(85));
}
