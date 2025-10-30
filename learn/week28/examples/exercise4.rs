use std::fmt::Debug;


fn swap_and_print<T: Debug>(a: T, b: T) -> (T, T) {
    println!("Before swap: a = {:?}, b = {:?}", a, b);
    let result = (b, a);
    println!("After swap:  a = {:?}, b = {:?}", result.0, result.1);
    result
}

fn main() {
    let (x, y) = swap_and_print(10, 20);
    let (s1, s2) = swap_and_print("Hello", "Rust");
}
