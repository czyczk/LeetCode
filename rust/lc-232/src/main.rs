mod solution;

use solution::MyQueue;

fn main() {
    let mut mq = MyQueue::new();
    mq.push(1);
    mq.push(2);
    mq.push(3);
    mq.push(4);
    // Expecting 1
    println!("{}", mq.peek());
    // Expecting 1
    println!("{}", mq.pop());
    mq.push(5);
    // Expecting 2
    println!("{}", mq.pop());
    // Expecting 3
    println!("{}", mq.pop());
    // Expecting 4
    println!("{}", mq.pop());
    // Expecting false
    println!("{}", mq.empty());
    // Expecting 5
    println!("{}", mq.pop());
    // Expecting true
    println!("{}", mq.empty());
}
