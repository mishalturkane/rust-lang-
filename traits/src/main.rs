use std::io;

fn main(){

    //
    // let mut  input = String::new();
    //
    // println!("Please enter a radius:");
    //
    // io::stdin().read_line(&mut input).expect("failed to read line");
    //
    // let radius: f64  = input.trim().parse().expect("failed to read line");
    //
    // let circle= Circle{
    //     radius
    // };
    // circle.display();
    //
    //
    // input.clear();
    // println!("Please enter a heigth:");
    //
    // io::stdin().read_line(&mut input).expect("failed to read line");
    //
    // let heigth: f64  = input.trim().parse().expect("failed to read line");
    //
    // input.clear();
    // println!("Please enter a heigth:");
    //
    // io::stdin().read_line(&mut input).expect("failed to read line");
    //
    // let width: f64  = input.trim().parse().expect("failed to read line");
    //
    // let rectangle = Rectangle{
    //     heigth,
    //     width
    // };
    //
    // rectangle.display();



    let person = Person{
        name:String::from("Mishal"),
        age: 22
    };

    println!("the person is :{:?}",person.describe());
}

trait  Display{
    fn display(&self);
}
struct  Circle {
    radius: f64,
}

struct  Rectangle{
    heigth: f64,
    width: f64,
}

impl Display for Rectangle{
    fn display(&self) {
        println!("area of rectangle is:{}",0.5*self.heigth*self.width);
    }
}
impl Display for Circle{
    fn display(&self)   {
       println!("area of circle is :{}", 3.14 * (self.radius * self.radius));
    }
}


trait Describable {
    fn describe(&self) -> String ;
}

struct Person {
    name:String,
    age: u8 ,
}

impl  Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }
}