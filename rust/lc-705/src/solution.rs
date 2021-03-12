const BASE: usize = 769;

pub struct MyHashSet {
    data: Vec<Vec<i32>>,
}

impl MyHashSet {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            data: vec![vec![]; BASE],
        }
    }

    pub fn add(&mut self, key: i32) {
        let idx = (key % BASE as i32) as usize;
        for k in self.data[idx].iter() {
            if k == &key {
                return;
            }
        }
        self.data[idx].push(key);
    }

    pub fn remove(&mut self, key: i32) {
        let idx = (key % BASE as i32) as usize;

        let mut existing_key_idx = None;
        for (i, &k) in self.data[idx].iter().enumerate() {
            if k == key {
                existing_key_idx = Some(i);
                break;
            }
        }

        if existing_key_idx.is_some() {
            self.data[idx].remove(existing_key_idx.unwrap());
        }
    }

    /** Returns true if this set contains the specified element */
    pub fn contains(&self, key: i32) -> bool {
        let idx = (key % BASE as i32) as usize;
        self.data[idx].contains(&key)
    }
}
