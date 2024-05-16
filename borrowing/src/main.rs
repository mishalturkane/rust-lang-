fn main(){


    let mut  my_string = String::from("hello");


    let r1 = &my_string;

    println!("{} ",r1);

    let mut_ref = &mut my_string;

    println!("{}",mut_ref);

}