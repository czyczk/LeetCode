mod solution;

use data_encoding::HEXUPPER;
use ring::digest::{Context, SHA256};
use solution::Solution;

macro_rules! hashset {
    ($( $key: expr ),*) => {{
         let mut set = ::std::collections::HashSet::new();
         $( set.insert($key.into()); )*
         set
    }}
}

fn main() {
    //println!("{}", Solution::evaluate("-5*6+7*8-9".as_bytes()));
    let ans1 = hashset!["1*2*3", "1+2+3"];
    let ans2 = hashset!["2*3+2", "2+3*2"];
    let ans3 = hashset!["1*0+5", "10-5"];
    let ans4 = hashset!["0*0", "0+0", "0-0"];
    let ans5 = std::collections::HashSet::new();
    let ans6 = ans5.clone();
    let ans7 = hashset!["1"];
    let ans8 = get_ans8();

    check_ans(ans1, Solution::add_operators("123".into(), 6));
    check_ans(ans2, Solution::add_operators("232".into(), 8));
    check_ans(ans3, Solution::add_operators("105".into(), 5));
    check_ans(ans4, Solution::add_operators("00".into(), 0));
    check_ans(ans5, Solution::add_operators("3456237490".into(), 9191));
    check_ans(ans6, Solution::add_operators("1".into(), 0));
    check_ans(ans7, Solution::add_operators("1".into(), 1));
    check_ans(ans8, Solution::add_operators("123456789".into(), 45));
}

fn check_ans(mut expected: std::collections::HashSet<String>, actual: Vec<String>) {
    assert_eq!(expected.len(), actual.len());

    let mut only_in_actual = vec![];
    for exp in actual.iter() {
        if !expected.remove(exp) {
            only_in_actual.push(exp);
        }
    }

    if !expected.is_empty() {
        println!("{} expressions only in expected:", expected.len());
        for exp in expected.iter() {
            print!("{} -> ", exp);
            println!("{}", calc_hash(exp));
        }
    }

    if !only_in_actual.is_empty() {
        println!("{} expressions only in actual:", only_in_actual.len());
        for exp in only_in_actual.iter() {
            print!("{} -> ", exp);
            println!("{}", calc_hash(exp));
        }
    }

    assert_eq!(true, expected.is_empty());
    assert_eq!(true, only_in_actual.is_empty());
}

