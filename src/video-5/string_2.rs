/// module to explaing ascii uppercase edge case in Rust

fn naive_capitalise(s: &str) -> String{
    // iterate the characters and capitalise using `to_uppercase()`
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

fn ascii_capitalise(s: &str) -> String {
    // iterate the characters and capitalise using `to_ascii_uppercase()`
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_ascii_uppercase().to_string() + c.as_str()
    }
}

fn main(){
    let names: Vec<String> = vec![
        "alice".to_string(),
        "bob".to_string(),
        "özgül".to_string(),
        "ßad".to_string()
    ];

    for name in names.iter() {
        println!("{} -> {}", name, naive_capitalise(name));
        println!("{} -> {}", name, ascii_capitalise(name));
        println!("---");
    }
}