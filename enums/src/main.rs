enum Cartypes {
    Hatchback,
    Sedan,
    SUV,
    MUV
}
fn  print_cars(car:Cartypes){
   match car{
    Cartypes::Hatchback =>{
        println!("Small car in  a segment");
    }

    Cartypes::Sedan =>{
        println!("Luxury car in a segment");
    }

    Cartypes::SUV =>{
        println!("Sport  utility bassed vehicle");
    }
    Cartypes::MUV =>{
        println!(" Multi  utility bassed vehicle");
    }
   }

}

fn main() {
    println!("Enums Demo");
    print_cars(Cartypes::Sedan);
}
