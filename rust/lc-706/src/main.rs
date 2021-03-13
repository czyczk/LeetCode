mod solution;

use solution::MyHashMap;

fn main() {
    let mut mhm = MyHashMap::new();
    mhm.put(1, 1);
    mhm.put(2, 2);
    assert_eq!(1, mhm.get(1));
    assert_eq!(-1, mhm.get(3));
    mhm.put(2, 1);
    assert_eq!(1, mhm.get(2));
    mhm.remove(2);
    assert_eq!(-1, mhm.get(2));
}
