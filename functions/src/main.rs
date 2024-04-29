//this is code adding two numbers

fn main() {
    println!("This is the function main!");
    println!("first func: {}",t_s_r_s(10, 20));
    t_s_r_n(11,22 );
    println!("third func: {}",t_n_r_s());
    t_n_r_n();
}

//take something return something 
fn t_s_r_s(a:i32,b:i32) ->i32{
        return a+b;
}

//take something return nothing
fn t_s_r_n(a:i32,b:i32){
        println!("second func: {},{}",a,b);
}
//take nothing return something
fn t_n_r_s() ->i32{
    let a=100;
    let b= 200;
    return a+b;
}
//take nothing return nothing
fn t_n_r_n(){
    let a=1000;
    let b= 2000;
    println!("fourth func: {},{}",a,b);
}