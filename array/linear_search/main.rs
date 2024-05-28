fn main(){
    let arr = vec![1,2,3,4,5];
    println!("element is :{}",linear_search(&arr,22) );
}
fn linear_search(arr: &Vec<i32>,a :i32) -> i32 {

    let element = -1;

    for &i in arr.iter(){
            if a== i{
                return i;
            }
    }
    element

}