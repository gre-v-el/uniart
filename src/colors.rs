use image::Rgba;

use crate::Args;

/// Convert an RGBA color to a 256-color terminal code.
fn color_to_code(color: Rgba<u8>) -> u8 {
    let [r, g, b, a] = color.0;
    // multiply by alpha and map to 0-5 integer
    let r = (r as f32 * a as f32 / 255.0 / 255.0 * 5.0).round() as u8;
    let g = (g as f32 * a as f32 / 255.0 / 255.0 * 5.0).round() as u8;
    let b = (b as f32 * a as f32 / 255.0 / 255.0 * 5.0).round() as u8;
    
    16 + 36*r + 6*g + b
}

/// Reset the terminal color to default.
pub fn reset_color() {
    print!("\x1b[0m")
}

/// Set the terminal color to a given color according to the arguments.
pub fn set_color(col: Rgba<u8>, args: &Args) {
    if args.truecolor {
        let [r, g, b, _] = col.0;
        print!("\x1b[38;2;{r};{g};{b}m")
    } 
    else if args.colors {
        let code = color_to_code(col);
        print!("\x1b[38;5;{code}m")
    }
}

/// Set the terminal color to a given color according to the arguments, with full brightness. Doesn't work with invert.
pub fn set_color_full_brightness(col: Rgba<u8>, args: &Args) {
    if !args.colors && !args.truecolor { return; }
    if args.invert {
        // Brightness recompensation doesn't work for on light backgrounds.
        set_color(col, args);
        return;
    }
    
    // Scale the color as much as possible without clipping.
    let (mut r, mut g, mut b) = (col[0] as f32/255.0, col[1] as f32/255.0, col[2] as f32/255.0);
    let max = r.max(g).max(b);
    r /= max;
    g /= max;
    b /= max;
    let (r, g, b) = ((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8);
    let col = Rgba::from([r, g, b, col.0[3]]);
    set_color(col, args)
}

/// Set the terminal color to a given foreground and background color according to the arguments.
pub fn set_color_bg(fg: Rgba<u8>, bg: Rgba<u8>, args: &Args) {
    if !args.colors && !args.truecolor { return; }
    
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

/// Set the terminal background color to black if the background flag is set.
pub fn set_black_background(args: &Args) {
    if args.background {
        print!("\x1b[48;5;0m");
    }
}