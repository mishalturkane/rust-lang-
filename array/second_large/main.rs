fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    println!("Second largest element: {}", second_largest(&arr));
}

fn second_largest(arr: &Vec<i32>) -> i32 {
    if arr.len() < 2 {
        return -1; // Return a default value indicating that there is no second largest element
    }

    let mut max = arr[0];
    let mut sec_max = i32::MIN;

    for &num in arr.iter().skip(1) {
        if num > max {
            sec_max = max;
            max = num;
        } else if num > sec_max && num < max {
            sec_max = num;
        }
    }

    sec_max
}
