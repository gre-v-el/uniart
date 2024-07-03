use image::{DynamicImage, GenericImageView, Rgba};

fn color_to_code(color: Rgba<u8>) -> u8 {
    let [r, g, b, a] = color.0;
    // multiply by alpha and map to 0-5 integer
    let r = (r as f32 * a as f32 / 255.0 / 255.0 * 5.0).round() as u8;
    let g = (g as f32 * a as f32 / 255.0 / 255.0 * 5.0).round() as u8;
    let b = (b as f32 * a as f32 / 255.0 / 255.0 * 5.0).round() as u8;
    
    16 + 36*r + 6*g + b
}

fn reset_color() {
    print!("\x1b[0m")
}

fn set_color(code: u8) {
    print!("\x1b[38;5;{code}m")
}

pub fn print_with_colors(lines: &Vec<String>, scaled: &DynamicImage) {
    let mut last_code = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let code = color_to_code(scaled.get_pixel(x as u32, y as u32));
            if code != last_code {
                set_color(code);
                last_code = code;
            }
            print!("{ch}")
        }
        reset_color();
        last_code = 0;
        println!()
    }
}