// Define a Stack struct
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // Create a new stack
    fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    // Push an element onto the stack
    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // Pop an element from the stack (if available)
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Peek at the top element without removing it
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // Get the size of the stack
    fn size(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();

    // Push elements onto the stack
    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Stack size: {}", stack.size());
    println!("Top element: {:?}", stack.peek());

    // Pop elements off the stack
    while !stack.is_empty() {
        println!("Popped: {:?}", stack.pop());
    }

    println!("Stack is empty: {}", stack.is_empty());
}
