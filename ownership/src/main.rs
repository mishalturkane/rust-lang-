fn main() {
    
    let mut s1 = String::from("Hello");
    
    let len ;
    (len,s1) = calculate_length(s1);

    println!("the length of {} is {}",s1,len );


}
fn calculate_length(s: String) -> (usize,String) {
    (s.len(),s)

}