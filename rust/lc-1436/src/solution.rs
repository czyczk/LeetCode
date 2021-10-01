pub struct Solution {}

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let n = paths.len();
        let mut src_set = std::collections::HashSet::with_capacity(n);
        let mut dest_set = std::collections::HashSet::new();

        for path in paths.iter() {
            src_set.insert(&path[0][..]);
            if !src_set.contains(&path[1][..]) {
                dest_set.insert(&path[1][..]);
            }
        }

        for dest in dest_set {
            if !src_set.contains(dest) {
                return dest.to_owned();
            }
        }

        return "".to_owned();
    }
}
