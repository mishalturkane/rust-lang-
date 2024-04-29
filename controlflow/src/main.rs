fn main() {
    println!("This is controlflow demo");

    
    // let marks = 32;

    // if 33 <= marks {
    //     println!("Pass");
    // }else{
    //     println!("Fail");
    // }


    let x = 9;

    if x%2==0 {
        println!("x:{} is divisible by {}",x,2);
    } else if x%3==0 {
        println!("x:{} is divisible by {}",x,3);
    }
}
