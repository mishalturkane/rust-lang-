// fn main() {
//     println!("Loop Practice");
//     fourth();
// }

// //loop
// //while
// //for 
// fn first(){
//     let mut x = 0;

//     loop{
//         x += 1;
//         println!("x = {}",x);

//         if x == 5 {
//             println!("We did it");
//             break;
//         }
//     }
// }
// fn second(){
//     let  mut number  = 0;

//     while number <=256 {
//         println!("{}",number);
//         number +=1;
//     }
//     println!("Hello");
// }
// fn third(){
//     let a = [10,20,30,40,50,60];

//     let mut index = 0;

//     while index < 6 {
//         println!("the value is:{}",a[index]);
//         index += 1;
//     }
// }
// //for loop 
// fn fourth(){
   
   
// }


fn main(){
    println!("This is main demo'");

    let mut x= 1;

    while x<=10 {
        println!("x :{}",x);
        x += 1;
    }

    let mut y = 11;

    loop{
            println!("y : {}",y);
            y +=10;
            if y>=100{
                break;
            }
    }

    for x in 1..11{
        println!("x :{}",x);
    }
   
}