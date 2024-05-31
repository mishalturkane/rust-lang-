// stack.rs
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // Create a new, empty stack
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // Push an element onto the stack
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // Pop an element off the stack
    // Returns Option<T>, where Some(T) is the element, or None if the stack is empty
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Peek at the top element of the stack without removing it
    // Returns Option<&T>, where Some(&T) is a reference to the element, or None if the stack is empty
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // Get the number of elements in the stack
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}
