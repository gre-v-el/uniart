use image::{GenericImageView, ImageError};

use crate::{colored_printer::{reset_color, set_color, set_color_bg}, Args};

pub fn print_pixels(args: &Args) -> Result<(), ImageError> {
    let scaled = args.image_file.as_ref().unwrap().resize_exact(
        args.width,
        args.height,
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );

    for y in 0..scaled.height() {
        for x in 0..scaled.width() {
            let pixel = scaled.get_pixel(x, y);

            set_color(pixel, args);
            print!("█");
        }
        reset_color();
        println!()
    }

    Ok(())
}

pub fn print_double_pixels(args: &Args) {
    let scaled = args.image_file.as_ref().unwrap().resize_exact(
        args.width,
        args.height * 2, // each character is two pixels - top and bottom
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );

    for y in 0..args.height {
        for x in 0..args.width {
            let upper_px = scaled.get_pixel(x, 2*y);
            let lower_px = scaled.get_pixel(x, 2*y+1);

            set_color_bg(lower_px, upper_px, args);
            print!("▄");
        }
        reset_color();
        println!()
    }
}