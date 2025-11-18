#[route(GET)]
fn get_user() {
    println!("Getting user");
}

#[route(POST)]
fn create_user() {
    println!("Creating user");
}

#[route(PUT)]
fn update_user() {
    println!("Updating user");
}

#[derive(Sql("users"))]

struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    fn new(id: u32, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

fn main() {
    let user = User::new(1, "Mark".to_string(), "mark.in@mail.com".to_string());
    println!("User: {:?}", user);
}