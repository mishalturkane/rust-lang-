fn main() {
    let mut arr = vec![1,2,3,2,6,8,0];
    println!("reverse array is:{:?}",reverse_array(&mut arr));
}
 fn reverse_array(arr: &mut Vec<i32>) -> &Vec<i32> {
 
         let mut i = 0;
         let mut j = arr.len()-1;
 
         while i < j{
             // let mut temp = arr[i];
             // arr[i] = arr[j];
             // arr[j] = temp;
             arr.swap(i,j);
             i +=1;
             j -= 1;
         }
         return arr;
 }
 