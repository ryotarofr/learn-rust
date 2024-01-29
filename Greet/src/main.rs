use std::collections::HashMap;

fn greet_map(id: i32, name: &str) -> HashMap<i32, &str> {
    let mut m: HashMap<i32, &str> = HashMap::new();

    // let message = format!("Hello, {}", name);
    m.insert(id, name);
    m
}

fn main() {
    let name: String = String::from("hello");
    let res: HashMap<i32, &str> = greet_map(1234, &name);
    println!("{:?}", res)
}
