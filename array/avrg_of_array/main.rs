fn main() {
    let arr = vec![10.0, 20.0, 30.0, 40.0, 50.0, 1212.0, 133.0, 10.0, 0.0, 12.99];
    println!("Average of the array is: {}", avrg_of_array(&arr));
}

fn avrg_of_array(arr: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    let ele = arr.len();

    for &i in arr.iter() {
        sum += i;
    }
    sum / ele as f64
}
