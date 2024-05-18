
enum Result{
    Success(String),
    Error(i32,String),
}

fn print_result(result: Result){
    match result {
        Result::Success(str) => {
            println!("the error message is :{}", str);
        },
        Result::Error(code, message) => {
            println!("Error {}  :{}", code, message);
        },
    }

}
fn main() {
    println!("enum practice");

    let success_result = Result::Success(String::from("Operation completed"));
    let error_result = Result::Error(404,String::from("Resurse not found".to_string()));
  print_result(success_result);

    print_result(error_result);
}
