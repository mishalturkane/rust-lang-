// main.rs
mod stack;
use stack::Stack;

fn main() {
    let mut stack = Stack::new();

    // Push elements onto the stack
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Peek at the top element
    if let Some(top) = stack.peek() {
        println!("Top element is: {}", top);
    } else {
        println!("The stack is empty.");
    }

    // Pop elements off the stack
    while let Some(value) = stack.pop() {
        println!("Popped value: {}", value);
    }

    // Check if the stack is empty
    if stack.is_empty() {
        println!("The stack is now empty.");
    }
}
