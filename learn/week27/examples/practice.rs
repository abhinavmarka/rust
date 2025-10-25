enum Status {
    Some(u32),
    None,
}

fn main() {
    let statuses = [
        Status::Some(5),
        Status::None,
        Status::Some(10),
    ];

    for status in statuses {
        match status {
            Status::Some(val) => println!("Got a value: {}", val),
            Status::None => println!("No value here"),
        }
    }
}
