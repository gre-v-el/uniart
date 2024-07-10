use image::Rgba;

use crate::Args;

fn color_to_code(color: Rgba<u8>) -> u8 {
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

pub fn set_color(col: Rgba<u8>, args: &Args) {
    if args.truecolor {
        let [r, g, b, _] = col.0;
        print!("\x1b[38;2;{r};{g};{b}m")
    } else {
        let code = color_to_code(col);
        print!("\x1b[38;5;{code}m")
    }
}

pub fn set_color_full_brightness(col: Rgba<u8>, args: &Args) {
    let (mut r, mut g, mut b) = (col[0] as f32/255.0, col[1] as f32/255.0, col[2] as f32/255.0);
    let max = r.max(g).max(b);
    r /= max;
    g /= max;
    b /= max;
    let (r, g, b) = ((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8);
    let col = Rgba::from([r, g, b, col.0[3]]);
    set_color(col, args)
}

pub fn set_color_bg(fg: Rgba<u8>, bg: Rgba<u8>, args: &Args) {
    if args.truecolor {
        let [rf, gf, bf, _] = fg.0;
        let [rb, gb, bb, _] = bg.0;

        print!("\x1b[48;2;{rb};{gb};{bb}m\x1b[38;2;{rf};{gf};{bf}m")
    }
    else {
        let fg_code = color_to_code(fg);
        let bg_code = color_to_code(bg);
        print!("\x1b[38;5;{fg_code}m\x1b[48;5;{bg_code}m")
    }
}