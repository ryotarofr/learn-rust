use num::Complex;
use std::str::FromStr;

fn main() {
    let input_str = "20,20";
    let separator = ',';
    let result = parse_pair::<i32>(input_str, separator);
    println!("{:?}", result);
    let result = pixel_to_point(
        (100, 100),
        (25, 75),
        Complex { re: -1.0, im: 1.0 },
        Complex { re: 1.0, im: -1.0 },
    );
    println!("{:?}", result);

    let result = render(
        &mut [0; 10000],
        (100, 100),
        Complex { re: -1.0, im: 1.0 },
        Complex { re: 1.0, im: -1.0 },
    );
    println!("{:?}", result);

    let result = write_image("output.png", &mut [0; 10000], (100, 100));
    println!("{:?}", result);
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn parse_pair_test() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);
    for r in 0..bounds.1 {
        for c in 0..bounds.0 {
            let point = pixel_to_point(bounds, (c, r), upper_left, lower_right);
            pixels[r * bounds.0 + c] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use std::io::{Error, Result, Write};

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    // Ensure that the size of the `pixels` slice matches the expected image dimensions
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    // Encode the image using the specified dimensions and color type
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;

    Ok(())
}
