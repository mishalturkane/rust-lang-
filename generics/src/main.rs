use std::future::poll_fn;

fn largest_i32 <T: std::cmp::PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    //
    // let result = largest_i32(&char_list);
    //
    // println!("The largest char is {}",result);


    let proint = Point {x:5,y:10};
}

struct  Point<T> {
    x:T,
    y:T,
}