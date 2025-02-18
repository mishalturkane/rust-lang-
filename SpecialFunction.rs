fn return_two_types<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    let result = return_two_types(42, "Hello");
    println!("First: {}, Second: {}", result.0, result.1);
}
