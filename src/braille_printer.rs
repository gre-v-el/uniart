use image::{imageops::{dither, BiLevel}, GenericImageView, Rgba};

use crate::{colored_printer::{reset_color, set_color}, Args};

const WEIGHTS: [[u8; 2]; 4] = [
    [0x1,  0x8], 
    [0x2,  0x10], 
    [0x4,  0x20], 
    [0x40, 0x80]
];

pub fn print_braille(args: &Args) {
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
    let mut image = image.to_luma8();
    dither(&mut image, &BiLevel);

    for y in (0..image.height()).step_by(4) {
        for x in (0..image.width()).step_by(2) {
            let mut braille = 0;
            for dy in 0..4 {
                for dx in 0..2 {
                    let [luminance] = image.get_pixel(x+dx, y+dy).0;
                    let mut bit = (luminance > 128) as u8;
                    if args.invert { bit = 1 - bit; }
                    braille += bit * WEIGHTS[dy as usize][dx as usize];
                }
            }
            if args.colors {
                let mut col = colors.get_pixel(x/2, y/4);
                if !args.invert {
                    // make the color's brightness maximum, because dithering takes care of brightness
                    let (mut r, mut g, mut b) = (col[0] as f32/255.0, col[1] as f32/255.0, col[2] as f32/255.0);
                    let max = r.max(g).max(b);
                    r /= max;
                    g /= max;
                    b /= max;
                    let (r, g, b) = ((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8);
                    col = Rgba::from([r, g, b, col.0[3]]);
                }
                set_color(col, args);
            }
            print!("{}", std::char::from_u32(0x2800 + braille as u32).unwrap());
        }
        reset_color();
        println!();
    }
}