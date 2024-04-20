use std::env;
use std::process;
use std::str::FromStr;
fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(i64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: sum <number>...");
        process::exit(1);
    }

    let mut d = numbers[0];
    for n in &numbers[1..] {
        d = gcd(d, *n);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
