
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    ($name:expr, $greeting:expr) => {
        println!("{}, {}!", $greeting, $name);
    };
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
    ($a:expr, $b:expr, $c:expr) => {
        $a + $b + $c
    };
}

fn main() {
    greet!("chris");
    greet!("john", "Hi");
    

    let sum1 = add!(5, 10);
    let sum2 = add!(1, 2, 3);
    println!("5 + 10 = {}", sum1);
    println!("1 + 2 + 3 = {}", sum2);
    

    greet!("Rust");
}

