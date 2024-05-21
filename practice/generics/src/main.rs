// fn main() {
//
//     let int_container = Container{ value: 20};
//     println!("value is :{}",int_container.value);
//
//     let char_container = Container{ value: 'c'};
//     println!("valus is :{}",char_container.value);
//
//     let success_result: MagicalResult<i32, String> = MagicalResult::Success(42);
//     let failure_result: MagicalResult<i32, String> = MagicalResult::Failure("Oops, something went wrong!".to_string());
//
//     match success_result {
//         MagicalResult::Success(value) => println!("We've got the answer: {}", value),
//         MagicalResult::Failure(error) => println!("Error: {}", error),
//     }
//
//     match failure_result {
//         MagicalResult::Success(value) => println!("We've got the answer: {}", value),
//         MagicalResult::Failure(error) => println!("Error: {}", error),
//     }
// }
// struct  Container<T>{
//     value : T,
// }
//
// enum MagicalResult<T,E> {
//     Success(T),
//     Failure(E),
// }

fn main(){


    let int_str_pair = FancyPair {
        first: 42,
        second: "The answer to life, the universe, and everything",
    };
    println!("{}",int_str_pair.first);
    println!("{}",int_str_pair.second);
}

struct FancyPair<T, U> {
    first: T,
    second: U,
}