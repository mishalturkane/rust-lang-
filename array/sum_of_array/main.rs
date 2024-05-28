fn main(){
    let arr = vec![10,20,30,40,50];
    println!("sum of the array is:{}",sum_of_array(arr));
}

fn sum_of_array(arr: Vec<i32>) -> i32{

        let mut sum = 0;

        for &i in arr.iter(){
           sum += i;
        }
        sum

}