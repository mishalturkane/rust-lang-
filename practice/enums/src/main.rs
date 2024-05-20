
// enum Result{
//     Success(String),
//     Error(i32,String),
// }
//
// fn print_result(result: Result){
//     match result {
//         Result::Success(str) => {
//             println!("the error message is :{}", str);
//         },
//         Result::Error(code, message) => {
//             println!("Error {}  :{}", code, message);
//         },
//     }
//
// }
// fn main() {
//     println!("enum practice");
//
//     let success_result = Result::Success(String::from("Operation completed"));
//     let error_result = Result::Error(404,String::from("Resurse not found".to_string()));
//   print_result(success_result);
//
//     print_result(error_result);
// }


enum TrafificLight {
    red,
    green,
    yellow,
}
fn color(light : TrafificLight) {
    match light {
        TrafificLight::red =>{
            println!("Stop");
         },
         TrafificLight::green => {
        println!("Go");
         },
         TrafificLight::yellow => {
        println!("Wait");
         },
    }
}

fn main() {
    let color1 = TrafificLight::red;
    let color2 = TrafificLight::green;
    let color3 = TrafificLight::yellow;
    color(color1);
    color(color2);
    color(color3);
}