// Define a function that takes a reference to an integer
fn print_number(num: &i32) {
    println!("The number is: {}", num);
}

fn main() {
    let number = 42; // Define an integer variable

    // Call the function and pass a reference to the integer variable
    print_number(&number);

    // Attempting to use 'number' after passing a reference is fine, because borrowing does not take ownership
    println!("The number after borrowing: {}", number);
}
