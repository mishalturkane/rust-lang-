fn main(){
  let my_string1 = String::from("hello");
  let my_string2 = String::from("word");

  let concatenated_string = concatenate_string(&my_string1, &my_string2);
println!("The concatenated string is:{}",concatenated_string);
}
fn concatenate_string(s1: &str , s2: &str) -> String{
 let mut  result = String::from(s1);
  result.push_str(s2);
  result
}