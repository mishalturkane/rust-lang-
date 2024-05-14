fn main() {
    let mut variable = 10; // Define a mutable variable

    let immutable_reference = &variable; // Immutable reference
    println!("Value of variable using immutable reference: {}", immutable_reference);

    // Uncommenting the line below will result in a compilation error because you cannot have a mutable reference while an immutable reference exists.
     let mutable_reference = &mut variable;

    // Attempting to modify variable using an immutable reference will result in a compilation error.
    // Uncommenting the line below will result in a compilation error.
    // *immutable_reference = 20;
}
