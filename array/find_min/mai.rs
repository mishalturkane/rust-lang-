fn find_min(arr: &mut Vec<i32>) -> i32{

    let mut  min: i32 = i32::max_value();

    
    for &num in arr.iter() {
        if num < min {
            min = num;
        }
    }

   return  min;

}

fn main() {
    let mut arr = vec![1,2,3,2,6,8,0];
     println!("max element is:{}",find_min(&mut arr));
 }
 
 