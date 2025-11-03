#[inline(always)]
fn add(a: i32, b: i32) -> i32 { a + b }

#[derive(Debug)]
struct Point { x: i32, y: i32 }

#[allow(unused_variables)]
fn main() {
    let x = 42; // allowed by attribute
    let p = Point { x: 1, y: 2 };
    let sum = add(x, p.x);
    println!("week29: attributes demo");
    println!("{:?}, sum={}", p, sum);
}