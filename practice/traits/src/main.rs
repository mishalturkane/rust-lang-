fn main() {
    let book = Book{
        title:String::from("Let's  get rusty"),
        author:String::from("Mishal"),
    };
    book.print_info();

    let pen = Pen{
        color: String::from("red"),
        brand: String::from("cello"),
    };
    pen.print_info();
}

trait PrintTable{
   fn  print_info(&self);
}
struct  Book{
    title:String,
    author:String,
}
struct  Pen {
    color: String,
    brand: String,
}

impl PrintTable for Book{
    fn print_info(&self) {
        println!("author is {}, title is {}",self.author,self.title);
    }
}

impl PrintTable for Pen{
    fn print_info(&self) {
        println!("color is {}, brand is {}",self.color,self.brand);
    }
}

