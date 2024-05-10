fn main() {
   let x =10;
   let y =20;
   let sum=first(x, y);
   println!("x:{} y:{} sum:{}",x,y,sum);
   second(x+10,y+20);
   println!("sum :{}",third());
   fourth();
}
fn first( a:i32,b:i32)-> i32{
     let x = a;
     let y = b;
     return x+y;
}
fn second(a:i32,b:i32){
    let x = a;
    let y = b;
    println!("x:{} y:{} sum:{} ",x,y,(x+y));
}
fn third() -> i32{
    let x =10;
    let y =20;
    println!("x:{} y:{}",x,y);
        return x+y;
}
fn fourth(){
    let x =100;
    let y =200;
    println!("x:{} y:{} sum:{}",x,y,(x+y));
}