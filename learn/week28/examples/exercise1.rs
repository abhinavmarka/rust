use chrono::prelude::*;

fn main() {

let utc = Utc::now();
let local = Local::now();


println!("time:{}", local);
println!("time:{}", utc);
}