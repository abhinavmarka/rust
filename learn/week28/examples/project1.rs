#[derive(Debug)]
struct Train;
#[derive(Debug)]
struct Flight;
#[derive(Debug)]
struct Bus;

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
        println!("Vehicle: {:?}", self.vehicle);
    }
}

fn main() {
    let t1 = Transport::new(Train);
    let t2 = Transport::new(Flight);
    let t3 = Transport::new(Bus);

    t1.show();
    t2.show();
    t3.show();
}
