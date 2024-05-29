
fn main() {
    let arr = vec![1, 2, 3, 4,  5];
    println!("Are there any duplicate elements in this array? {}", check_for_duplicates(&arr));
}

fn check_for_duplicates(arr: &Vec<i32>) -> bool {
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[i] == arr[j] {
                return true;
            }
        }
    }
    false
}
