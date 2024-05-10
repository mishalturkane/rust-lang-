fn main() {
   //first();
   //second();
   third();
}
fn first(){ 
        for i in 0..5 {
        
        for j in 0..=i{
            print!("* ");
        }
       println!();
    
        }   
}
fn second(){
   
        for i in 0..5 {
            for _ in i..4 {
                print!(" ");
            }
            for _ in 0..=i {
                print!("*");
            }
            println!();
        }
    
    
}
fn third() {
    let rows = 5;
    for i in 0..rows {
        for _ in 0..(rows - i - 1) {
            print!(" ");
        }
        for _ in 0..( 2*i + 1) {
            print!("*");
        }
        println!();
    }
}





