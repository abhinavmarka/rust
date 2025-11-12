#[derive(Debug, Clone)]
struct User {
    name: String,
    id: u32,
}

#[derive(Debug)]
enum Role {
    Admin,
    User,
}

fn main() {
    let user1 = User {
        name: String::from("chris"),
        id: 101,
    };
    
    let user2 = user1.clone();
    let role = Role::Admin;
    
    println!("User 1: {:?}", user1);
    println!("User 2: {:?}", user2);
    println!("Role: {:?}", role);
}

