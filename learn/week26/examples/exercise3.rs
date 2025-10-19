fn main() {
    let x: u32 = 25;

    if x > 24 {
      for i in 0..3{
        println!("your are eligible to get more than 1 cr");
      }
    } else if x > 20{
        println!("your are eligible to get more than 50 lakhs");
    } else if x > 15{
        println!("your are eligible to get more than 25 lakhs");
    } else {
        println!("your are eligible to get more than 10 lakhs")
    }
}