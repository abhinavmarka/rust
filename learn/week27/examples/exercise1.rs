fn main() {
    let s1 = String::from("Harkirat");
    let (len, s1) = get_length(s1);
    println!("Length 1 = {}", len);
    let (len2, _s1) = get_length(s1);
    println!("Length 2 = {}", len2);
}

fn get_length(s2: String) -> (usize, String) {
    (s2.len(), s2)
}

