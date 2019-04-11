extern crate image;

use serial::prelude::SerialPort;
use image::Pixels;
use image::Luma;
use image::ImageBuffer;
use image::FilterType;
use image::imageops::colorops::{BiLevel, dither};

fn main() {
    println!("Generating printer data...");

    let img_vec = conv_img();
    let printer_data = gen_printer_data(img_vec);

    for byte in &printer_data {
        print!("{:?} ", byte);
    }

    let mut port = serial::open("COM3").unwrap();
    send_serial(&printer_data, &mut port);
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
                if !img_vec[i][x] { byte |= 0b0000_0001 }
                byte = byte.rotate_left(1);
            }
            if !img_vec[y-1][x] { byte |= 0b0000_0001 }

            res.push(byte);
        }
    }

    res
}

fn send_serial<T: SerialPort>(data: &Vec<u8>, port: &mut T) {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    });

    port.write(&[27, 64]); // Reset the printer

    port.write(data);
}