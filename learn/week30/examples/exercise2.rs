use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
struct Person {
    username: String,
    password: String,
}

fn main() {
    let p: Person = Person { username: String::from("abhi"), password: String::from("123456") };
    let serialized_bytes: Vec<u8> = p.try_to_vec().unwrap();
    println!("{:?}", serialized_bytes);