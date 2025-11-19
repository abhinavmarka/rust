#[derive(Clone, Debug)]
struct Player {
    name: String,
    score: u32,
}

fn main() {
    let p1 = Player {
        name: "Abhi".into(),
        score: 50,
    };

    let p2 = p1.clone();
}
