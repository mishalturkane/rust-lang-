fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![10, 25, 56, 7, 89];
    let largest_number = find_largest(&numbers);
    println!("The largest number is: {}", largest_number);

    let floats = vec![2.5, 7.8, 1.3, 9.9, 4.4];
    let largest_float = find_largest(&floats);
    println!("The largest float is: {}", largest_float);
}
