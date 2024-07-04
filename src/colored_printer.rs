use image::Rgba;

pub fn color_to_code(color: Rgba<u8>) -> u8 {
    let [r, g, b, a] = color.0;
    // multiply by alpha and map to 0-5 integer
    let r = (r as f32 * a as f32 / 255.0 / 255.0 * 5.0).round() as u8;
    let g = (g as f32 * a as f32 / 255.0 / 255.0 * 5.0).round() as u8;
    let b = (b as f32 * a as f32 / 255.0 / 255.0 * 5.0).round() as u8;
    
    16 + 36*r + 6*g + b
}

pub fn reset_color() {
    print!("\x1b[0m")
}

pub fn set_color(code: u8) {
    print!("\x1b[38;5;{code}m")
}

pub fn set_color_bg(fg_code: u8, bg_code: u8) {
    print!("\x1b[38;5;{fg_code}m\x1b[48;5;{bg_code}m")
}