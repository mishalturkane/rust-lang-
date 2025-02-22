fn short_username(name: &str, length: i8) -> String {
    if name.len() > length as usize {
        for (i, ch) in name.chars().enumerate() {
            if ch == ' ' || i >= length as usize {
                return name[..i].to_string();
            }
        }
    }
    name.to_string()
}

fn main() {
    let name = "John Doe";
    let shorted = short_username(name, 5);
    println!("{}", shorted);  // Output: "John"
}
