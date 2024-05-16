fn main (){

    let mut  string = String::from("hello world");

    let length;
    (length,string) = print_string(string);
    println!("length is :{} and string is:{}",length,string);

}
fn print_string(string: String) -> (usize,String){
    (string.len(),string)
}