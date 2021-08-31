mod solution;

use solution::LFUCache;

fn main() {
    let mut c = LFUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    assert_eq!(1, c.get(1));
    c.put(3, 3);
    assert_eq!(-1, c.get(2));
    assert_eq!(3, c.get(3));
    c.put(4, 4);
    assert_eq!(-1, c.get(1));
    assert_eq!(3, c.get(3));
    assert_eq!(4, c.get(4));
}
