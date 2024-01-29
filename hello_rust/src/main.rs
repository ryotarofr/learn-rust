// 変数、整数、シャドーイング、型
fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x); //5
    x = 6;
    println!("The value of x is: {}", x); //6
    let y: i32 = 6;
    {
        let y: i32 = y + 1;
        println!("The value of x is: {}", y); //7
    }
    println!("The value of x is: {}", y); //6

    let some_strings: &str = "aaa";
    println!("The value of: {}", some_strings);
    let some_strings: usize = some_strings.len();
    println!("The value of: {}", some_strings);
}
