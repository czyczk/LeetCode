pub struct MyQueue {
    push_vec: Vec<i32>,
    pop_vec: Vec<i32>,
}

impl MyQueue {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            push_vec: vec![],
            pop_vec: vec![],
        }
    }
    
    /** Push element x to the back of queue. */
    pub fn push(&mut self, x: i32) {
        self.push_vec.push(x);
    }
    
    /** Removes the element from in front of queue and returns that element. */
    pub fn pop(&mut self) -> i32 {
        match self.pop_vec.pop() {
            Some(val) => val,
            None => {
                self.prepare_pop_vec();
                self.pop_vec.pop().unwrap()
            }
        }
    }
    
    /** Get the front element. */
    pub fn peek(&mut self) -> i32 {
        match self.pop_vec.last() {
            Some(val) => *val,
            None => {
                self.prepare_pop_vec();
                *self.pop_vec.last().unwrap()
            }
        }
    }
    
    /** Returns whether the queue is empty. */
    pub fn empty(&self) -> bool {
        self.push_vec.is_empty() && self.pop_vec.is_empty()
    }

    fn prepare_pop_vec(&mut self) {
        while !self.push_vec.is_empty() {
            self.pop_vec.push(self.push_vec.pop().unwrap());
        }
    }
}

