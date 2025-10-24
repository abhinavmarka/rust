struct Person {
    name: String,
    age: u32,
    weight: f32,
    height: f32,
}

impl Person {
    fn bmi(&self) -> f32 {
        self.weight / (self.height * self.height)
    }
}

fn main() {
let charan1 = Person {
    name: String::from("charan"),
    age: 21,
    weight: 92.0,
    height: 1.95
};
println!("name: {}", charan1.name);
println!("age: {}", charan1.age);
println!("bmi of charan is {}", charan1.bmi());

if charan1.bmi() <= 18.5 {
    println!("underweight");
} else if charan1.bmi() <= 24.99 {
    println!("normal weight");
} else if charan1.bmi() <= 29.00 {
    println!("over weight");
} else {
    println!("obese");
}

}