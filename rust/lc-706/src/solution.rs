const BASE: usize = 769;

#[derive(Clone, Copy)]
struct Pair {
    key: i32,
    value: i32,
}

pub struct MyHashMap {
    data: Vec<Vec<Pair>>,
}

impl MyHashMap {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            data: vec![vec![]; BASE],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let idx = (key % BASE as i32) as usize;
        for entry in self.data[idx].iter_mut() {
            if entry.key == key {
                entry.value = value;
                return;
            }
        }
        self.data[idx].push(Pair{key, value});
    }

    pub fn remove(&mut self, key: i32) {
        let idx = (key % BASE as i32) as usize;

        let mut existing_key_idx = None;
        for (i, pair) in self.data[idx].iter().enumerate() {
            if pair.key == key {
                existing_key_idx = Some(i);
                break;
            }
        }

        if existing_key_idx.is_some() {
            self.data[idx].remove(existing_key_idx.unwrap());
        }
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no
     * mapping for the key */
    pub fn get(&self, key: i32) -> i32 {
        let idx = (key % BASE as i32) as usize;

        for entry in self.data[idx].iter() {
            if entry.key == key {
                return entry.value;
            }
        }

        return -1;
    }
}
