fn main() {
    let mut s = "Hello".to_string();
    let t = s;
    s = "World".to_string();
    println!("s : {}, t: {}", s, t)
}
