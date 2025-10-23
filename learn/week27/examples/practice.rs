fn main() {
    let mut s1: String = String::from("Abhi");
    
    let s2: &mut String = &mut s1;
    s2.push_str("marka");

    let s3: &String = &s1;
    let s4: &String = &s1;
    println!("{} {}", s3, s4);

}