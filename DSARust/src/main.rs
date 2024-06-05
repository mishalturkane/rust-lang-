fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let value = find_number(&vec, 4);
    println!("The index of the value is: {}", value);
}

fn find_number(arr: &Vec<i8>, target: i8) -> i8 {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return index as i8;
        }
    }
    -1 // return -1 if the target is not found
}
