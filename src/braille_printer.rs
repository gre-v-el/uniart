use image::{imageops::{dither, BiLevel}, DynamicImage, GenericImageView};

use crate::{colors::{reset_color, set_black_background, set_color_full_brightness}, Args};

/// Dot weights for Braille characters in unicode. 
const WEIGHTS: [[u8; 2]; 4] = [
    [0x1,  0x8], 
    [0x2,  0x10],
    [0x4,  0x20], 
    [0x40, 0x80]
];

pub fn print_braille(args: &Args, image: &DynamicImage) {
    // one pixel per symbol
    let colors = image.resize_exact( 
        args.width, 
        args.height, 
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );

    // 2x4 pixels per symbol (braille grid)
    let image = image.resize_exact( 
        args.width*2, 
        args.height*4, 
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );
    let mut image = image.to_luma8();
    dither(&mut image, &BiLevel);

    for y in (0..image.height()).step_by(4) {
        set_black_background(args);
        for x in (0..image.width()).step_by(2) {
            let mut braille_code = 0;
            for dy in 0..4 {
                for dx in 0..2 {
                    let [luminance] = image.get_pixel(x+dx, y+dy).0;
                    let mut bit = (luminance > 128) as u8;
                    if args.invert { bit = 1 - bit; }
                    braille_code += bit * WEIGHTS[dy as usize][dx as usize];
                }
            }
            set_color_full_brightness(colors.get_pixel(x/2, y/4), args);
            print!("{}", std::char::from_u32(0x2800 + braille_code as u32).unwrap());
        }
        reset_color();
        println!();
    }
}