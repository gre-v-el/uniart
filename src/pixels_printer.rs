use image::{GenericImageView, ImageError};

use crate::{colored_printer::{color_to_code, reset_color, set_color, set_color_bg}, Args};

pub fn print_pixels(args: &Args) -> Result<(), ImageError> {
    let scaled = args.image_file.as_ref().unwrap().resize_exact(
        args.width,
        args.height,
        image::imageops::FilterType::Nearest
    );

    for y in 0..scaled.height() {
        for x in 0..scaled.width() {
            let pixel = scaled.get_pixel(x, y);

            let code = color_to_code(pixel);
            set_color(code);
            print!("█");
        }
        reset_color();
        println!()
    }

    Ok(())
}

pub fn print_double_pixels(args: &Args) -> Result<(), ImageError> {
    let scaled = args.image_file.as_ref().unwrap().resize_exact(
        args.width,
        args.height * 2, // each character is two pixels - top and bottom
        image::imageops::FilterType::Nearest
    );

    for y in 0..args.height {
        for x in 0..args.width {
            let upper_px = scaled.get_pixel(x, 2*y);
            let lower_px = scaled.get_pixel(x, 2*y+1);

            let bg_code = color_to_code(upper_px);
            let fg_code = color_to_code(lower_px);
            set_color_bg(fg_code, bg_code);
            print!("▄");
        }
        reset_color();
        println!()
    }

    Ok(())
}