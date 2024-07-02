use image::ImageError;

use crate::Args;

// https://stackoverflow.com/questions/30097953/ascii-art-sorting-an-array-of-ascii-characters-by-brightness-levels-c-c
const CHARS: &str = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const BRIGHTNESS: [f32; 92] = [0.0, 0.0751, 0.0829, 0.0848, 0.1227, 0.1403, 0.1559, 0.185, 0.2183, 0.2417, 0.2571, 0.2852, 0.2902, 0.2919, 0.3099, 0.3192, 0.3232, 0.3294, 0.3384, 0.3609, 0.3619, 0.3667, 0.3737, 0.3747, 0.3838, 0.3921, 0.396, 0.3984, 0.3993, 0.4075, 0.4091, 0.4101, 0.42, 0.423, 0.4247, 0.4274, 0.4293, 0.4328, 0.4382, 0.4385, 0.442, 0.4473, 0.4477, 0.4503, 0.4562, 0.458, 0.461, 0.4638, 0.4667, 0.4686, 0.4693, 0.4703, 0.4833, 0.4881, 0.4944, 0.4953, 0.4992, 0.5509, 0.5567, 0.5569, 0.5591, 0.5602, 0.5602, 0.565, 0.5776, 0.5777, 0.5818, 0.587, 0.5972, 0.5999, 0.6043, 0.6049, 0.6093, 0.6099, 0.6465, 0.6561, 0.6595, 0.6631, 0.6714, 0.6759, 0.6809, 0.6816, 0.6925, 0.7039, 0.7086, 0.7235, 0.7302, 0.7332, 0.7602, 0.7834, 0.8037, 0.9999];

fn char_from_luminance(luminance: f32) -> char {
    // binary search
    let mut low = 0;
    let mut high = BRIGHTNESS.len() - 1;
    while low < high {
        let mid = (low + high) / 2;
        if BRIGHTNESS[mid] < luminance {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    CHARS.chars().nth(low).unwrap()
}

pub fn convert_luminance(args: &Args) -> Result<(String, u32, u32), ImageError> {
    let image = image::open(&args.image)?;

    let (w, h) = (image.width() as f32, image.height() as f32);
    let image = image.resize_exact(
        args.width, 
        (h/w*args.width as f32 / args.font_aspect_ratio) as u32, 
        image::imageops::FilterType::Nearest
    );

    let luminance = image.to_luma8();

    let mut res = String::with_capacity((luminance.width() * luminance.height()) as usize);
    for y in 0..luminance.height() {
        for x in 0..luminance.width() {
            let pixel = luminance.get_pixel(x, y);
            let luminance = pixel[0] as f32 / 255.0;
            res.push(char_from_luminance(luminance));
        }
    }

    return Ok((res, luminance.width(), luminance.height()));
}