use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let name = env::var("MY_NAME").unwrap();
    let language = env::var("MY_LANGUAGE").unwrap();
    let domain = env::var("MY_DOMAIN").unwrap();

    println!("Hello {}, welcome to {}, and my domain is {}!", name, language, domain);
}
