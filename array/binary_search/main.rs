fn main() {
    let arr = vec![10, 20, 30, 40, 50];
    println!("{}", binary_search(&arr, 20));
}

fn binary_search(arr: &Vec<i32>, a: i32) -> i32 {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        if arr[mid] == a {
            return mid as i32;
        } else if arr[mid] < a {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    -1
}
