#[derive(Debug)] // Add Debug trait implementation
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)] // Add Debug trait implementation
enum Fruit {
    Apple,
    Banana,
    Orange,
}

fn main() {
    // Primitive Types
    let num: i32 = 42;
    let num_u64: u64 = 1234567890;
    let float_num: f64 = 3.14;
    let float_num_f32: f32 = 2.71828;
    let is_true: bool = true;
    let is_false = false;
    let char_value: char = 'A';

    println!("Primitive Types:");
    println!("Integer: {}", num);
    println!("Unsigned Integer: {}", num_u64);
    println!("Float (f64): {}", float_num);
    println!("Float (f32): {}", float_num_f32);
    println!("Boolean (true): {}", is_true);
    println!("Boolean (false): {}", is_false);
    println!("Character: {}", char_value);

    // Compound Types
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let tuple: (i32, char, f64) = (42, 'A', 3.14);

    println!("\nCompound Types:");
    println!("Array: {:?}", arr);
    println!("Tuple: {:?}", tuple);

    // User-defined Types
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let fruit_choice = Fruit::Apple;

    println!("\nUser-defined Types:");
    println!("Person: {:?}", person1);
    println!("Fruit: {:?}", fruit_choice);

    // References and Pointers
    let x = 10;
    let y = &x;
    let mut z = 5;
    let z_ref = &mut z;

    println!("\nReferences and Pointers:");
    println!("Immutable Reference: {}", y);
    println!("Mutable Reference: {:?}", z_ref);

    // Slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    let s = String::from("hello");
    let slice_str = &s[0..2];

    println!("\nSlices:");
    println!("Array Slice: {:?}", slice);
    println!("String Slice: {:?}", slice_str);
}
