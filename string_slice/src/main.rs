fn main(){
  let numbers = [1,2,3,4,5];

  let slice = &numbers[0..5];

  println!("slice first element is {}",slice[0]);

}