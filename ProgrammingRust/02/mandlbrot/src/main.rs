use num::Complex;

fn main() {
    println!("Hello, world!");
    let c = Complex::new(0.340231, 0.3848);
    let limit = 1000;
    let result = escape_time(c, limit);
    println!("{:?}", result);
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}
