
fn main() {
    let  arr = vec![10,20,30,40,50];
    print_arr(&arr);
}

fn print_arr(arr: &Vec<i32>){

    println!("{:?}",arr );
}