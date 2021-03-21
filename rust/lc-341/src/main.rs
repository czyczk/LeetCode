mod solution;

use solution::NestedInteger;
use solution::NestedIterator;

fn main() {
    let list1 = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];

    let list2 = vec![
        NestedInteger::Int(1),
        NestedInteger::List(vec![
            NestedInteger::Int(4),
            NestedInteger::List(vec![NestedInteger::Int(6)]),
        ]),
    ];

    assert_eq!(
        vec![1, 1, 2, 1, 1],
        call_until_false(NestedIterator::new(list1))
    );

    assert_eq!(vec![1, 4, 6], call_until_false(NestedIterator::new(list2)));
}

fn call_until_false(mut ni: NestedIterator) -> Vec<i32> {
    let mut res = vec![];

    while ni.has_next() {
        res.push(ni.next());
    }

    res
}
