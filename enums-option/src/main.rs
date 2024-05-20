fn main() {

    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(value) => println!("The square root of {} is: {}", number, value),
        None => println!("The square root of {} is not a real number.", number),
    }
}

// enum Option<T>{
//     Some(T),
//     None,
// }
fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}