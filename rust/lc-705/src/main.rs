mod solution;

use solution::MyHashSet;

fn main() {
    let mut hs = MyHashSet::new();
    hs.add(1);
    hs.add(2);
    assert_eq!(true, hs.contains(1));
    assert_eq!(false, hs.contains(3));
    hs.add(2);
    assert_eq!(true, hs.contains(2));
    hs.remove(2);
    assert_eq!(false, hs.contains(2));
}
