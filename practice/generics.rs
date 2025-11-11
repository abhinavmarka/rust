#[derive(Debug)]
struct Transport<T> {
    vehicle: T,
}

impl<T> Transport<T> {
    fn new(vehicle: T) -> Self {
        Transport { vehicle }
    }

    fn show(&self)
    where
        T: std::fmt::Debug,
    {
        println!("generics: {:?}", self.vehicle);
    }
}

#[derive(Debug)]
struct Train;

fn main() {
    let t = Transport::new(Train);
    t.show();
}


