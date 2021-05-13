#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct NestedIterator {
    list: Vec<i32>,
    cur_idx: usize,
}

impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut list = vec![];
        let mut stack = vec![nested_list.iter()];
        while !stack.is_empty() {
            match stack.last_mut().unwrap().next() {
                Some(ni) => match ni {
                    NestedInteger::Int(num) => list.push(*num),
                    NestedInteger::List(l) => stack.push(l.iter()),
                },
                None => {
                    stack.pop().unwrap();
                }
            }
        }

        Self { list, cur_idx: 0 }
    }

    pub fn next(&mut self) -> i32 {
        let ret = self.list[self.cur_idx];
        self.cur_idx += 1;
        ret
    }

    pub fn has_next(&self) -> bool {
        self.cur_idx < self.list.len()
    }
}