fn get_ans8() -> std::collections::HashSet<String> {
    let mut set = std::collections::HashSet::with_capacity(121);
    set.insert("1+2+3+4+5+6+7+8+9".into());
    set.insert("1+2+3+4+5-6*7+8*9".into());
    set.insert("1+2+3+4-5*6+7*8+9".into());
    set.insert("1+2+3+4-5*6-7+8*9".into());
    set.insert("1+2+3-4*5+6*7+8+9".into());
    set.insert("1+2+3-4*5-6+7*8+9".into());
    set.insert("1+2+3-4*5-6-7+8*9".into());
    set.insert("1+2+3*4+5+6*7-8-9".into());
    set.insert("1+2+3*4*5+6-7-8-9".into());
    set.insert("1+2+3-45+67+8+9".into());
    set.insert("1+2+3*45-6-78-9".into());
    set.insert("1+2-3+4*5+6*7-8-9".into());
    set.insert("1+2-3-4-5*6+7+8*9".into());
    set.insert("1+2-3*4+5*6+7+8+9".into());
    set.insert("1+2-3*4-5+6*7+8+9".into());
    set.insert("1+2-3*4-5-6+7*8+9".into());
    set.insert("1+2-3*4-5-6-7+8*9".into());
    set.insert("1+2-3*4*5+6+7+89".into());
    set.insert("1+2-3+45+6-7-8+9".into());
    set.insert("1+2-3+45-6+7+8-9".into());
    set.insert("1+2-3-45-6+7+89".into());
    set.insert("1+2*3+4*5-6+7+8+9".into());
    set.insert("1+2*3+4*5*6+7-89".into());
    set.insert("1+2*3-4-5-6*7+89".into());
    set.insert("1+2*3*4+5*6+7-8-9".into());
    set.insert("1+2*3*4-5+6*7-8-9".into());
    set.insert("1+2*3*4*5+6+7-89".into());
    set.insert("1+2-34+5+6+7*8+9".into());
    set.insert("1+2-34+5+6-7+8*9".into());
    set.insert("1+2-34-5-6+78+9".into());
    set.insert("1+2*34-5*6+7+8-9".into());
    set.insert("1-2+3+4-5*6+78-9".into());
    set.insert("1-2+3-4*5-6+78-9".into());
    set.insert("1-2+3*4*5-6-7+8-9".into());
    set.insert("1-2+3+45+6-7+8-9".into());
    set.insert("1-2+3-45+6-7+89".into());
    set.insert("1-2-3+4-5+67-8-9".into());
    set.insert("1-2-3*4+5+6+7*8-9".into());
    set.insert("1-2-3*4-5-6+78-9".into());
    set.insert("1-2-3+45-6-7+8+9".into());
    set.insert("1-2*3+4+5+6*7+8-9".into());
    set.insert("1-2*3+4+5-6+7*8-9".into());
    set.insert("1-2*3+4*5+6+7+8+9".into());
    set.insert("1-2*3+4*5-6*7+8*9".into());
    set.insert("1-2*3+4+56+7-8-9".into());
    set.insert("1-2*3-4+5*6+7+8+9".into());
    set.insert("1-2*3-4-5+6*7+8+9".into());
    set.insert("1-2*3-4-5-6+7*8+9".into());
    set.insert("1-2*3-4-5-6-7+8*9".into());
    set.insert("1-2*3*4+5-6+78-9".into());
    set.insert("1-2*3*4-5-6+7+8*9".into());
    set.insert("1-2*3+45-67+8*9".into());
    set.insert("1-2-34+5+6+78-9".into());
    set.insert("1-2-34-5+6+7+8*9".into());
    set.insert("1-2-34+56+7+8+9".into());
    set.insert("1-2*34+5*6-7+89".into());
    set.insert("1*2+3+4+5*6+7+8-9".into());
    set.insert("1*2+3+4-5+6*7+8-9".into());
    set.insert("1*2+3+4-5-6+7*8-9".into());
    set.insert("1*2+3*4-56+78+9".into());
    set.insert("1*2+3+45+67-8*9".into());
    set.insert("1*2+3-45+6+7+8*9".into());
    set.insert("1*2-3+4-5-6*7+89".into());
    set.insert("1*2-3-4*5+67+8-9".into());
    set.insert("1*2-3-4+56-7-8+9".into());
    set.insert("1*2-3*4+5+67-8-9".into());
    set.insert("1*2*3+4+5+6+7+8+9".into());
    set.insert("1*2*3+4+5-6*7+8*9".into());
    set.insert("1*2*3+4-5*6+7*8+9".into());
    set.insert("1*2*3+4-5*6-7+8*9".into());
    set.insert("1*2*3-4*5+6*7+8+9".into());
    set.insert("1*2*3-4*5-6+7*8+9".into());
    set.insert("1*2*3-4*5-6-7+8*9".into());
    set.insert("1*2*3*4+5+6-7+8+9".into());
    set.insert("1*2*3*4*5-6-78+9".into());
    set.insert("1*2*3-45+67+8+9".into());
    set.insert("1*2+34+5-6-7+8+9".into());
    set.insert("1*2+34-5+6+7-8+9".into());
    set.insert("1*2+34+56-7*8+9".into());
    set.insert("1*2+34-56+7*8+9".into());
    set.insert("1*2+34-56-7+8*9".into());
    set.insert("1*2-34+5*6+7*8-9".into());
    set.insert("1*2*34-5+6-7-8-9".into());
    set.insert("1*2*34+56-7-8*9".into());
    set.insert("1+23+4+5+6+7+8-9".into());
    set.insert("1+23+4-5*6+7*8-9".into());
    set.insert("1+23+4-5-67+89".into());
    set.insert("1+23-4-5+6+7+8+9".into());
    set.insert("1+23-4-5-6*7+8*9".into());
    set.insert("1+23-4*5+6*7+8-9".into());
    set.insert("1+23-4*5-6+7*8-9".into());
    set.insert("1+23*4+5-6-7*8+9".into());
    set.insert("1+23*4-5-6*7+8-9".into());
    set.insert("1+23*4-56+7-8+9".into());
    set.insert("1+23-45+67+8-9".into());
    set.insert("1-23+4-5+67-8+9".into());
    set.insert("1-23+4*5-6*7+89".into());
    set.insert("1-23-4+5+67+8-9".into());
    set.insert("1-23-4-5-6-7+89".into());
    set.insert("1-23*4+5+6*7+89".into());
    set.insert("1-23+45-67+89".into());
    set.insert("1*23+4*5-6+7-8+9".into());
    set.insert("1*23-4-56-7+89".into());
    set.insert("12+3+4-56-7+89".into());
    set.insert("12+3-4*5+67-8-9".into());
    set.insert("12+3*4+5+6-7+8+9".into());
    set.insert("12+3-45+6+78-9".into());
    set.insert("12+3*45-6-7-89".into());
    set.insert("12-3+4*5+6-7+8+9".into());
    set.insert("12-3+4+56-7-8-9".into());
    set.insert("12-3-4+5*6-7+8+9".into());
    set.insert("12-3-4-56+7+89".into());
    set.insert("12-3*4-5+67-8-9".into());
    set.insert("12-3*4*5+6+78+9".into());
    set.insert("12-3-45-6+78+9".into());
    set.insert("12*3+4+5+6-7-8+9".into());
    set.insert("12*3+4+5-6+7+8-9".into());
    set.insert("12*3-4-5-6+7+8+9".into());
    set.insert("12*3-4-56+78-9".into());
    set.insert("12*3*4-5*6-78+9".into());
    set.insert("12+34-5-6-7+8+9".into());
    set
}

fn calc_hash(exp: &String) -> String {
    let mut context = Context::new(&SHA256);
    context.update(exp.as_bytes());
    let digest = context.finish();
    HEXUPPER.encode(digest.as_ref())
}
