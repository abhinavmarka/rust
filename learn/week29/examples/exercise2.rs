#[must_use]
fn square(n: i32) -> i32 { n * n }
#[deprecated(note = "use square instead")]
fn old_square(n: i32) -> i32 { n * n }
#[allow(deprecated)]
fn main() {
    let s = square(5);
    let _ = old_square(3);
    println!("square(5) = {}", s);
}
