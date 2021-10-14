use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

thread_local! {static ANS: RefCell<Vec<Rc<String>>> = RefCell::new(Vec::with_capacity(30));}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        (*Self::get_ans((n - 1) as usize)).clone()
    }

    fn get_ans(n: usize) -> Rc<String> {
        ANS.with(|ans| {
            if ans.borrow().len() > n {
                return ans.borrow()[n].clone();
            }

            if n == 0 {
                let ret = Rc::new("1".to_owned());
                ans.borrow_mut().push(ret.clone());
                return ret;
            }

            let last_ans = Self::get_ans(n - 1);
            let mut last_ch = b'\0';
            let mut cnt = 0;
            let mut ret = String::new();
            for (i, &ch) in last_ans.as_bytes().iter().enumerate() {
                if i == 0 {
                    last_ch = ch;
                    cnt = 1;
                    continue;
                }

                if ch == last_ch {
                    cnt += 1;
                } else {
                    ret.push_str(&format!("{}{}", cnt, last_ch as char));
                    last_ch = ch;
                    cnt = 1;
                }
            }
            ret.push_str(&format!("{}{}", cnt, last_ch as char));

            let ret = Rc::new(ret);
            ans.borrow_mut().push(ret.clone());
            return ret;
        })
    }
}
