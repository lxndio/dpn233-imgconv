extern crate image;

use image::FilterType;
use image::imageops::colorops::{BiLevel, dither};

fn main() {
    println!("Hello, world!");

    conv_img();
}

fn conv_img() {
    let mut img = image::open("test.png").unwrap();
    img = img.resize(100, 100, FilterType::Nearest);
    img = img.grayscale();

    let mut img_buf = img.as_mut_luma8().unwrap();
    dither(&mut img_buf, &BiLevel);

    img_buf.save("test2.png").unwrap();
}