fn main() {
    // Ownership example
    let s1 = String::from("Hello, Rust!"); // s1 owns the String
    let s2 = s1; // Ownership is moved from s1 to s2

    // println!("{}", s1); // Uncommenting this will cause an error because s1 no longer owns the value
    println!("Ownership example: {}", s2); // Ownership now belongs to s2

    // Borrowing examples
    let s3 = String::from("Hello, Borrowing!");

    // Immutable Borrowing
    let len = calculate_length(&s3); // Borrow s3 immutably
    println!("Immutable Borrowing: The length of '{}' is {}.", s3, len); // s3 is still valid here

    // Mutable Borrowing
    let mut s4 = String::from("Hello");
    append_world(&mut s4); // Borrow s4 mutably
    println!("Mutable Borrowing: {}", s4); // s4 is modified
}

fn calculate_length(s: &String) -> usize {
    s.len() // Accessing the borrowed value
}

fn append_world(s: &mut String) {
    s.push_str(", World!"); // Modifying the borrowed value
}
