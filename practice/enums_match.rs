enum Laptop {
    HP,
    MacBook,
    Dell,
}

fn main() {
    let mine = Laptop::Dell;
    let os = match mine {
        Laptop::HP => "Windows",
        Laptop::MacBook => "macOS",
        Laptop::Dell => "Linux",
    };
    println!("enums+match: {}", os);
}


