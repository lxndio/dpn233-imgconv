extern crate image;

use image::Pixels;
use image::Luma;
use image::ImageBuffer;
use image::FilterType;
use image::imageops::colorops::{BiLevel, dither};

fn main() {
    println!("Generating printer data...");

    let img_vec = conv_img();
    let printer_data = gen_printer_data(img_vec);

    for byte in printer_data {
        print!("{:?} ", byte);
    }
}

fn conv_img() -> [[bool; 144]; 144] {
    let mut img = image::open("test.png").unwrap();
    img = img.resize(144, 144, FilterType::Nearest);
    img = img.grayscale();

    let mut img_buf = img.as_mut_luma8().unwrap();
    dither(&mut img_buf, &BiLevel);

    img_buf.save("test2.png").unwrap(); // Save for debugging purposes

    let mut res = [[true; 144]; 144];
    for y in 0..img_buf.height() {
        for x in 0..img_buf.width() {
            res[y as usize][x as usize] = if img_buf.get_pixel(x, y).data[0] == 0 { false } else { true }
        }
    }
    
    res
}

fn gen_printer_data(img_vec: [[bool; 144]; 144]) -> Vec<u8> {
    let mut res = Vec::new();

    // Set line spacing to 0
    res.append(&mut vec![27, 49, 0]);

    // Set image data
    for y in (8..145).step_by(8) {
        // Begin new image printing command
        res.append(&mut vec![27, 75, 144, 0]);

        // Add image data
        for x in 0..144 {
            let mut byte: u8 = 0;
            for i in y-8..y-1 {
                if img_vec[i][x] { byte |= 0b0000_0001 }
                byte = byte.rotate_left(1);
            }
            if img_vec[y-1][x] { byte |= 0b0000_0001 }

            res.push(byte);
        }
    }

    res
}

