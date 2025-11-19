#[derive(Debug)]
enum Role {
    Admin,
    User,
}

use std::fmt;

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "Administrator"),
            Role::User => write!(f, "Regular User"),
        }
    }
}

fn main() {
    let role = Role::Admin;
    println!("{}", role);
}