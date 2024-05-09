use std::fmt::Display;

fn print_value<T: Display>(value: T) {
    println!("Value: {}", value);
}

fn main() {
    print_value(5);             // integer
    print_value("Hello, Rust!"); // string
    print_value(3.14);          // float
}
