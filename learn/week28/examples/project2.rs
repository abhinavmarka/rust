enum Transport<T> {
    Train(T),
    Flight(T),
    Bus(T),
}

fn main() {
    let t1 = Transport::Train("navajivan Express");
    let t2 = Transport::Flight("Air India");
    let t3 = Transport::Bus("City Center");

    show_transport(t1);
    show_transport(t2);
    show_transport(t3);
}

fn show_transport<T: std::fmt::Display>(t: Transport<T>) {
    match t {
        Transport::Train(name) => println!("Train: {}", name),
        Transport::Flight(name) => println!("Flight: {}", name),
        Transport::Bus(name) => println!("Bus: {}", name),
    }
}
