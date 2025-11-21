fn main() {
    let str1: String = String::from("Abhi");
    let ans: &String;
    {
        let str2: String = String::from("");
        ans = longest_string(s1:&str1, s2:&str2);
        println!("{}", ans);
    }
    
}

fn longest_string<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    } 
    
    return s2;
}