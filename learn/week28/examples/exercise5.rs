stuct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height
    }
}

fn main() {
    let r:Rect = Rect {
        width = 10,
        height = 10
    }
    println!("{}", r.area());
}
