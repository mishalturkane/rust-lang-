use crate::error::my_function;

mod error;

fn main() {

    // let result = divide(5.0, 9.0);
    // println!("{:?}",result);

    my_function();
}

fn divide(numerator: f64, dinominator: f64) -> Option<f64> {
    if dinominator == 0.0{
        None
    }
    else {
        Some(numerator/dinominator)
    }
}

