
fn find_max(arr: &mut Vec<i32>) -> i32{
 
    let mut  max: i32 = i32::min_value();

    
    for &num in arr.iter() {
        if num > max {
            max = num;
        }
    }

   return  max;

}
fn main() {
    let mut arr = vec![2,343,3,34];
    println!("max element is:{}",find_max(&mut arr));
 }