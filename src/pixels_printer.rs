use image::{DynamicImage, GenericImageView, ImageError};

use crate::colored_printer::{color_to_code, reset_color, set_color};

pub fn print_pixels(scaled: &DynamicImage) -> Result<(), ImageError> {
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