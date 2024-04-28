
fn main() {
  
  println!("Advance enums practice");

  let person1 = Person{
    name:String::from("Mishal Turkane"),
    email:String::from("mishalturkane@gmail.com"),
    age: 20,
    gender:GenderCat::Male

  };

  println!("{:?}",person1);
}



#[derive(Debug)]
enum  GenderCat{
    Male,Female,TransGender
}  

#[derive(Debug)]
struct Person{
    name: String,
    email: String,
    age: i32,
    gender: GenderCat
}