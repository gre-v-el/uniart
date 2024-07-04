use image::{DynamicImage, GenericImageView, ImageError};

use crate::{colored_printer::{color_to_code, reset_color, set_color}, Args};

// https://stackoverflow.com/questions/30097953/ascii-art-sorting-an-array-of-ascii-characters-by-brightness-levels-c-c
const DENSE_CHARS: &str = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const DENSE_BRIGHTNESS: [f32; 92] = [0.0, 0.0751, 0.0829, 0.0848, 0.1227, 0.1403, 0.1559, 0.185, 0.2183, 0.2417, 0.2571, 0.2852, 0.2902, 0.2919, 0.3099, 0.3192, 0.3232, 0.3294, 0.3384, 0.3609, 0.3619, 0.3667, 0.3737, 0.3747, 0.3838, 0.3921, 0.396, 0.3984, 0.3993, 0.4075, 0.4091, 0.4101, 0.42, 0.423, 0.4247, 0.4274, 0.4293, 0.4328, 0.4382, 0.4385, 0.442, 0.4473, 0.4477, 0.4503, 0.4562, 0.458, 0.461, 0.4638, 0.4667, 0.4686, 0.4693, 0.4703, 0.4833, 0.4881, 0.4944, 0.4953, 0.4992, 0.5509, 0.5567, 0.5569, 0.5591, 0.5602, 0.5602, 0.565, 0.5776, 0.5777, 0.5818, 0.587, 0.5972, 0.5999, 0.6043, 0.6049, 0.6093, 0.6099, 0.6465, 0.6561, 0.6595, 0.6631, 0.6714, 0.6759, 0.6809, 0.6816, 0.6925, 0.7039, 0.7086, 0.7235, 0.7302, 0.7332, 0.7602, 0.7834, 0.8037, 0.9999];

const SPARSE_CHARS: &str = " .-':,^=+T|IoxOXH80";
const SPARSE_BRIGHTNESS: [f32; 19] = [0.0, 0.117, 0.1197, 0.1731, 0.198, 0.2611, 0.308, 0.3411, 0.4119, 0.5287, 0.575, 0.6058, 0.6463, 0.6612, 0.8427, 0.9123, 0.9258, 0.9357, 0.9999];

fn char_from_luminance(luminance: f32, dense: bool) -> char {
    let chars = if dense {DENSE_CHARS} else {SPARSE_CHARS};
    let brightness: &[f32] = if dense {&DENSE_BRIGHTNESS} else {&SPARSE_BRIGHTNESS};

    // binary search
    let mut low = 0;
    let mut high = brightness.len() - 1;
    while low < high {
        let mid = (low + high) / 2;
        if brightness[mid] < luminance {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    chars.chars().nth(low).unwrap()
}

pub fn print_luminance(args: &Args, scaled: &DynamicImage) -> Result<(), ImageError> {
    for y in 0..scaled.height() {
        for x in 0..scaled.width() {
            let pixel = scaled.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;
            let mut luminance = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
            luminance *= a as f32 / 255.0;
            if args.invert {luminance = 1.0 - luminance}

            let char = char_from_luminance(luminance, args.dense);
            if args.colors {
                let code = color_to_code(pixel);
                set_color(code);
            }
            print!("{char}");
        }
        reset_color();
        println!()
    }

    Ok(())
}