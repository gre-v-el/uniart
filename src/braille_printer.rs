use image::{DynamicImage, GenericImageView, ImageBuffer, ImageError, Luma};

use crate::{colored_printer::{reset_color, set_color}, Args};

const SHIFTS: [[u8; 2]; 4] = [
    [0x1, 0x8], 
    [0x2, 0x10], 
    [0x4, 0x20], 
    [0x40, 0x80]
];

fn get_median_luminance(data: &ImageBuffer<Luma<u8>, Vec<u8>>) -> u8 {
    let mut luminances = Vec::new();
    for pixel in data.pixels() {
        let [luminance] = pixel.0;
        luminances.push(luminance);
    }
    luminances.sort();
    let mid = luminances.len() / 2;
    luminances[mid]
}

pub fn print_braille(args: &Args) -> Result<(), ImageError> {
    let image = args.image_file.as_ref().unwrap();
    let colors = image.resize_exact( // one pixel per symbol
        args.width, 
        args.height, 
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );
    let image = image.resize_exact( // 2x4 pixels per symbol (braille)
        args.width*2, 
        args.height*4, 
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );
    let image = image.to_luma8();

    let threshold = get_median_luminance(&image);

    for y in (0..image.height()).step_by(4) {
        for x in (0..image.width()).step_by(2) {
            let mut braille = 0;
            for dy in 0..4 {
                for dx in 0..2 {
                    let [luminance] = image.get_pixel(x+dx, y+dy).0;
                    let bit = (luminance > threshold) as u8;
                    braille += bit * SHIFTS[dy as usize][dx as usize];
                }
            }
            if args.colors {
                set_color(colors.get_pixel(x/2, y/4), args);
            }
            print!("{}", std::char::from_u32(0x2800 + braille as u32).unwrap());
        }
        reset_color();
        println!();
    }



    Ok(())
}