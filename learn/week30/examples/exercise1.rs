use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    username: String,
    password: String,
}

fn main() {
    let u: User = User { 
        username: String::from("abhi"),
        password: String::from("123456"),
    };

    let serialized_string: Result<String, serde_json::Error> = serde_json::to_string(&u);
    match serialized_string {
        Ok(str: String) => println!("{}", str),
        Err(e) => println!("Error while converting to string: {}")
    }
}