enum Laptop {
    HP,
    MacBook,
    Dell,
}

fn main() {
    let my_laptop = Laptop::Dell;

    match my_laptop {
        Laptop::HP => println!("HP runs Windows OS."),
        Laptop::MacBook => println!("MacBook runs macOS."),
        _=> println!("Dell runs on linux OS.")
    }
}

