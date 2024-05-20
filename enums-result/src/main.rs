fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed.".to_string())
    } else {
        Ok(a / b)
    }
}

// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    let a = 10.0;
    let b = 0.0;
    let division_result = divide(a, b);

    match division_result {
        Ok(value) => println!("{} divided by {} is: {}", a, b, value),
        Err(error_message) => println!("Error: {}", error_message),
    }
}