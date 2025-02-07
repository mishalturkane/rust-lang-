fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![3, 7, 2, 9, 5];
    let result = largest(&numbers);
    println!("Largest number: {}", result);

    let chars = vec!['a', 'y', 'k', 'm'];
    let result = largest(&chars);
    println!("Largest char: {}", result);
}
