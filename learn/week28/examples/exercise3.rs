fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

fn main() {
    let (x, y) = swap(7, 12);
    println!("x = {}, y = {}", x, y);

    let (s1, s2) = swap("Abhi", "Marka");
    println!("s1 = {}, s2 = {}", s1, s2);
}
