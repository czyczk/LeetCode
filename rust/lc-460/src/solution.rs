pub struct LFUCache {
    capacity: usize,
    cache_map: std::collections::HashMap<i32, std::rc::Rc<std::cell::RefCell<CacheItem>>>,
    freq_map: std::collections::HashMap<
        u32,
        std::collections::VecDeque<std::rc::Rc<std::cell::RefCell<CacheItem>>>,
    >,
    min_freq: u32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        return Self {
            capacity: capacity as usize,
            cache_map: std::collections::HashMap::new(),
            freq_map: std::collections::HashMap::new(),
            min_freq: 1,
        };
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }

        match self.cache_map.get_mut(&key) {
            None => {
                return -1;
            }
            Some(item) => {
                // Update the freq: freq += 1
                let old_freq = item.borrow().freq;
                item.borrow_mut().incr_freq();
                // Remove the CacheItem from the linked list of oldFreq and add it to the
                // linked list of the new freq.
                let old_freq_list = self.freq_map.get_mut(&old_freq).unwrap();
                match old_freq_list.iter().position(|it| it.borrow().key == key) {
                    None => panic!("Invalid state of `old_freq_list`."),
                    Some(idx) => {
                        old_freq_list.remove(idx);
                    }
                }
                //old_freq_list.retain(|it| it.borrow().key != key);
                if old_freq == self.min_freq && old_freq_list.is_empty() {
                    // If the linked list of oldFreq is empty and has an impact on minFreq,
                    // increment minFreq by 1, as minReq is now the updated freq.
                    self.min_freq += 1;
                }

                self.freq_map
                    .entry(item.borrow().freq)
                    .or_insert(std::collections::VecDeque::new())
                    .push_back(item.clone());

                return item.borrow().value;
            }
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        let existing_val;
        {
            existing_val = self.get(key);
        }
        match existing_val {
            -1 => {
                if self.cache_map.len() == self.capacity {
                    // The cache is full, remove the LFU (or LRU on tie) item.
                    // No need to consider if the freq list is empty, since a new item
                    // will be inserted soon, which will make minFreq 1 again.
                    let lfu_item = self
                        .freq_map
                        .get_mut(&self.min_freq)
                        .unwrap()
                        .pop_front()
                        .unwrap();
                    self.cache_map.remove(&lfu_item.borrow().key);
                }

                let new_item =
                    std::rc::Rc::new(std::cell::RefCell::new(CacheItem::new(key, value)));

                self.freq_map
                    .entry(1)
                    .or_insert(std::collections::VecDeque::new())
                    .push_back(new_item.clone());

                self.min_freq = 1;

                self.cache_map.insert(key, new_item);
            }
            val if val != value => {
                self.cache_map.get_mut(&key).unwrap().borrow_mut().value = value;
            }
            _ => {}
        }
    }
}

struct CacheItem {
    key: i32,
    value: i32,
    freq: u32,
}

impl CacheItem {
    pub fn new(key: i32, value: i32) -> Self {
        return Self {
            key,
            value,
            freq: 1,
        };
    }

    pub fn incr_freq(&mut self) {
        self.freq += 1;
    }
}
