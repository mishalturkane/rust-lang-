fn main() {
    let arr = vec![1, 2, 3, 4, 4, 5];
    println!("Array with duplicates removed: {:?}", remove_duplicates(&arr));
}

fn remove_duplicates(arr: &Vec<i32>) -> Vec<i32> {
    let mut unique_elements = Vec::new();

    for &num in arr {
        if !unique_elements.contains(&num) {
            unique_elements.push(num);
        }
    }

    unique_elements
}
