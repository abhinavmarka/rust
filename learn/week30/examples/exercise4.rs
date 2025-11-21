struct User<'a> {
    name: &'a str,
    
}

impl<'a> User<'a> {
    fn new(name: &'a str) -> User<'a> {
        User { name }
    }
}

fn main() {
    let name = String::from("Abhi");
    let user = User::new(&name);
    println!("{}", user.name);
}