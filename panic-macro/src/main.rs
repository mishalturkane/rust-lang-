fn main() {
   let veggies = ["carrot","celery","tomato"];

    // chop(veggies[0]);
    // chop(veggies[1]);
    // chop(veggies[2]);
    // chop("KUCH NHI HAIN KATNE KO");




}
fn chop(vegetable: &str) {
    match vegetable {
        "carrot" => println!("Chopping a carrot."),
        "celery" => println!("Chopping a celery."),
        "tomato" => panic!("Don't know how to handle a tomato!"),
        _ => println!("Chopping some unknown vegetable."),
    }
}