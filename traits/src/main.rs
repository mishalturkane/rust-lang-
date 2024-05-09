trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;
struct Cow;

impl Animal for Cow {
    fn make_sound(&self) {
        println!("Cow says: Moo!");
    }
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Dog says: Woof!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Cat says: Meow!");
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    let cow = Cow;

    dog.make_sound();
    cat.make_sound();
    cow.make_sound();
}
