fn main() {
    let book = Book {
        title: String::from("The Rusty Boy"),
        author: String::from("Mishal"),
        publication_year: 2029,

    };


    println!("book is {} writen by {} in {}", book.title, book.author, book.publication_year);
    let mut book = Book {
        title: String::from("The Rusty Boy"),
        author: String::from("Mishal"),
        publication_year: 2029,

    };
    book.publication_year = 2025;
    println!("book is {} writen by {} in {}", book.title, book.author, book.publication_year);

    let book_data = get_book_data(book);
    for data in book_data {
        println!("{data}");
    }

    let my_book = create_book("Let get rusty".to_string(),"chacha".to_string(),2024);
    println!("and my book is {:?}",my_book);


    let tuple_book = Tuple_book("crack the coding interview".to_string(),"mama ne likha hain".to_string(),2025);

    println!("the book name is:{}",tuple_book.0);
    println!("the author:{}",tuple_book.1);
    println!("the publication year:{}",tuple_book.2);

    let my_rectangle = Rectangle{
        width : 20,
        height: 20,
    };

    println!("width is :{} heigth is :{}",my_rectangle.width,my_rectangle.height);

    let area = my_rectangle.area();
    println!("are of the rectangle is:{}",area);

    let mishal = CyberSecurity{
        name: String::from("Mishal"),
        enrollment_number: String::from("0126cy211034"),
        cgpa: 9.87,
    };

    println!("Mishal is:{:?}",mishal);

    let pooja = CyberSecurity{
        name: String::from("Pooja"),
        enrollment_number: String::from("0126cy211051"),
        cgpa: 98.9,
    };
    println!("pooja is:{:?}",pooja);

}


#[derive(Debug)]
struct Book{
    title: String,
    author: String,
    publication_year: u32,
}

struct Tuple_book  (String,String,u32);
fn get_book_data(book: Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];
    return data;
}
fn create_book(title: String , author: String , publication_year : u32 ) -> Book{
        let book = Book{
            title,
            author,
            publication_year,
        };
    return book;
}

struct  Rectangle {
    width: u32 ,
    height: u32 ,
}
impl Rectangle {
    fn area(&self) -> u32{
        return &self.width * &self.height;
    }
}


#[derive(Debug)]
struct  CyberSecurity{
    name: String,
    enrollment_number: String,
    cgpa: f32,
}

