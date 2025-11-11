#[inline(always)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(unused_variables)]
fn main() {
    let p = Point { x: 2, y: 3 };
    let s = add(p.x, p.y);
    println!("attributes: {:?}, sum = {}", p, s);
}


