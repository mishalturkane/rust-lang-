fn main() {
  let my_str = "hello world";
   println!("Old my_str is :{}",my_str);
   print_string(my_str);
    println!("Old my_str2 is :{}",my_str);

}

fn print_string(string: &str)     {
  println!("print the value in function of string :{}",string);
}
