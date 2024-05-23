use std::io;
use std::num::ParseIntError;


fn main() {

    // let result = divide(5.0, 0.0);
    // println!("{:?}",result);

   let result = read_number();
    println!("{:?}",result);

}

fn divide(numerator: f64, dinominator: f64) -> Result<f64 , &'static str> {
    if dinominator == 0.0{
       return Err("cant divide by 0");
    }
        Ok(numerator/dinominator)
}

fn read_number() -> Result<i32, io::Error> {
    let mut input = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut input)?;

    // Attempt to parse the input string into an integer
    let number: i32 = match input.trim().parse() {
        Ok(parsed) => parsed,
        Err(parse_error) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Failed to parse input: {}", parse_error),
            ));
        }
    };
    Ok(number)
}