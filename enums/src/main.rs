fn main(){

    let msg = Message::Write("Hello Rust".to_string());
   process_massage(msg);
}


enum Message{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    changeColor(i32,i32,i32),
}
fn process_massage(msg:Message){
    match msg{
        Message::Quit =>{
            println!("Please quit the chat");
        }
        Message::Move{y,x} => {
            println!("i am moving from x: {} to y:{}",x,y);
        }
        Message::Write(msg) =>{
            println!("Message is:{}",msg);
        }
        Message::changeColor(r,g,b) => {
            println!("colors are : r {} g {} b{}",r,g,b);
        }

    }

}